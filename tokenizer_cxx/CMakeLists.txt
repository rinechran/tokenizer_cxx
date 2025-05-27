SET(PROJECT_NAME tokenizer_cxx)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --verbose)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --verbose)
    set(TARGET_DIR "release")
endif ()

if (DEFINED ENV{CARGO_BUILD_TARGET})
    set(TARGET_DIR "$ENV{CARGO_BUILD_TARGET}/${TARGET_DIR}")
endif ()

set(RUST_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/lib${PROJECT_NAME}.a")
set(RUST_CXX "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}.cpp")
set(RUST_HDR "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}.h")

include(CheckIPOSupported)
check_ipo_supported(RESULT result OUTPUT output)
if(result)
    set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
endif()

add_library(${PROJECT_NAME} STATIC ${RUST_CXX})

set_target_properties(${PROJECT_NAME} PROPERTIES INTERPROCEDURAL_OPTIMIZATION TRUE)

add_custom_command(
    OUTPUT ${RUST_CXX} ${RUST_LIB}
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/${PROJECT_NAME}/src/lib.rs.cc ${RUST_CXX}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/${PROJECT_NAME}/src/lib.rs.h ${RUST_HDR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(${PROJECT_NAME} pthread dl ${RUST_LIB})

