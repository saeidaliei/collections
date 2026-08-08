#include <stdio.h>

#include "logging.h"

static const char* LOGNAME = "main";

int main()
{
    logger(LOGNAME, INFO, "this is fed into logger func.");
    logger(LOGNAME, DEBUG, "this is another with %d args iniside it.", 5);

    return 0;
}
