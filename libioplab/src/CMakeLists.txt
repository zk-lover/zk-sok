cmake_minimum_required(VERSION 3.10)
project(iop_benchmarks)

# 在项目开始时禁用 benchmark 测试
set(BENCHMARK_ENABLE_TESTING OFF)
set(BENCHMARK_ENABLE_GTEST_TESTS OFF)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 添加依赖库
find_package(OpenSSL REQUIRED)

# 添加所有可执行文件
add_executable(cubic_aurora cubic_sim_Aurora.cpp)
add_executable(cubic_fractal cubic_sim_Fractal.cpp)
add_executable(cubic_ligero cubic_sim_Ligero.cpp)

add_executable(rangeproof_aurora rangeproof_sim_Aurora.cpp)
add_executable(rangeproof_fractal rangeproof_sim_Fractal.cpp)
add_executable(rangeproof_ligero rangeproof_sim_Ligero.cpp)

add_executable(sha256_aurora sha256_sim_Aurora.cpp)
add_executable(sha256_fractal sha256_sim_Fractal.cpp)
add_executable(sha256_ligero sha256_sim_Ligero.cpp)

# 为每个可执行文件添加链接库和包含目录
foreach(target 
    cubic_aurora cubic_fractal cubic_ligero
    rangeproof_aurora rangeproof_fractal rangeproof_ligero
    sha256_aurora sha256_fractal sha256_ligero)
    
    target_link_libraries(${target}
        PRIVATE
        ff
        iop
        ${OPENSSL_LIBRARIES}
    )

    target_include_directories(${target}
        PRIVATE
        ${CMAKE_SOURCE_DIR}/deps/libiop
        ${CMAKE_SOURCE_DIR}/deps/libiop/depends
        ${OPENSSL_INCLUDE_DIR}
    )
endforeach()