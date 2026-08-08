#ifndef LOGGER_H
#define LOGGER_H

#include <stdint.h>

#define MAX_DATETIME_BUFF_LEN 64

typedef enum {DEBUG, INFO, WARNING, ERROR, CRITICAL} LogLevel;
typedef struct {
    const char* red;
    const char* green;
    const char* yellow;
    const char* blue;
    const char* magenta;
    const char* cyan;
    const char* reset;
} Colors;

void logger(const char* logger_name, const uint8_t level, const char* message, ...);

#endif // LOGGER_H
