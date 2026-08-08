#include <cmath>
#include <cstdio>
#include <stdio.h>
#include <stdlib.h>

#define N 1000

template<typename T>
__global__ void vector_add(T* a, T* b, T* c) {
    size_t tid = blockIdx.x * blockDim.x + threadIdx.x;
    // each thread checks for this, 
    // to avoid beyond memory bound access.
    if (tid < N) {
        c[tid] = a[tid] + b[tid];
    }
}

int main() {
    double* h_a = (double*)malloc(N * sizeof(double));
    double* h_b = (double*)malloc(N * sizeof(double));
    double* h_c = (double*)malloc(N * sizeof(double));
    for(int i = 0; i < N; i++) {
        h_a[i] = 1.0;
        h_b[i] = 2.0;
        h_c[i] = 0.0;
    }

    double* d_a;
    double* d_b;
    double* d_c;
    // returns error code, takes pointer to an address
    cudaMalloc((void**)&d_a, N * sizeof(double));
    cudaMalloc((void**)&d_b, N * sizeof(double));
    cudaMalloc((void**)&d_c, N * sizeof(double));

    // target, source
    cudaMemcpy(d_a, h_a, N * sizeof(double), cudaMemcpyHostToDevice);
    cudaMemcpy(d_b, h_b, N * sizeof(double), cudaMemcpyHostToDevice);

    int block_size = 256;
    // ceiling division: N = n * block_size + left_over
    int num_blocks = (N + block_size - 1) / block_size;
    vector_add<double><<<num_blocks, block_size>>>(d_a, d_b, d_c);

    // target, source
    cudaMemcpy(h_c, d_c, N * sizeof(double), cudaMemcpyDeviceToHost);

    bool sucess = true;
    for (int i = 0; i < N; i++) {
        if (fabs(static_cast<double>(h_c[i] - (h_a[i] + h_b[i]))) > 1e-5) {
            fprintf(stderr, "Error in vector addition at index: %d\n", i);
            sucess = false;
            break;
        }
    }
    if (sucess) {
        fprintf(stdout, "Vector addition test passed.\n");
    }

    free(h_a);
    free(h_b);
    free(h_c);
    cudaFree(d_a);
    cudaFree(d_b);
    cudaFree(d_c);
}
