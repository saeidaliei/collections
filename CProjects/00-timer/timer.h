/*
 * A utility header to get the current time since epoch 
 * in nano second resolution. Depending on the platform, 
 * Linux, Windows or Apple we select the high-resoltuion timer.
 */

#ifndef TIMER_H
#define TIMER_H

#include <stdint.h>
#if defined (__linux)
#   define HAVE_POSIX_TIMER
#   include <time.h>
    // description of different clock ids: 
    // https://people.cs.rutgers.edu/~pxk/416/notes/c-tutorials/gettime.html
#   ifdef CLOCK_MONOTONIC
#       define CLOCKID CLOCK_MONOTONIC
#   else
#       define CLOCKID CLOCK_REALTIME
#   endif

#elif defined(__APPLE__)
#   define HAVE_MACH_TIMER
#   include <mach/mach_time.h>
#elif defined(__WIN32)
#   define WIN32_LEAN_AND_MEAN
#   include <windows.h>
#endif

static uint64_t get_nsecs() {
    static uint64_t is_init = 0;    
#if defined(__linux)
    static struct timespec res;
    if(is_init == 0) {
        clock_getres(CLOCKID, &res);
        is_init = 1;
    }
    uint64_t now;
    struct timespec spec;
    clock_gettime(CLOCKID, &spec);
    now = spec.tv_sec * 1.0e9 + spec.tv_nsec;
    return now;

#elif defined(__APPLE__)
    static mach_timebase_info_data_t info;
    if(is_init == 0) {
        mach_timebase_info(&info);
        is_init = 1;
    }
    uint64_t now;
    now = mach_absolute_time();
    now *= info.numer;
    now /= info.denom;
    return now;

#elif defined(__WIN32)
    static LARGE_INTEGER freq;
    if(is_init == 0) {
        QueryPerformanceFrequency(&freq);
        is_init = 1;
    }
    LARGE_INTEGER now;
    QueryPerformanceFrequency(&freq);
    return (uint64_t) ((1e9 * now.QuadPart) / freq.QuadPart);
#endif
}

#endif // TIMER_H
