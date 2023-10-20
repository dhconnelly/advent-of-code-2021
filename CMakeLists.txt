cmake_minimum_required(VERSION 3.1)

project(
    AdventOfCode2021
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_executable(day24 src/bin/day24.cc)
