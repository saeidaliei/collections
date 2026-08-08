include_directories(${logging_SOURCE_DIR}/inc)
add_compile_options(-Wall -O3)
add_library(log logging.c)
