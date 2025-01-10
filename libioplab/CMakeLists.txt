cmake_minimum_required(VERSION 3.0)
project(libioplab)

# 添加 libiop
add_subdirectory(deps/libiop)

# 添加你的源文件目录
add_subdirectory(src)