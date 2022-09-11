cmake_minimum_required(VERSION 3.23)
project(clox C)

set(CMAKE_C_STANDARD 23)

add_executable(
        clox
        src/compiler.c
        src/debug.c
        src/chunk.c
        src/main.c
        src/memory.c
        src/object.c
        src/scanner.c
        src/table.c
        src/value.c
        src/vm.c
)
