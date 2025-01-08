# libsnark

libsnark is a C++ library for building and verifying zero-knowledge proofs (ZKP). It aims to provide developers with efficient and secure tools for building applications based on zero-knowledge proofs. It provides multiple SNARK implementations for cryptography researchers and developers.

The library framework of libsnark mainly consists of three parts:

1. General proof system:
libsnark Implemented multiple proof system, including BCTV14, Groth16, GM17 etc.
Their names in libsnark correspond to the following:
BCTV14: ppzksnark
Groth16: gg_ppzksnark
GM17: se_ppzksnark

2. Gadget libraries (gadgetlib1 and gadgetlib2), which provide components for building circuits, allowing users to define more complex computational logic, supporting primitives such as commitment, curve, pairing, hashing, etc.

3. Examples and test cases: libsnark provides multiple examples and test cases to help developers understand how to use the library in different application scenarios.

## Docker setup

Create a Docker image. This will take a few minutes. You only have to do 
this once.
```
$ docker build -t libsnark .
```

Spin up a Docker container from the image.
```
$ docker run -it --rm libsnark
```

Please note that any changes you make in the container are not persistent. 

## Architecture

## Running examples

Navigate to the directory of the program you would like to run.
Our examples are at `/app/src`.
Before you execute the sample program, you need run the following code to compile it.
```
$ cd build
$ cmake ..
$ make
```

Then you can run the following code to execute the sample program.

```
$ cd app/build/src
$ ./cubic_expression
$ ./range_proof
$ ./sha256
```

## Modifying examples
you can write your own c++ file xxx.cpp in the src directory, and add the following code to the CMakeLists.txt.
```
add_executable(xxx xxx.cpp)
```

