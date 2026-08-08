#include <stdlib.h>
#include <stdio.h>
#include <cuda.h>

#include "common.h"

__global__ void kernel_sqrt(double* input_arr, double* output_arr, size_t len) {
    unsigned int tid = blockIdx.x * blockDim.x + threadIdx.x;
    if(tid < len) {
        output_arr[tid] = sqrt(input_arr[tid]);
    }
}

void get_sqrt(double* arr, size_t len) {
    size_t nbytes = len * sizeof(double);
    double* d_arr = NULL;
    double* d_ret = NULL;
    CUDAErrorCheck(cudaMalloc((void**)&d_arr, nbytes));
    CUDAErrorCheck(cudaMalloc((void**)&d_ret, nbytes));
    CUDAErrorCheck(cudaMemcpy(d_arr, arr, nbytes, cudaMemcpyHostToDevice));
    kernel_sqrt<<<GPU_BLOCKS_PER_GRID, GPU_THREADS_PER_BLOCK>>>(d_arr, d_ret, len);
    CUDAErrorCheck(cudaMemcpy(arr, d_ret, nbytes, cudaMemcpyDeviceToHost)); // inplace sqrt of array
    CUDAErrorCheck(cudaFree(d_arr));
    CUDAErrorCheck(cudaFree(d_ret));
} 
