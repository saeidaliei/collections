## Vector Addition

Add two vectors.

For compilation with `nvcc`, issue: `make` for the default target.

```text
$ make help
Available targets:
  vec.x       make default executable (default)
  debug       make debug executable
  registers   make register usage info version
  multiarch   make multi-architecture version
  clean       remove executable and object files
  help        print this help message
```

### Thread Organization

1. Each thread has a unique 3D accessible via:

- thread index: `threadIdx.x`, `threadIdx.y`, and `threadIdx.z`,
- block index:  `blockIdx.x`,  `blockIdx.y`, and  `blockIdx.z`,
- block size:   `blockDim.x`,  `blockDim.y`, and  `blockDim.z`,
- grid size:    `gridDim.x`,   `gridDim.y`, and   `gridDim.z`

2. Warp size (32 threads):

- A warp is the smallest execution unit in CUDA.
- All threads in a warp execute the same instruction (SIMT).
- If threads in a warp take different paths (diverge), the execution is serialized.

Example of warp divergence:

```cuda
__global__ void divergent_threads(int* data) {
    int tid = threadIdx.x;
    if (tid%2 == 0) {
        data[tid] *= 2;  // even threads
    }
    else {
        data[tid] += 1;  // odd threads
    }
}
```

3. Thread ID calucaltion:

```cuda
// 1D
int tid = blockIdx.x * blockDim.x + threadIdx.x;
// 2D
int tid = (blockIdx.y * gridDim.x + blockIdx.x) * (blockDim.x * blockDim.y) +
          (threadIdx.y * blockDim.x + threadIdx.x);
// 3D
int tid = (blockIdx.z * gridDim.y * gridDim.x + blockIdx.y * gridDim.x + blockIdx.x) *
          (blockDim.x * blockDim.y * blockDim.z) +
          (threadIdx.z * blockDim.y * blockDim.x + threadIdx.y * blockDim.x + threadIdx.x);
```

Example matrix multiplication:

```cuda
__global__ void matrix_mul(double* A, double* B, double* C, int width) {
    // 2D thread ids
    int row = blockIdx.y * blockDim.y + threadIdx.y;
    int col = blockIdx.x * blockDim.x + threadIdx.x;

    if (row < width && col < width) {
        double sum = 0.0;
        for (int i = 0; i < width; i++) {
            sum += A[row * width + i] * B[i * width + col];
        }
        C[row * width + col] = sum;
    }
}

// kernel launch
dim3 block_size(16, 16);  // 256 threads per block
dim3 num_blocks((width + block_size.x - 1) / block_size.x, (width + block_size.y - 1) / block_size.y);
matrix_mul<<<num_blocks, block_size>>>(d_A, d_B, d_C, width);
```

4. Important limitations:

- Maximum threads per block: 1024 (varies by GPU)
- Maximum block dimensions: 1024 x 1024 x 64
- Maximum grid dimensions: 2**31-1 x 65535 x 65535

5. Memory coalescing:

- Threads in a warp should access consecutive memory addresses for best performance:

```cuda
// coalesced access
int tid = blockIdx.x * blockDim.x + threadIdx.x;
double value = data[tid];

// strided access
int tid = threadIdx.x * stride;
double value = data[tid];
```

6. Block size considerations:

- Should be multiple of warp size: 32
- Common block size: 128, 256, 512
- Example:

```cuda
dim3 block(256, 1, 1);    // 1D: 256 threads
dim3 block(16, 16, 1);    // 2D: 256 threads
dim3 block(8, 8, 4);      // 3D: 256 threads

int num_blocks = (N + block.x - 1) / block.x;
kernel<<<num_blocks, block>>>(data);
```

### NVCC Flags

These are some of the main `nvcc` flags.

1. Architecture related:

- `-arch=sm_XX`: Specify GPU architecture (compute capability). Example: 
- `-arch=sm_75`: for Turing.
- `-arch=sm_80`: for Ampere.
- `-arch=sm_90`: for Hopper.
- `-code=sm_XX`: Specify actual GPU code to generate. Example:
- `-gencode arch=compute_XX,code=sm_XX`: Generate code for multiple architectures.

2. Optimization flags:

- `-O0`: No optimization (default).
- `-O1`, `-O2`, `-O3`: Different optimization levels.
- `-use_fast_math`: Use faster but less precise math functions.
- `-ftz=true/false`: Flush denormal numbers to zero.
- `-prec-div=true/false`: Precise/imprecise division.
- `-prec-sqrt=true/false`: Precise/imprecise square root.

3. Debugging & profiling:

- `-g`: Generate debug information.
- `-G`: Generate debug information for device code.
- `-lineinfo`: Include line number information.
- `-Xptxas -v`: Show register usage and compilation statistics.
- `-pgpu`: Enable GPU profiling.

4. Warning & error control:

- `-Wall`: Enable all warnings.
- `-Werror`: Treat warnings as errors.
- `-Wno-deprecated-declarations`: Disable deprecated function warnings.
- `-w`: Disable all warning messages.

5. Compilation control:

- `-c`: Compile only, don't link.
- `-dc`: Generate device code only.
- `-dlink`: Device link only.
- `-rdc=true`: Enable relocatable device code.
- `-M`: Generate dependency file.
- `-std=c++17`: Specify C++ standard (11,14,17,etc).

6. Include & library paths:

- `-I`: Add include directory.
- `-L`: Add library directory.
- `-l`: Link with library.
