## Reduction

Summing up an array.

- [Version 1](#version-1)
- [Version 2](#version-2)
- [Version 3](#version-3)
- [Version 4](#version-4)
- [Version 5](#version-5)
- [Version 6](#version-6)
- [Version 7](#version-7)

### Version 1

- Has thread divergence due to expensive modulo operation.
- Only half of threads in a block are active in each reduction iteration.
- Bank conflicts occure in shared memory, since:
    - Shared memory is divided into 32 banks.
    - Consecutive 32-bit words are assigned to consecutive banks.
    - When multiple threads access the same bank simultaneously, the accesses are serialized. 
        - For example: when s = 4 we would have:
            - Thread 0 accesses: sdata[0] and sdata[4]
            - Thread 4 accesses: sdata[4] and sdata[8]
            - Thread 8 accesses: sdata[8] and sdata[12]
        - sdata[4] is accessed by both thread 0 and 4 causing a bank conflict.
        This happens because the stride eventually become larger than the warp size, 
        leading to multiple threads accessing the same bank.
- Non-Coalesced memory access:
    - The initial load from global memory is coalesced because:
        - `sdata[tid] = (i < N) ? input[i] : 0;`
        - Consecutive threads access consecutive memory locations.
        - Thread 0 reads input[0], Thread 1 reads input[1], etc.
    - In reduction loop, as the stride increases, the memory access pattern becomes non-coalesced.
        - When s = 16:
        - Thread 00 reads from: tid + 16
        - Thread 16 reads from: tid + 16
        - Thread 32 reads from: tid + 16
    - This creates a strided access pattern where threads are reading from memory locations 
    far apart from each other, preventing coalescing.

```cpp
__shared__  double sdata[BLOCK_SIZE];

int tid = threadIdx.x;
// index of global input array elements
int idx = blockIdx.x * blockDim.x + threadIdx.x;

// load global array into shared memory array
sdata[tid] = (idx < N) ? idata[idx] : 0;
__syncthreads();

/*
 * e.g. idata: [1, ..., 1] (16 elements), and block size is 8, then:
 * sdata[1, 1, 1, 1, 1, 1, 1, 1] for one block, then for 
 * s = 1 loop (tid % 2 == 0):
 * active threads: 0, 2, 4, 6
 * thread 0: sdata[0] += sdata[1]
 * thread 2: sdata[2] += sdata[3]
 * thread 4: sdata[4] += sdata[5]
 * thread 6: sdata[6] += sdata[7]
 * result: sdata[2, 1, 2, 1, 2, 1, 2, 1]
 * s = 2 loop (tid % 4 == 0):
 * active threads: 0, 4
 * thread 0: sdata[0] += sdata[2]
 * thread 4: sdata[4] += sdata[6]
 * result: sdata[4, 1, 2, 1, 4, 1, 2, 1]
 * s = 3 loop (tid % 8 == 0):
 * active threads: 0
 * thread 0: sdata[0] += sdata[4]
 * result: sdata[8, 1, 2, 1, 4, 1, 2, 1]
 */
for (uint16_t s = 1; s < blockDim.x; s *= 2) {
    // thread divergence, expensive modulo operation, 
    // only half threads are active in each iteration,
    // bank conflicts, non-coalesced access
    if (tid % (2*s) == 0) {
        sdata[tid] += sdata[tid + s];
    }
    __syncthreads();
} 

if (tid == 0) {
    odata[blockIdx.x] = sdata[0];
}

```

---
### Version 2

- Replaces divergent branch, `if (tid % (2*s) == 0)`, with strided index and non-divergent branch:
```cpp
for (uint32_t s = 1; s < blockDim.x; s *= 2) {
    // strided index and non-divergent branch,
    // but with bank conflicts
    int index = 2 * s * tid;
    if (index < blockDim.x) {
        sdata[tid] += sdata[tid + index];
    }
    __syncthreads();
}
```
- High enought strides, `s`, will still cause bank conflicts.

---
### Version 3

- Removes bank coflicts by starting at the middle of the block:
```cpp
for (uint32_t s = blockDim.x / 2; s > 0; s >>= 1) {
    if (tid < s) {
        // no bank conflicts but half of threads
        // are idle in the first iteration
        sdata[tid] += sdata[tid + s];
    }
    __syncthreads();
}
```
- Note the syntax, `s>>= 1`, which is `s = s >> 1`, or right shifting the stride, which 
halves it at each iteration.

- Half of threads are idle in the first iteration.

---
### Version 4

- Each thread loads two elements and sums the other half of the block:
```cpp
int idx = blockIdx.x * blockDim.x * 2 + threadIdx.x;

/*
 * each thread loads two elements from global input array,
 * e.g. if idata: [1, 2, 3, 4, 5, 6, 7, 8], and if blockDim.x = 4, 
 * thread 0: sdata[0] = idata[0] + idata[4] = 6
 * thread 1: sdata[1] = idata[1] + idata[5] = 8
 * and so on for threads 2 and 3 in blockIdx.x = 0
 * so sdata = [6, 8, 10, 12]
 */
sdata[tid] = idata[idx] + idata[idx + blockDim.x];

// still half threads are used in a block, but when loading 
// all threads are used and they do a first reduction
for (uint32_t s = blockDim.x / 2; s > 0; s >>= 1) {
    if (tid < s) {
        /*
         * s = 2, tid = 0:
         * sdata[0] += sdata[2] -> sdata[0] = 16
         * s = 2, tid = 1:
         * sdata[1] += sdata[3] -> sdata[1] = 20
         * after first iteration: sdata = [16, 20, 10, 12]
         * s = 1, tid = 0:
         * sdata[0] += sdata[1] -> sdata[0] = 36
         */
        sdata[tid] += sdata[tid + s];
    }
    __syncthreads();
}
```

- In each iteration of the reduction loop, the number of active threads is halved again.

---
### Version 5

- Threads inside a warp do an unrolled reduction in a separate device function, which 
avoids expensive unnecessary synchronization, loop index calculation and condition checking.
- Threads in the same warp execute together (SIMD), so they don't need explicit synchronisation.
- Reduces Loop Overhead.
```cpp
__device__ void warp_reduce_v1(volatile double* sdata, int tid) {
    sdata[tid] += sdata[tid + 32];
    sdata[tid] += sdata[tid + 16];
    sdata[tid] += sdata[tid + 8];
    sdata[tid] += sdata[tid + 4];
    sdata[tid] += sdata[tid + 2];
    sdata[tid] += sdata[tid + 1];
}

for (uint32_t s = blockDim.x / 2; s > 32; s >>= 1) {
    // still could lead to warp divergence
    if (tid < s) {
        sdata[tid] += sdata[tid + s];
        __syncthreads();
    }
}

if (tid < 32) { warp_reduce_v1(sdata, tid); } 
```

- This saves useless work in all warps, not just the last one.
Without unrolling, all warps execute every iteration of the for loop and if statement.

---
### Version 6

- If we knew the number of iterations at compile time, we could completely unroll the reduction.
- The block size is limited by the GPU to 512 threads, Also, we are sticking to power-of-2 block sizes.
- We can easily unroll for a fixed block size.
- All the if statements will be evaluated at compile time:
```cpp
template <uint32_t block_size>
__device__ void warp_reduce_v2(volatile double* data, int tid) {
    // if checks are evaluated at compile time.
    if (block_size >= 64) sdata[tid] += sdata[tid + 32];
    if (block_size >= 32) sdata[tid] += sdata[tid + 16];
    if (block_size >= 16) sdata[tid] += sdata[tid +  8];
    if (block_size >=  8) sdata[tid] += sdata[tid +  4];
    if (block_size >=  4) sdata[tid] += sdata[tid +  2];
    if (block_size >=  2) sdata[tid] += sdata[tid +  1];
}

template <uint32_t block_size>
__global__ void reduction_v6(double* idata, double* odata) {
    // extern means the size is not known at compile, 
    // this is a dynamically allocated shared memory
    extern __shared__ double sdata[];

    // completely unrolled reduction, checks are evaluated at compile time.
    if (blockSize >= 512) { if (tid < 256) { sdata[tid] += sdata[tid + 256]; } __syncthreads(); }
    if (blockSize >= 256) { if (tid < 128) { sdata[tid] += sdata[tid + 128]; } __syncthreads(); }
    if (blockSize >= 128) { if (tid <  64) { sdata[tid] += sdata[tid +  64]; } __syncthreads(); }

    if (tid < 32) { warp_reduce_v2<block_size>(sdata, tid); }
}

```

- No loop overhead, minimised synchronisation.
- The compiler can eliminate unnecessary code when blockSize is small.

---
### Version 7

- Replace load and add of two elements:
```cpp
int tid = threadIdx.x;
int idx = blockIdx.x * (blockDim.x*2) + threadIdx.x;

sdata[tid] = idata[idx] + idata[idx + blockDim.x];
__syncthreads();
```
with a while loop to add as many as necessary:
```cpp
int tid = threadIdx.x;
int idx = blockIdx.x * (block_size * 2) + threadIdx.x;
int grid_size = block_size * 2 * gridDim.x;

double sum = 0.0;
while (idx < n) {
    sdata[tid] += idata[idx] + idata[idx + block_size];
    // grid_size loop striding to maintain coalescing
    idx += grid_size;
}
sdata[tid] = sum;
__syncthreads();

```

- Most performant version.

---
- **Note**: This presentation was the main source for this project: [Optimizing Parallel Reduction in CUDA](https://developer.download.nvidia.com/assets/cuda/files/reduction.pdf).
