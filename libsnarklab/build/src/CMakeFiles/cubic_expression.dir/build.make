# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.27

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/local/bin/cmake

# The command to remove a file.
RM = /usr/local/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/daqi/test1221/libsnarklab2

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/daqi/test1221/libsnarklab2/build

# Include any dependencies generated for this target.
include src/CMakeFiles/cubic_expression.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include src/CMakeFiles/cubic_expression.dir/compiler_depend.make

# Include the progress variables for this target.
include src/CMakeFiles/cubic_expression.dir/progress.make

# Include the compile flags for this target's objects.
include src/CMakeFiles/cubic_expression.dir/flags.make

src/CMakeFiles/cubic_expression.dir/cubicexpression.cpp.o: src/CMakeFiles/cubic_expression.dir/flags.make
src/CMakeFiles/cubic_expression.dir/cubicexpression.cpp.o: /home/daqi/test1221/libsnarklab2/src/cubicexpression.cpp
src/CMakeFiles/cubic_expression.dir/cubicexpression.cpp.o: src/CMakeFiles/cubic_expression.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/daqi/test1221/libsnarklab2/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object src/CMakeFiles/cubic_expression.dir/cubicexpression.cpp.o"
	cd /home/daqi/test1221/libsnarklab2/build/src && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT src/CMakeFiles/cubic_expression.dir/cubicexpression.cpp.o -MF CMakeFiles/cubic_expression.dir/cubicexpression.cpp.o.d -o CMakeFiles/cubic_expression.dir/cubicexpression.cpp.o -c /home/daqi/test1221/libsnarklab2/src/cubicexpression.cpp

src/CMakeFiles/cubic_expression.dir/cubicexpression.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/cubic_expression.dir/cubicexpression.cpp.i"
	cd /home/daqi/test1221/libsnarklab2/build/src && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/daqi/test1221/libsnarklab2/src/cubicexpression.cpp > CMakeFiles/cubic_expression.dir/cubicexpression.cpp.i

src/CMakeFiles/cubic_expression.dir/cubicexpression.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/cubic_expression.dir/cubicexpression.cpp.s"
	cd /home/daqi/test1221/libsnarklab2/build/src && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/daqi/test1221/libsnarklab2/src/cubicexpression.cpp -o CMakeFiles/cubic_expression.dir/cubicexpression.cpp.s

# Object files for target cubic_expression
cubic_expression_OBJECTS = \
"CMakeFiles/cubic_expression.dir/cubicexpression.cpp.o"

# External object files for target cubic_expression
cubic_expression_EXTERNAL_OBJECTS =

src/cubic_expression: src/CMakeFiles/cubic_expression.dir/cubicexpression.cpp.o
src/cubic_expression: src/CMakeFiles/cubic_expression.dir/build.make
src/cubic_expression: deps/libsnark/libsnark/libsnark.a
src/cubic_expression: deps/libsnark/depends/libff/libff/libff.a
src/cubic_expression: /usr/lib/x86_64-linux-gnu/libgmp.so
src/cubic_expression: /usr/lib/x86_64-linux-gnu/libgmp.so
src/cubic_expression: /usr/lib/x86_64-linux-gnu/libgmpxx.so
src/cubic_expression: deps/libsnark/depends/libzm.a
src/cubic_expression: src/CMakeFiles/cubic_expression.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/home/daqi/test1221/libsnarklab2/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX executable cubic_expression"
	cd /home/daqi/test1221/libsnarklab2/build/src && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/cubic_expression.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
src/CMakeFiles/cubic_expression.dir/build: src/cubic_expression
.PHONY : src/CMakeFiles/cubic_expression.dir/build

src/CMakeFiles/cubic_expression.dir/clean:
	cd /home/daqi/test1221/libsnarklab2/build/src && $(CMAKE_COMMAND) -P CMakeFiles/cubic_expression.dir/cmake_clean.cmake
.PHONY : src/CMakeFiles/cubic_expression.dir/clean

src/CMakeFiles/cubic_expression.dir/depend:
	cd /home/daqi/test1221/libsnarklab2/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/daqi/test1221/libsnarklab2 /home/daqi/test1221/libsnarklab2/src /home/daqi/test1221/libsnarklab2/build /home/daqi/test1221/libsnarklab2/build/src /home/daqi/test1221/libsnarklab2/build/src/CMakeFiles/cubic_expression.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : src/CMakeFiles/cubic_expression.dir/depend

