cmake_minimum_required(VERSION 3.25)
project(jong_bangla_fcitx5 LANGUAGES CXX)

find_package(Fcitx5Core 5.1.23 REQUIRED)
find_package(Threads REQUIRED)
find_program(CARGO_EXECUTABLE cargo REQUIRED)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

get_filename_component(JONG_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../.." ABSOLUTE)
file(GLOB_RECURSE RUST_SOURCES CONFIGURE_DEPENDS
    "${JONG_ROOT}/src/*.rs" "${CMAKE_CURRENT_SOURCE_DIR}/ffi/src/*.rs")
set(RUST_LIBRARY "${CMAKE_CURRENT_BINARY_DIR}/rust/release/libjong_bangla_fcitx5_ffi.a")
add_custom_command(OUTPUT "${RUST_LIBRARY}"
    COMMAND "${CARGO_EXECUTABLE}" build --offline --locked --release
        --manifest-path "${JONG_ROOT}/Cargo.toml" -p jong-bangla-fcitx5-ffi
        --target-dir "${CMAKE_CURRENT_BINARY_DIR}/rust"
    DEPENDS ${RUST_SOURCES} "${JONG_ROOT}/Cargo.toml" "${JONG_ROOT}/Cargo.lock"
        "${CMAKE_CURRENT_SOURCE_DIR}/ffi/Cargo.toml"
    WORKING_DIRECTORY "${JONG_ROOT}"
    COMMENT "Building the local Rust composition boundary" VERBATIM)
add_custom_target(jong_rust DEPENDS "${RUST_LIBRARY}")
add_library(jong_ffi STATIC IMPORTED)
set_target_properties(jong_ffi PROPERTIES IMPORTED_LOCATION "${RUST_LIBRARY}")
add_dependencies(jong_ffi jong_rust)

add_library(jong-bangla MODULE addon.cpp)
target_compile_options(jong-bangla PRIVATE -Wall -Wextra -Wpedantic -Werror)
target_link_libraries(jong-bangla PRIVATE jong_ffi Fcitx5::Core Threads::Threads ${CMAKE_DL_LIBS} m)
target_link_options(jong-bangla PRIVATE -Wl,--no-undefined -Wl,--exclude-libs,ALL)
set_target_properties(jong-bangla PROPERTIES CXX_VISIBILITY_PRESET hidden
    VISIBILITY_INLINES_HIDDEN YES)

# Build-local metadata only. No install target or desktop configuration changes.
set(HARNESS_DATA "${CMAKE_CURRENT_BINARY_DIR}/harness-data")
configure_file(jong-bangla.conf.in "${HARNESS_DATA}/fcitx5/addon/jong-bangla.conf" @ONLY)
configure_file(jong-bangla-inputmethod.conf
    "${HARNESS_DATA}/fcitx5/inputmethod/jong-bangla.conf" COPYONLY)
configure_file(tests/null-addon.conf
    "${HARNESS_DATA}/fcitx5/addon/jong-harness-null.conf" COPYONLY)
configure_file(tests/null-inputmethod.conf
    "${HARNESS_DATA}/fcitx5/inputmethod/keyboard-us.conf" COPYONLY)

enable_testing()
add_executable(jong-fcitx5-harness tests/harness.cpp)
target_compile_definitions(jong-fcitx5-harness PRIVATE JONG_HARNESS_DATA="${HARNESS_DATA}")
target_compile_options(jong-fcitx5-harness PRIVATE -Wall -Wextra -Wpedantic -Werror)
target_link_libraries(jong-fcitx5-harness PRIVATE Fcitx5::Core)
add_dependencies(jong-fcitx5-harness jong-bangla)
add_test(NAME fcitx5-synthetic COMMAND jong-fcitx5-harness)
set_tests_properties(fcitx5-synthetic PROPERTIES TIMEOUT 30)

# Links the same release Rust archive without Fcitx, isolating C-ABI ownership.
add_executable(jong-ffi-memory tests/memory.cpp)
target_compile_options(jong-ffi-memory PRIVATE -Wall -Wextra -Wpedantic -Werror)
target_link_libraries(jong-ffi-memory PRIVATE jong_ffi Threads::Threads ${CMAKE_DL_LIBS} m)
add_test(NAME ffi-memory-lifecycle COMMAND jong-ffi-memory)
set_tests_properties(ffi-memory-lifecycle PROPERTIES TIMEOUT 120)
