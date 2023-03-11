cmake_minimum_required(VERSION 3.10)
project(Lab6 C)

set(CMAKE_C_STANDARD 11)

add_executable(Lab6 main.c mgmres.c)
target_link_libraries(Lab6 m)
