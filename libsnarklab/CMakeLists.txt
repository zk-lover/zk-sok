cmake_minimum_required(VERSION 3.0)
project(libsnarklab)

# 添加 libsnark
add_subdirectory(deps/libsnark)

# 添加你的源文件目录
add_subdirectory(src)