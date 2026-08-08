#include <stdlib.h>
#include <stdint.h>
#include <stdarg.h>
#include <stdio.h>
#include <time.h>

#include "logging.h"

static const char* LOGNAME = "logger";

static const Colors set_colors() {
    Colors colors;
    colors.red     = "\x1b[31m";
    colors.green   = "\x1b[32m";
    colors.yellow  = "\x1b[33m";
    colors.blue    = "\x1b[34m";
    colors.magenta = "\x1b[35m";
    colors.cyan    = "\x1b[36m";
    colors.reset   = "\x1b[0m";

    return colors;
}

void logger(const char* name, const uint8_t level, const char* message, ...) {
    Colors colors = set_colors();

    const char* level_color;
    const char* level_string;
    switch(level) {
        case DEBUG:
            level_color = colors.green;
            level_string = "DEBUG";
            break;
        case INFO:
            level_color = colors.yellow;
            level_string = "INFO";
            break;
        case WARNING:
            level_color = colors.blue;
            level_string = "WARNING";
            break;
        case ERROR:
            level_color = colors.red;
            level_string = "ERROR";
            break;
        case CRITICAL:
            level_color = colors.red;
            level_string = "CRITICAL";
            break;
        default:
            logger(LOGNAME, ERROR, "no support for logging level: %d", level);
            exit(EXIT_FAILURE);
    }

    time_t now = time(NULL);
    struct tm* datetime = localtime(&now);
    char datetime_buff[MAX_DATETIME_BUFF_LEN];
    strftime(datetime_buff, sizeof(datetime_buff), "%d-%m-%Y", datetime);

    va_list args;
    va_start(args, message);
    fprintf(stderr, "%s[%s]%s %5s %s%5s%s: ", 
            colors.cyan, datetime_buff, colors.reset, 
            name, level_color, level_string, colors.reset);
    vfprintf(stderr, message, args);
    fprintf(stderr, "\n");
    va_end(args);
}


