add_executable(cubic_expression cubicexpression.cpp)
add_executable(range_proof rangeproof.cpp)
add_executable(sha256_proof sha256.cpp)

foreach(target cubic_expression range_proof sha256_proof)
    target_include_directories(${target} PUBLIC
        ${CMAKE_SOURCE_DIR}/deps/libsnark/
        ${CMAKE_SOURCE_DIR}/deps/libsnark/depends/libff/
        ${CMAKE_SOURCE_DIR}/deps/libsnark/depends/
    )

   target_link_libraries(${target}
        snark
        ff
        gmp
        gmpxx
        crypto
        sodium
        procps
    )

 target_compile_definitions(${target} PUBLIC
        CURVE_BN128
        BINARY_OUTPUT
        MONTGOMERY_OUTPUT
        USE_ASM
    )
endforeach()