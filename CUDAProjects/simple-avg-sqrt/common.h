#ifndef COMMON_H
#define COMMON_H 1

#include <cuda.h>
#include <mpi.h>

#define GPU_THREADS_PER_BLOCK 256
#define GPU_BLOCKS_PER_GRID 1000

#define DEFAULT_ARR_VALUE 4.0

extern int comm_size;
extern int proc_rank;

// NOTE: we assumed here that the calling processor is in 
// the default MPI communicator, MPI_COMM_WORL, and each
// MPI process is identified by the variable 'proc_rank'
#define MPIErrorCheck(call) \
    do { \
        int ierr = (call); \
        if(ierr != MPI_SUCCESS) { \
            char err_str[MPI_MAX_ERROR_STRING]; \
            int err_str_len = 0; \
            MPI_Error_string(ierr, err_str, &err_str_len); \
            if(proc_rank == 0) { \
                fprintf(stderr, "MPI Error: %s:%d %d:%s\n", __FILE__, __LINE__, ierr, err_str); \
            } \
            MPI_Abort(MPI_COMM_WORLD, 1); \
        } \
    } while(0)

#define CUDAErrorCheck(call) \
    do { \
        cudaError_t cuErr = (call); \
        if(cuErr != cudaSuccess) { \
            if(proc_rank == 0) { \
                fprintf(stderr, "CUDA Error: %s:%d %d:%s\n", __FILE__, __LINE__, cuErr, cudaGetErrorString(cuErr)); \
            } \
            MPI_Abort(MPI_COMM_WORLD, 2); \
        } \
    } while(0)

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void get_sqrt(double* arr, size_t len);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // COMMON_H
