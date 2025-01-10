# libsnark
Libsnark is a C++ project started in 2014 and is the first project that aims to provide comprehensive support for zk-SNARKs. The schemes implemented in libsnark such as `BCTV14`, `Groth16`, `GM17` are all based on QAP because at that time only QAP-based schemes are efficient. Libsnark offers a great overlook of QAP-based schemes, as it not only includes the most popular `Groth16` scheme but also compares with former related works.  
The core toolkit in libsnark involves relation(defining the representation of NP language), reduction(containing functions that convert each elation), ppzksnark(implementing the core proof systems), and gadget (simplifying the procedure of specified constraints). Libsnark defines various relations like R1CS, TBCS (Two-input Boolean Circuit Satisfiability), USCS (Unitary-Square Constraint System), RAM (Random Access Model), etc. A notable feature of libsnark is that those relations can be converted fro to another using functions in reduction. Nowadays, the relations except R1CS and RAM are rarely used due to efficiency issues, and our tests are based on R1CS. Another feature of libsnark is its gadget tool. In the gadget, libsnark implements four data types such as variables, array, linear combination variable, and linear combination array. The gadget in the libsnark helps programmers manage constraints and mitigate the problem of not having a general compiler.

Next we show how to build Docker to run our sample program based on the libsnark library.

## load and update libsnark submodules
Before building docker, execute:
```
$ git submodule update --init --recursive libsnarklab/deps/libsnark
```

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
$ mkdir build
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

