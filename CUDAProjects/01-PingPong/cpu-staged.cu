#include <stdlib.h>
#include <stdio.h>
#include <cuda.h>
#include <mpi.h>

#define MAX_ARRAY_BIT_SHIFT 20
#define LOOP_COUNT 50

// do while because in situations like: if() cudaCheckError(...) else something
// with do while we can use it in if else blocks with one statement following it,
// that have no bracket scoping.
#define cudaCheckError(call) \
    do { \
        cudaError_t cuErr = call; \
        if(cuErr != cudaSuccess) { \
            if(rank == 0) { \
                fprintf(stderr, "CUDA Error: %s:%d: %s\n", __FILE__, __LINE__, cudaGetErrorString(cuErr)); \
            } \
            MPI_Finalize(); \
            exit(EXIT_FAILURE); \
        } \
    } while(0)

void print_result(long int num_elems, double elapsed_secs) {
    static int num_bytes = 1;
    long int total_bytes = 8 * num_elems;
    long int bytes_in_G = 1 << 30;
    double GB = (double)total_bytes / (double)bytes_in_G;
    double avg_sec_per_loop = elapsed_secs / (2.0 * (double)LOOP_COUNT);
    double GB_per_sec = GB / avg_sec_per_loop;
    printf("%2d %15li %20.9f %18.9f\n", num_bytes, total_bytes, elapsed_secs, GB_per_sec);
    num_bytes += 1;
}

int main(int argc, char** argv) {
    int size, rank;
    MPI_Status status;
    MPI_Init(&argc, &argv);
    MPI_Comm_size(MPI_COMM_WORLD, &size);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    if(size != 2) {
        if(rank == 0) {
            fprintf(stderr, "Expected 2 MPI processes, got: %d\n", size);
        }
        MPI_Finalize();
        exit(EXIT_FAILURE);
    }

    int num_devices = 0;
    cudaCheckError(cudaGetDeviceCount(&num_devices));
    cudaCheckError(cudaSetDevice(rank % num_devices));

    int max_bit_shift = MAX_ARRAY_BIT_SHIFT - 3; // 3 becaue double is 8 bytes = 2^3 bytes
    for(int i = 0; i <= max_bit_shift; i++) {
        long int N = 1 << i;
        double* A = (double*)malloc(N * sizeof(double));
        for(int j = 0; j < N; j++) {
            A[j] = 1.0;
        }
        double* d_A;
        cudaCheckError(cudaMalloc((void**)&d_A, N * sizeof(double)));
        // order of params: dst, src, size, kind
        cudaCheckError(cudaMemcpy(d_A, A, N * sizeof(double), cudaMemcpyHostToDevice));

        int tag0 = 10;
        int tag1 = 20;
        for(int j = 0; j < 5; j++) {
            if(rank == 0) {
                cudaCheckError(cudaMemcpy(A, d_A, N * sizeof(double), cudaMemcpyDeviceToHost));
                MPI_Send(A, N, MPI_DOUBLE, 1, tag0, MPI_COMM_WORLD);
                MPI_Recv(A, N, MPI_DOUBLE, 1, tag1, MPI_COMM_WORLD, &status);
                cudaCheckError(cudaMemcpy(d_A, A, N * sizeof(double), cudaMemcpyHostToDevice));
            }
            else if(rank == 1) {
                MPI_Recv(A, N, MPI_DOUBLE, 0, tag0, MPI_COMM_WORLD, &status);
                cudaCheckError(cudaMemcpy(d_A, A, N * sizeof(double), cudaMemcpyHostToDevice));
                cudaCheckError(cudaMemcpy(A, d_A, N * sizeof(double), cudaMemcpyDeviceToHost));
                MPI_Send(A, N, MPI_DOUBLE, 0, tag1, MPI_COMM_WORLD);
            }
        }

        double start = MPI_Wtime();
        for(int j = 0; j < LOOP_COUNT; j++) {
            if(rank == 0) {
                // order of params: dst, src, size, kind
                cudaCheckError(cudaMemcpy(A, d_A, N * sizeof(double), cudaMemcpyDeviceToHost));
                MPI_Send(A, N, MPI_DOUBLE, 1, tag0, MPI_COMM_WORLD);
                MPI_Recv(A, N, MPI_DOUBLE, 1, tag1, MPI_COMM_WORLD, &status);
                cudaCheckError(cudaMemcpy(d_A, A, N * sizeof(double), cudaMemcpyHostToDevice));
            }
            else if(rank == 1) {
                MPI_Recv(A, N, MPI_DOUBLE, 0, tag0, MPI_COMM_WORLD, &status);
                cudaCheckError(cudaMemcpy(d_A, A, N * sizeof(double), cudaMemcpyHostToDevice));
                cudaCheckError(cudaMemcpy(A, d_A, N * sizeof(double), cudaMemcpyDeviceToHost));
                MPI_Send(A, N, MPI_DOUBLE, 0, tag1, MPI_COMM_WORLD);
            }
        }
        double elapsed_secs = MPI_Wtime() - start;
        if(rank == 0) {
            print_result(N, elapsed_secs);
        }

        free(A);
        cudaCheckError(cudaFree(d_A));
    }

    MPI_Finalize();
    return 0;
}
