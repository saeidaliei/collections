#include <stdlib.h>
#include <stdio.h>
#include <mpi.h>

#include "common.h"

/*
 * Root MPI process creates an array, initializes it.
 * We scatter the array to available MPI processes, 
 * Each MPI process launches a GPU kernel to compute 
 * the square root of each element of the array, for which
 * it needs to copy the data per MPI process in the Host to 
 * Device and back to the Host after computation. Then we
 * MPI_Reduce each sum into a global sum and report the average.
 */

static void init_array(double* arr, size_t len) {
    for(size_t i = 0; i < len; i++) {
        arr[i] = DEFAULT_ARR_VALUE;
    }
}

static double sum_array(double* arr, size_t len) {
    double sum = 0.0;
    for(size_t i = 0; i < len; i++) {
        sum += arr[i];
    }
    return sum;
}

int main(int argc, char** argv) {
    int comm_size, proc_rank;
    MPIErrorCheck(MPI_Init(&argc, &argv));
    MPIErrorCheck(MPI_Comm_size(MPI_COMM_WORLD, &comm_size));
    MPIErrorCheck(MPI_Comm_rank(MPI_COMM_WORLD, &proc_rank));

    size_t arr_len_local = GPU_THREADS_PER_BLOCK * GPU_BLOCKS_PER_GRID;
    size_t arr_len = arr_len_local * comm_size;

    double* data = NULL;
    if(proc_rank == 0) {
        printf("MPI number of processes: %d\n", comm_size);
        data = (double*)malloc(arr_len * sizeof(double));
        init_array(data, arr_len);
    }

    double* data_local = (double*)malloc(arr_len_local * sizeof(double));
    MPIErrorCheck(MPI_Scatter(data, arr_len_local, MPI_DOUBLE, data_local, arr_len_local, MPI_DOUBLE, 0, MPI_COMM_WORLD));
    if(proc_rank == 0) {
        free(data); // no longer needed, as we have scattered it
    }
    
    get_sqrt(data_local, arr_len_local); // inplace sqrt of input array
    double sum_local = sum_array(data_local, arr_len_local);
    double sum_total = 0.0;
    MPIErrorCheck(MPI_Reduce(&sum_local, &sum_total, 1, MPI_DOUBLE, MPI_SUM, 0, MPI_COMM_WORLD)); // 1 is the count

    if(proc_rank == 0) {
        double avg = sum_total / arr_len;
        printf("Average: %lf\n", avg);
    }
    
    free(data_local);
    MPIErrorCheck(MPI_Finalize());
    return 0;
}
