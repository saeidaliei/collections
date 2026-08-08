### Logging

A simple logging module.

```C++
#include "logging.hpp"

int main()
{
    // logging::Logger logger(Logging::ERROR, "main", "./log.txt");
    logging::Logger logger();
    logger.setName("main");
    logger.setLevel(Logging::ERROR);
    logger.setFilepath("./log.txt");
   
    logger.debug("Test debug message");
    logger.info("Test message number %d", 2);
    logger.warn("Warning in function %s", "main");
    logger.error("Error message");
    logger.critical("A fatal error occured");
}
```
