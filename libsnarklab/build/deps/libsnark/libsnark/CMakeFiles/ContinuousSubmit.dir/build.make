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

# Utility rule file for ContinuousSubmit.

# Include any custom commands dependencies for this target.
include deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit.dir/compiler_depend.make

# Include the progress variables for this target.
include deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit.dir/progress.make

deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit:
	cd /home/daqi/test1221/libsnarklab2/build/deps/libsnark/libsnark && /usr/local/bin/ctest -D ContinuousSubmit

ContinuousSubmit: deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit
ContinuousSubmit: deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit.dir/build.make
.PHONY : ContinuousSubmit

# Rule to build all files generated by this target.
deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit.dir/build: ContinuousSubmit
.PHONY : deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit.dir/build

deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit.dir/clean:
	cd /home/daqi/test1221/libsnarklab2/build/deps/libsnark/libsnark && $(CMAKE_COMMAND) -P CMakeFiles/ContinuousSubmit.dir/cmake_clean.cmake
.PHONY : deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit.dir/clean

deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit.dir/depend:
	cd /home/daqi/test1221/libsnarklab2/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/daqi/test1221/libsnarklab2 /home/daqi/test1221/libsnarklab2/deps/libsnark/libsnark /home/daqi/test1221/libsnarklab2/build /home/daqi/test1221/libsnarklab2/build/deps/libsnark/libsnark /home/daqi/test1221/libsnarklab2/build/deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : deps/libsnark/libsnark/CMakeFiles/ContinuousSubmit.dir/depend

