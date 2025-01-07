# 添加多个可执行文件
add_executable(cubic_expression cubicexpression.cpp)
add_executable(range_proof rangeproof.cpp)
add_executable(sha256_proof sha256.cpp)

# 为每个可执行文件添加头文件路径
foreach(target cubic_expression range_proof sha256_proof)
    target_include_directories(${target} PUBLIC
        ${CMAKE_SOURCE_DIR}/deps/libsnark/
        ${CMAKE_SOURCE_DIR}/deps/libsnark/depends/libff/
        ${CMAKE_SOURCE_DIR}/deps/libsnark/depends/
    )

    # 添加库依赖
    target_link_libraries(${target}
        snark
        ff
        gmp
        gmpxx
        crypto
        sodium
        procps
    )

    # 添加编译选项
    target_compile_definitions(${target} PUBLIC
        CURVE_BN128
        BINARY_OUTPUT
        MONTGOMERY_OUTPUT
        USE_ASM
    )
endforeach()