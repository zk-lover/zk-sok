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
include deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/compiler_depend.make

# Include the progress variables for this target.
include deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/progress.make

# Include the compile flags for this target's objects.
include deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/flags.make

deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.o: deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/flags.make
deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.o: /home/daqi/test1221/libsnarklab2/deps/libsnark/libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp
deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.o: deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/daqi/test1221/libsnarklab2/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.o"
	cd /home/daqi/test1221/libsnarklab2/build/deps/libsnark/libsnark && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.o -MF CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.o.d -o CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.o -c /home/daqi/test1221/libsnarklab2/deps/libsnark/libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp

deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.i"
	cd /home/daqi/test1221/libsnarklab2/build/deps/libsnark/libsnark && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/daqi/test1221/libsnarklab2/deps/libsnark/libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp > CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.i

deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.s"
	cd /home/daqi/test1221/libsnarklab2/build/deps/libsnark/libsnark && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/daqi/test1221/libsnarklab2/deps/libsnark/libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp -o CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.s

# Object files for target demo_r1cs_ppzkadsnark
demo_r1cs_ppzkadsnark_OBJECTS = \
"CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.o"

# External object files for target demo_r1cs_ppzkadsnark
demo_r1cs_ppzkadsnark_EXTERNAL_OBJECTS =

deps/libsnark/libsnark/demo_r1cs_ppzkadsnark: deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/demo_r1cs_ppzkadsnark.cpp.o
deps/libsnark/libsnark/demo_r1cs_ppzkadsnark: deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/build.make
deps/libsnark/libsnark/demo_r1cs_ppzkadsnark: deps/libsnark/libsnark/libsnark_adsnark.a
deps/libsnark/libsnark/demo_r1cs_ppzkadsnark: deps/libsnark/libsnark/libsnark.a
deps/libsnark/libsnark/demo_r1cs_ppzkadsnark: deps/libsnark/depends/libff/libff/libff.a
deps/libsnark/libsnark/demo_r1cs_ppzkadsnark: /usr/lib/x86_64-linux-gnu/libgmp.so
deps/libsnark/libsnark/demo_r1cs_ppzkadsnark: /usr/lib/x86_64-linux-gnu/libgmp.so
deps/libsnark/libsnark/demo_r1cs_ppzkadsnark: /usr/lib/x86_64-linux-gnu/libgmpxx.so
deps/libsnark/libsnark/demo_r1cs_ppzkadsnark: deps/libsnark/depends/libzm.a
deps/libsnark/libsnark/demo_r1cs_ppzkadsnark: deps/libsnark/depends/libsnark_supercop.a
deps/libsnark/libsnark/demo_r1cs_ppzkadsnark: deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/home/daqi/test1221/libsnarklab2/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX executable demo_r1cs_ppzkadsnark"
	cd /home/daqi/test1221/libsnarklab2/build/deps/libsnark/libsnark && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/demo_r1cs_ppzkadsnark.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/build: deps/libsnark/libsnark/demo_r1cs_ppzkadsnark
.PHONY : deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/build

deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/clean:
	cd /home/daqi/test1221/libsnarklab2/build/deps/libsnark/libsnark && $(CMAKE_COMMAND) -P CMakeFiles/demo_r1cs_ppzkadsnark.dir/cmake_clean.cmake
.PHONY : deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/clean

deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/depend:
	cd /home/daqi/test1221/libsnarklab2/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/daqi/test1221/libsnarklab2 /home/daqi/test1221/libsnarklab2/deps/libsnark/libsnark /home/daqi/test1221/libsnarklab2/build /home/daqi/test1221/libsnarklab2/build/deps/libsnark/libsnark /home/daqi/test1221/libsnarklab2/build/deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : deps/libsnark/libsnark/CMakeFiles/demo_r1cs_ppzkadsnark.dir/depend

