#include <iostream>
#include <stdlib.h>
#include <cmath>
#include <cuda_runtime.h>

#define N 1024
#define TILE_WIDTH 16

__global__ void matrix_mul(double* A, double* B, double* C) {
    // shared memory can be thought of as an explicitely managed L1 cache,
    // that is private to each block, can be useful if we need to access 
    // data more than once, by one thread or by threads within a block,
    __shared__ double t_A[TILE_WIDTH * TILE_WIDTH];
    __shared__ double t_B[TILE_WIDTH * TILE_WIDTH];

    // each thread block takes one tile of C,
    // row and col are the indices of an element of C
    size_t bx = blockIdx.x; size_t by = blockIdx.y;
    size_t tx = threadIdx.x; size_t ty = threadIdx.y;
    size_t row = by * TILE_WIDTH + ty;
    size_t col = bx * TILE_WIDTH + tx;
    
    double sum = 0.0;
    // for a tile size of 2 for a matrix of size 4, possible 
    // amount of movements in either x or y direction is 2 tiles,
    // or N / TILE_WIDTH with possbile padding
    for (int i = 0; i < (N + TILE_WIDTH - 1) / TILE_WIDTH; i++) {
        // every thread in a block will load one element from 
        // global matrices to the shared tile matrices.
        // we're moving in the x direction for matrix A,
        // select row from A, row is constant for A for this tile
        if ((row < N) && ((i * TILE_WIDTH + tx) < N)) {
            t_A[ty * TILE_WIDTH + tx] = A[(N * row) + (i * TILE_WIDTH + tx)];
        }
        else {
            t_A[ty * TILE_WIDTH + tx] = 0.0;
        }

        // we're moving in the y direction for matrix B.
        // select col from B, col is constant for B for this tile
        if ((col < N) && ((i * TILE_WIDTH + ty) < N)) {
            t_B[ty * TILE_WIDTH + tx] = B[(i * TILE_WIDTH + ty) * N + col];
        }
        else {
            t_B[ty * TILE_WIDTH + tx] = 0.0;
        }
        
        // ensure threads/warps are done writing and reading
        __syncthreads();

        // partial partial multiplications
        for (int k = 0; k < TILE_WIDTH; k++) {
            sum += t_A[ty * TILE_WIDTH + k] * t_B[k * TILE_WIDTH + tx];
        }
        
        // ensure multiplication is finished, 
        // before overriding the tiles.
        __syncthreads();
    }

    if ((row < N) && (col < N)) {
        C[row * N + col] = sum;
    }
}

void run_matrix_mul(double* h_A, double* h_B, double* h_C) {
    size_t byte_size = N * N * sizeof(double);
    
    double* d_A;
    double* d_B;
    double* d_C;

    cudaMalloc((void**)&d_A, byte_size);
    cudaMalloc((void**)&d_B, byte_size);
    cudaMalloc((void**)&d_C, byte_size);

    cudaMemcpy(d_A, h_A, byte_size, cudaMemcpyHostToDevice);
    cudaMemcpy(d_B, h_B, byte_size, cudaMemcpyHostToDevice);
    cudaMemcpy(d_C, h_C, byte_size, cudaMemcpyHostToDevice);
    
    // 2D block with TILE_WIDTH * TILE_WIDTH threads
    dim3 dim_block(TILE_WIDTH, TILE_WIDTH);
    // 2D grid with N/TILE_WIDTH * N/TILE_WIDTH blocks
    dim3 dim_grid((N + TILE_WIDTH - 1) / TILE_WIDTH, 
                  (N + TILE_WIDTH - 1) / TILE_WIDTH);

    matrix_mul<<<dim_grid, dim_block>>>(d_A, d_B, d_C);

    cudaMemcpy(h_C, d_C, byte_size, cudaMemcpyDeviceToHost);

    cudaFree(d_A);
    cudaFree(d_B);
    cudaFree(d_C);
}

int main() {
    size_t byte_size = N * N * sizeof(double);

    double* A = (double*)malloc(byte_size);
    double* B = (double*)malloc(byte_size);
    double* C = (double*)malloc(byte_size);

    for (size_t i = 0; i < N * N; i++) {
        A[i] = 1.0;
        B[i] = 1.0;
        C[i] = 0.0;
    }

    run_matrix_mul(A, B, C);

    bool sucess = true;
    for (size_t i = 0; i < N * N; i++) {
        if (fabs(static_cast<double>(C[i] - N)) > 1e-5) {
            std::cerr << "Error in matrix multiplication at index: " << i << std::endl;
            sucess = false;
            break;
        }
    }
    if (sucess) {
        std::cout << "Tiled matrix multiplication test passed." << std::endl;
    }

    free(A);
    free(B);
    free(C);
}


