add_executable(cubic_expression_groth16 cubicexpression_groth16.cpp)
add_executable(range_proof_groth16 rangeproof_groth16.cpp)
add_executable(sha256_proof_groth16 sha256_groth16.cpp)
add_executable(cubic_expression_bctv14 cubicexpression_bctv14.cpp)
add_executable(range_proof_bctv14 rangeproof_bctv14.cpp)
add_executable(sha256_proof_bctv14 sha256_bctv14.cpp)
add_executable(cubic_expression_gm17 cubicexpression_gm17.cpp)
add_executable(range_proof_gm17 rangeproof_gm17.cpp)
add_executable(sha256_proof_gm17 sha256_gm17.cpp)

foreach(target cubic_expression_groth16 range_proof_groth16 sha256_proof_groth16 cubic_expression_bctv14 range_proof_bctv14 sha256_proof_bctv14 cubic_expression_gm17 range_proof_gm17 sha256_proof_gm17)
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