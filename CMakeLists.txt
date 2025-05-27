cmake_minimum_required(VERSION 3.15)
project(example CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# === Subdirectory for Rust + CXX bridge ===
add_subdirectory(tokenizer_cxx)

# === Main executable ===
add_executable(main
    main.cpp
)

# === Linking ===
target_link_libraries(main
    PRIVATE
        tokenizer_cxx
)

# === Include Rust-generated headers ===
target_include_directories(main
    PRIVATE
        ${CMAKE_BINARY_DIR}/tokenizer_cxx
)
