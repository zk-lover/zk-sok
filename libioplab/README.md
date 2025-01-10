# libiop

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

