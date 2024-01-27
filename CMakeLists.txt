cmake_minimum_required(VERSION 3.5...3.28)

project(
    zcc
    VERSION 0.1
    LANGUAGES CXX
)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(zcc)
target_include_directories(zcc PRIVATE src)

if(MSCV)
    target_compile_options(zcc PRIVATE /std:c++11 /W4 /WX)
else()
    target_compile_options(zcc PRIVATE -std=c++11 -Wall -Wextra -Wshadow -Werror -pedantic)
endif()

if(SANITIZE)
    if(NOT MSCV)
        target_compile_options(zcc PRIVATE -fsanitize=address,undefined -fno-omit-frame-pointer)
        target_link_options(zcc PRIVATE -fsanitize=address,undefined -fno-omit-frame-pointer)
    endif()
endif()

add_subdirectory(src)
