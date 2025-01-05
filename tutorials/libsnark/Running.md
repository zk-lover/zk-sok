### Run the example programs

We recommend running our example programs in Ubuntu. You can first refer to [Howard Wu](https://github.com/howardwu/libsnark-tutorial) for installing some dependencies of libsnark;

We have already set up the development environment, so you can compile and run the example programs directly by following the steps below.

1. Enter the libsnark-playground folder and create a build directory to store all the generated build files. This will keep the project files organized.

   ```cma
   mkdir build
   cd build
   ```

2. Next, generate the build files and compile.

   ```
   cmake ..
   make
   ```

3. After that, you can run the example programs.

   ```
   ./src/cubicexpression
   ./src/rangeproof
   ./src/sha256
   ```

### Your own code

If you want to write your own code using the libsnark library, you can place your code in the libsnark-playground/src folder and modify the CMakeLists.txt file in that folder. Then, compile and run it. This will be more convenient.

If you want to setting up a development environment from start, and building zkSNARK application,you can refer to the steps on[Howard Wu](https://github.com/howardwu/libsnark-tutorial).
