cmake_minimum_required(VERSION 3.16.3)
project(lox C)
 
set(CMAKE_C_STANDARD 99)
 
add_executable(
        lox
        src/chunk.c
        src/compiler.c
        src/debug.c
        src/main.c
        src/memory.c
        src/scanner.c
        src/value.c
        src/vm.c
)
