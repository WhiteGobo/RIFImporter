cmake_minimum_required(VERSION 3.25)

project(RIFImporter VERSION 0.1.0 LANGUAGES C)
set(CMAKE_C_FLAGS "-g") #enable lines in memtest

include(packages.cmake)

add_subdirectory(src)

enable_testing()
add_subdirectory(test)
