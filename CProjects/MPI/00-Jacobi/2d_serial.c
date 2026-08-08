#include <stdlib.h>
#include <stdio.h>

#define NX 100
#define NY 100

#define TOP 1.0
#define LEFT 1.0
#define RIGHT 1.0
#define BOTTOM 10.0

#define TOLERANCE 0.0001
#define NPRINT 1000
#define MAX_ITER 50000

int main(int argc, char** argv) {
    // for averaging the edge rows and columns,
    // we need extra column and row in each direction
    size_t nx2 = NX + 2;
    size_t ny2 = NY + 2;
    double* phi = (double*)malloc(nx2 * ny2 * sizeof(double));
    double* phi_new = (double*)malloc(nx2 * ny2 * sizeof(double));
    
    for(size_t )
    
    return 0;
}
