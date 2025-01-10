# libiop

Libiop is a C++ library created in 2019 that implemented the three latest IOP-based schemes at that time. A feature
of libiop is that it packs three different proving systems as alternatives for users, including aurora , fractal and
ligero.   
However, libiop has not provided a complete toolchain for creating an R1CScircuit. We follow the API of the libraries and test the efficiency of randomly generated circuits with roughly the same quantity of constraints compared with the sample programs using libsnark，which developed by the same team of libiop. Toolkits: To support three proving systems, libiop uses a
namespace iop which contains aurora_iop, fractal_iop, and ligero_iop as specific protocols. The user-level APIs of the
three schemes are the same, which makes them convenient to use.

Next we show how to build Docker to run our sample program based on the libiop library.

## load and update libiop submodules
Before building docker, execute:
```
$ git submodule update --init --recursive libioplab/deps/libiop
```

## Docker setup

Create a Docker image. This will take a few minutes. You only have to do 
this once.
```
$ docker build -t libiop .
```

Spin up a Docker container from the image.
```
$ docker run -it --rm libiop
```

Please note that any changes you make in the container are not persistent. 

## Running examples

Navigate to the directory of the program you would like to run.
Our examples are at `/app/src`.
Before you execute the sample program, you need run the following code to compile it.
```
$ mkdir build
$ cd build
$ cmake ..
$ make
```

Then you can run the following code to execute the sample program.

```
$ cd app/build/src
$ ./cubic_aurora
$ ./cubic_ligero
$ ./cubic_fractal
$ ./rangeproof_aurora
$ ./rangeproof_ligero
$ ./rangeproof_fractal
$ ./sha256_aurora
$ ./sha256_ligero
$ ./sha256_fractal
```

## Modifying examples
you can write your own c++ file xxx.cpp in the src directory, and add the following code to the CMakeLists.txt.
```
add_executable(xxx xxx.cpp)
```

