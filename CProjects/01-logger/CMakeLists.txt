cmake_minimum_required(VERSION 3.1...3.24)

project(logging VERSION 1.0 LANGUAGES C)

add_subdirectory(src)
include_directories(${logging_SOURCE_DIR}/inc)

enable_testing()
add_subdirectory(tests)

macro(MACRO_ENSURE_OUT_OF_SOURCE_BUILD MSG)
    string(COMPARE EQUAL "${CMAKE_SOURCE_DIR}" "${CMAKE_BINARY_DIR}" insource)
    get_filename_component(PARENTDIR ${CMAKE_SOURCE_DIR} PATH)
    string(COMPARE EQUAL "${CMAKE_SOURCE_DIR}" "${PARENTDIR}" insourcesubdir)
    if(insource OR insourcesubdir)
        message(FATAL_ERROR "${MSG}")
    endif(insource OR insourcesubdir)
endmacro(MACRO_ENSURE_OUT_OF_SOURCE_BUILD)
MACRO_ENSURE_OUT_OF_SOURCE_BUILD("${CMAKE_PROJECT_NAME} requires an out of source build.")
