#include <cstdio>
#include <stdio.h>
#include <stdlib.h>
#include <cuda_runtime.h>

#define CHECK_CUDA_CALL(val) check_cuda((val), #val, __FILE__, __LINE__)
template <typename T>
void check_cuda(T err, const char* const func, const char* const file, const int line) {
    if (err != cudaSuccess) {
        fprintf(stderr, "CUDA Runtime Error at: %s:%d\n", file, line);
        fprintf(stderr, "%s %s\n", cudaGetErrorString(err), func);
        exit(1);
    }
}
