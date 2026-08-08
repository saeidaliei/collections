#include <stdlib.h>
#include <stdio.h>
#include <mpi.h>

#define MAX_ARRAY_BYTES_BIT_SHIFT_COUNT 27 // 1 << 29, 0.5GB
#define LOOP_COUNT 30

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
    if(rank == 0) {
        printf("count | Transfer size(B) | Transfer time(s) | Bandwidth(GB/s)\n");
    }
    // transfer loop for data of size 8 Bytes to 1 << MAX_ARRAY_BYTES_BIT_SHIFT_COUNT,
    // note that since we use a sizeof double (2^3) in malloc,
    // hence the MAX_ARRAY_BYTES_BIT_SHIFT_COUNT-3 in the loop exit condition.
    int max_size = (int)MAX_ARRAY_BYTES_BIT_SHIFT_COUNT - 3;
    for(int i = 0; i <= max_size; i++) {
        long int N = 1 << i;
        double* A = (double*)malloc(N * sizeof(double));
        for(int j = 0; j < N; j++) {
            A[j] = 1.0;
        }

        // run warm up MPI send-recv to remove any MPI setup cost
        short tag0 = 10;
        short tag1 = 20;
        for(int j = 0; j < 5; j++) {
            if(rank == 0) {
                MPI_Send(A, N, MPI_DOUBLE, 1, tag0, MPI_COMM_WORLD); // 1: dest rank, 0 sends to 1
                MPI_Recv(A, N, MPI_DOUBLE, 1, tag1, MPI_COMM_WORLD, &status); // 1: src rank, 0 receives from 1
            }
            else if(rank == 1) {
                MPI_Recv(A, N, MPI_DOUBLE, 0, tag0, MPI_COMM_WORLD, &status); // rank 0 tags the messages with tag0
                MPI_Send(A, N, MPI_DOUBLE, 0, tag1, MPI_COMM_WORLD); // rank 1 tags the messages with tag1
            }
        }

        // time the actual send-recv after warm up send-recv
        double start = MPI_Wtime();
        for(int j = 0; j < LOOP_COUNT; j++) {
            if(rank == 0) {
                MPI_Send(A, N, MPI_DOUBLE, 1, tag0, MPI_COMM_WORLD);
                MPI_Recv(A, N, MPI_DOUBLE, 1, tag1, MPI_COMM_WORLD, &status);
            }
            else if(rank == 1) {
                MPI_Recv(A, N, MPI_DOUBLE, 0, tag0, MPI_COMM_WORLD, &status);
                MPI_Send(A, N, MPI_DOUBLE, 0, tag1, MPI_COMM_WORLD);
            }
        }
        double elapsed_secs = MPI_Wtime() - start;
        if(rank == 0) {
            print_result(N, elapsed_secs);
        }

        free(A);
    }

    MPI_Finalize();
    return 0;
}
