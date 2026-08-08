#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <string.h>

#define NX 100

#define LEFT 10.0
#define RIGHT 1.0

#define ERR_TOLERANCE 0.00001
#define PRINT_FREQ 1000
#define MAX_ITER 50000

double get_norm(double* phi, size_t n) {
    double total_norm = 0.0;
    for(size_t i = 1; i < n-1; i++) {
        double tmp = 2.0 * phi[i] - (phi[i-1] + phi[i+1]);
        total_norm += tmp * tmp;
    }
    return sqrt(total_norm);
}

int main(int argc, char** argv) {
    size_t nx2 = NX + 2;
    double* phi = (double*)malloc(nx2 * sizeof(double));
    double* phi_new = (double*)malloc(nx2 * sizeof(double));

    phi[0] = phi_new[0] = LEFT;
    phi[nx2-1] = phi_new[nx2-1] = RIGHT;
    for(size_t i = 1; i < nx2-1; i++) {
        phi[i] = phi_new[i] = 0.0;
    }

    double init_norm = get_norm(phi, nx2);
    double relative_norm = 1.0;
    size_t itr;
    printf("%11s | %10s\n", "Iteration", "Relative norm");
    for(itr = 0; itr < MAX_ITER; itr++) {
        double norm = get_norm(phi, nx2);
        relative_norm = norm / init_norm;
        if(relative_norm < ERR_TOLERANCE) {
            break;
        }

        for(size_t i = 1; i < nx2-1; i++) {
            phi_new[i] = 0.5 * (phi[i-1] + phi[i+1]);
        }
        memcpy(phi, phi_new, nx2 * sizeof(double));

        if(itr % PRINT_FREQ == 0) {
            printf("%5zu %14.5lf\n", itr, relative_norm);
        }
    }
    printf("Terminated on iteration: %5zu - Relative norm: %1.5lf\n", itr, relative_norm);

    free(phi);
    free(phi_new);
    return 0;
}
