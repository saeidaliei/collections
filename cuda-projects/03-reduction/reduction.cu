#include <cmath>
#include <cstdint>
#include <cstdio>
#include <stdio.h>
#include <stdlib.h>
#include <cuda_runtime.h>

#define N (1 << 20)
#define BLOCK_SIZE 256

__global__ void reduction_v1(double* idata, double* odata) {
    // each thread block allocates its own sdata in shared memory
    __shared__  double sdata[BLOCK_SIZE];

    int tid = threadIdx.x;
    // index of global input array elements
    int idx = blockIdx.x * blockDim.x + threadIdx.x;

    // load global array into shared memory array
    sdata[tid] = (idx < N) ? idata[idx] : 0;
    __syncthreads();

    // e.g. idata: [1, ..., 1] (16 elements), and block size is 8, then:
    // sdata[1, 1, 1, 1, 1, 1, 1, 1] for one block, then for 
    // s = 1 loop (tid % 2 == 0):
    // active threads: 0, 2, 4, 6
    // thread 0: sdata[0] += sdata[1]
    // thread 2: sdata[2] += sdata[3]
    // thread 4: sdata[4] += sdata[5]
    // thread 6: sdata[6] += sdata[7]
    // result: sdata[2, 1, 2, 1, 2, 1, 2, 1]
    // s = 2 loop (tid % 4 == 0):
    // active threads: 0, 4
    // thread 0: sdata[0] += sdata[2]
    // thread 4: sdata[4] += sdata[6]
    // result: sdata[4, 1, 2, 1, 4, 1, 2, 1]
    // s = 3 loop (tid % 8 == 0):
    // active threads: 0
    // thread 0: sdata[0] += sdata[4]
    // result: sdata[8, 1, 2, 1, 4, 1, 2, 1]
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
}

__global__ void reduction_v2(double* idata, double* odata) {
    __shared__ double sdata[BLOCK_SIZE];

    int tid = threadIdx.x;
    int idx = blockIdx.x * blockDim.x + threadIdx.x;

    sdata[tid] = (idx < N) ? idata[idx] : 0;
    __syncthreads();

    for (uint32_t s = 1; s < blockDim.x; s *= 2) {
        // strided index and non-divergent branch,
        // but with bank conflicts
        int index = 2 * s * tid;
        if (index < blockDim.x) {
            sdata[tid] += sdata[tid + index];
        }
        __syncthreads();
    }

    if (tid == 0) {
        odata[blockIdx.x] = sdata[0];
    }
}

__global__ void reduction_v3(double* idata, double* odata) {
    __shared__ double sdata[BLOCK_SIZE];

    int tid = threadIdx.x;
    int idx = blockIdx.x * blockDim.x + threadIdx.x;

    sdata[tid] = (idx < N) ? idata[idx] : 0;
    __syncthreads();

    // suppose the block is: [1, 2, 3, 4, 5, 6, 7, 8],
    // s = 4: [1+5, 2+6, 3+7, 4+8] -> [6, 8, 10, 12]
    // s = 2: [6+8, 10+12] -> [14, 22]
    // s = 1: [14+22] -> [36]
    // s = 0: terminate
    for (uint32_t s = blockDim.x / 2; s > 0; s >>= 1) {
        if (tid < s) {
            // no bank conflicts but half of threads
            // are idle in the first iteration
            sdata[tid] += sdata[tid + s];
        }
        __syncthreads();
    }

    if (tid == 0) {
        odata[blockIdx.x] = sdata[0];
    }
}

__global__ void reduction_v4(double* idata, double* odata) {
    __shared__ double sdata[BLOCK_SIZE];

    int tid = threadIdx.x;
    int idx = blockIdx.x * blockDim.x * 2 + threadIdx.x;
    
    // each thread loads two elements from global input array,
    // e.g. if idata: [1, 2, 3, 4, 5, 6, 7, 8], and if blockDim.x = 4, 
    // thread 0: sdata[0] = idata[0] + idata[4] = 6
    // thread 1: sdata[1] = idata[1] + idata[5] = 8
    // and so on for threads 2 and 3 in blockIdx.x = 0
    // so sdata = [6, 8, 10, 12]
    sdata[tid] = idata[idx] + idata[idx + blockDim.x];

    // still half threads are used in a block, but when loading 
    // all threads are used and they do a first reduction
    for (uint32_t s = blockDim.x / 2; s > 0; s >>= 1) {
        if (tid < s) {
            // s = 2, tid = 0:
            // sdata[0] += sdata[2] -> sdata[0] = 16
            // s = 2, tid = 1:
            // sdata[1] += sdata[3] -> sdata[1] = 20
            // after first iteration: sdata = [16, 20, 10, 12]
            // s = 1, tid = 0:
            // sdata[0] += sdata[1] -> sdata[0] = 36
            sdata[tid] += sdata[tid + s];
        }
        __syncthreads();
    }

    if (tid == 0) {
        // odata[0] = 36, and so on for blockIdx.x = 1
        odata[blockIdx.x] = sdata[0];
    }

}

// this assumes there are at least 32 threads per block, for 
// smaller blocks tid+32, etc will go out of bound, which is
// safe in shared memory, although it is undefined behavior.
// volatile keyword is also needed, memory writes in shared 
// memory maybe reordered due to compiler optimization which 
// may cache values in registers instead of writing them back 
// to shared memory, the gpu memory subsystem although may 
// reorder reads and writes. If sdata is cached in registers 
// updates made by one thread might not be visible to another
// threads in the same warp. volatile makes sure all writes 
// to sdata are immediately visible to other threads and each 
// read fetches the latest value directly from the shared memory.
//
// __device__ identifier tells that this function is meant to 
// be used only in the device and can only be called from device 
// kernels or functions and not from host side.
__device__ void warp_reduce_v1(volatile double* sdata, int tid) {
    sdata[tid] += sdata[tid + 32];
    sdata[tid] += sdata[tid + 16];
    sdata[tid] += sdata[tid + 8];
    sdata[tid] += sdata[tid + 4];
    sdata[tid] += sdata[tid + 2];
    sdata[tid] += sdata[tid + 1];
}

__global__ void reduction_v5(double* idata, double* odata) {
    __shared__ double sdata[BLOCK_SIZE];

    int tid = threadIdx.x;
    int idx = blockIdx.x * blockDim.x * 2 + threadIdx.x;

    sdata[tid] = idata[idx] + idata[idx + blockDim.x];

    // still loop condition checking, branching, and index calculation
    for (uint32_t s = blockDim.x / 2; s > 32; s >>= 1) {
        // still could lead to warp divergence
        if (tid < s) {
            sdata[tid] += sdata[tid + s];
            __syncthreads();
        }
    }

    // all threads (32) in a warp execute in lockstep (SIMD), 
    // we don't need explicit expensive synchronization when threads < 32,
    // e.g. if idata[1, 2, 3, 4, 5, 6, 7, 8] and blockDim.x = 4, all threads 
    // will immediately jump to warp_reduce
    if (tid < 32) {
        warp_reduce_v1(sdata, tid);
    } 

    if (tid == 0) {
        odata[blockIdx.x] = sdata[0];
    }
}

// remove the out of bound accesses, all reduction steps are 
// explicitely written, no index calculation or loop checking, 
// no unnecessary synchronization at warp level.
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

    int tid = threadIdx.x;
    int idx = blockIdx.x * (block_size * 2) + threadIdx.x;

    // add second half during load (v4),
    // e.g. if N = 1024, and blockDim.x = 512,
    // sdata[tid] = idata[idx] + idata[idx + 512],
    // sdata = [(0+512), (1+513), ..., (511+1023)],
    // so size of sdata is now 512 instead of 1024
    sdata[tid] = idata[idx] + idata[idx + block_size];
    __syncthreads();

    // completely unroll the loop, no index calculation, etc.
    // these if checks are evaluated at compile time.
    if (block_size >= 512) {
        if (tid < 256) {
            // sdata[0] += sdata[256],
            // sdata[1] += sdata[257], ...,
            // sdata[255] += sdata[511]
            // so sdata size is now 256
            sdata[tid] += sdata[tid + 256];
        }
        __syncthreads();
    }
    if (block_size >= 256) {
        if (tid < 128) {
            // sdata[0]  += sdata[64],
            // sdata[1]  += sdata[65], ...,
            // sdata[63] += sdata[127]
            // so sdata size is now 128
            sdata[tid] += sdata[tid + 128];
        }
        __syncthreads();
    }
    if (block_size >= 128) {
        if (tid < 64) {
            // sdata[0]  += sdata[64],
            // sdata[1]  += sdata[65], ...,
            // sdata[63] += sdata[127]
            // so sdata size is now 64
            sdata[tid] += sdata[tid + 64];
        }
        __syncthreads();
    }

    if (tid < 32) {
        // threads with tid < 32 do the warp level reduction
        warp_reduce_v2<block_size>(sdata, tid);
    }

    if (tid == 0) {
        odata[blockIdx.x] = sdata[0];
    }
}

template <uint32_t block_size>
__global__ void reduction_v7(double* idata, double* odata, uint32_t n) {
    extern __shared__ double sdata[];

    int tid = threadIdx.x;
    int idx = blockIdx.x * (block_size * 2) + threadIdx.x;
    int grid_size = block_size * 2 * gridDim.x;

    double sum = 0.0;
    while (idx < n) {
        sdata[tid] += idata[idx] + idata[idx + block_size];
        idx += grid_size;
    }
    sdata[tid] = sum;
    __syncthreads();

    if (block_size >= 512) {
        if (tid < 256) {
            sdata[tid] += sdata[tid + 256];
        }
        __syncthreads();
    }
    if (block_size >= 256) {
        if (tid < 128) {
            sdata[tid] += sdata[tid + 128];
        }
        __syncthreads();
    }
    if (block_size >= 128) {
        if (tid < 64) {
            sdata[tid] += sdata[tid + 64];
        }
        __syncthreads();
    }

    if (tid < 32) {
        warp_reduce_v2<block_size>(sdata, tid);
    }

    if (tid == 0) {
        odata[blockIdx.x] = sdata[0];
    }
 
}



int main() {
    double* h_idata = (double*)malloc(N * sizeof(double));
    uint32_t num_blocks = (N + BLOCK_SIZE - 1) / BLOCK_SIZE;
    double* h_odata = (double*)malloc(num_blocks * sizeof(double));

    for (uint32_t i = 0; i < N; i++) {
        h_idata[i] = 1.0;
    }
    for (uint32_t i = 0; i < num_blocks; i++) {
        h_odata[i] = 0.0;
    }

    double* d_idata;
    double* d_odata;
    cudaMalloc((void**)&d_idata, N * sizeof(double));
    cudaMalloc((void**)&d_odata, num_blocks * sizeof(double));

    cudaMemcpy(d_idata, h_idata, N * sizeof(double), cudaMemcpyHostToDevice);

    dim3 dim_block(N);
    dim3 dim_grid(num_blocks);

    cudaEvent_t start, stop;
    cudaEventCreate(&start);
    cudaEventCreate(&stop);

    cudaEventRecord(start);
    reduction_v1<<<dim_grid, dim_block>>>(d_idata, d_odata);
    cudaEventRecord(stop);
    cudaEventSynchronize(stop);

    double ms_v1 = 0.0;
    cudaEventElapsedTime(&ms_v1, start, stop);

    cudaMemcpy(d_odata, h_odata, num_blocks * sizeof(double), cudaMemcpyDeviceToHost);
    double d_sum = 0.0;
    for (uint32_t i = 0; i < num_blocks; i++) {
        d_sum += h_odata[i];
    }

    if (fabs(static_cast<double>(d_sum - N)) > 1e-5) {
        fprintf(stderr, "Error: Test failed (v1).");
        fprintf(stderr, "Device sum (v1): %f\n", d_sum);
        fprintf(stderr, "Host sum (v1): %f\n", static_cast<double>(N));
    }
    else {
        fprintf(stdout, "Test passed (v1).\n");
        fprintf(stdout, "Elapsed time (v1): %f ms\n", ms_v1);
    }

}
