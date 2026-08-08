#include <stdlib.h>
#include <stdio.h>
#include <cuda.h>

#define cudaCheckError(call) \
    do { \
        cudaError_t cuErr = call; \
        if(cuErr != cudaSuccess) { \
            fprintf(stderr, "Cuda Error: %s:%d: %s\n", __FILE__, __LINE__, cudaGetErrorString(cuErr)); \
            exit(EXIT_FAILURE); \
        } \
    } while(0)


__global__ void vector_add(double* A, double* B, double* C, size_t N) {
    long int i = blockDim.x * blockIdx.x + threadIdx.x;
    if(i < N) {
        C[i] = A[i] + B[i]; 
    }
}

int main(int argc, char** argv) {
    const unsigned long int N = 1 << 20;
    size_t nbytes = N * sizeof(double);

    int ndevices;
    cudaCheckError(cudaGetDeviceCount(&ndevices));
    printf("Number of devices: %d\n", ndevices);

    double* A = (double*)malloc(nbytes);
    double* B = (double*)malloc(nbytes);
    double* C = (double*)malloc(nbytes);
    for(size_t i = 0; i < N; i++) {
        A[i] = 1.0;
        B[i] = 2.0;
    }

    double* d_A;
    double* d_B;
    double* d_C;
    cudaCheckError(cudaMalloc((void**)&d_A, nbytes));
    cudaCheckError(cudaMalloc((void**)&d_B, nbytes));
    cudaCheckError(cudaMalloc((void**)&d_C, nbytes));
    cudaCheckError(cudaMemcpy(d_A, A, nbytes, cudaMemcpyHostToDevice));
    cudaCheckError(cudaMemcpy(d_B, B, nbytes, cudaMemcpyHostToDevice));

    int threads_per_block = 256;
    int blocks_per_grid = ceil((double)N / threads_per_block);

    vector_add<<<blocks_per_grid, threads_per_block>>>(d_A, d_B, d_C, N);
    cudaCheckError(cudaMemcpy(C, d_C, nbytes, cudaMemcpyDeviceToHost));

    const double err_tolerance = 1e-14;
    for(size_t i = 0; i < N; i++) {
        if(fabs(C[i] - 3.0) > err_tolerance) {
            fprintf(stderr, "Error: Expected C[%li]=%2.1lf, got: C[%li]=%2.9lf\n", i, 3.0, i, C[i]);
        }
    }

    cudaFree(d_A);
    cudaFree(d_B);
    cudaFree(d_C);
    free(A);
    free(B);
    free(C);

    return 0;
}
