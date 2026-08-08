#include <stdio.h>

// Quake III Fast Inverse Square Root Algorithm,
// computes 1 / sqrt(x)
// for an explanation of the algorithm, see:
// youtu.be/p8u_k2LIZyo
float Q_rsqrt(float number) {
    long i;
    float x2, y;
    const float threehalfs = 1.5F;

    x2 = number * 0.5F;
    y = number;
    i = * (long*) &y; // interprete the floating bits as long bits
    i = 0x5f3759df - ( i>>1 ); // calculate the inverse sqaure root
    y = * (float*) &i; // reinterprete the long bits as floating bits
    y = y * (threehalfs - (x2 * y * y)); // 1st iteration of newton approximation method
    // y = y * (threehalfs - (x2 * y * y)); // 2nd iteration of newton approximation method

    return y;
}

int main() {

    float rsq = Q_rsqrt(4.0);
    printf("%lf\n", rsq);

}
