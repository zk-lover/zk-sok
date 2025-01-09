# gnark

gnark is a zero-knowledge proof (ZKP) library implemented in Go, focusing on providing efficient and easy-to-use tools to build and verify zero-knowledge proofs. It provides implementations of zero-knowledge proof protocols (such as Plonk, Groth16, etc.) and supports the construction of various types of circuits, suitable for application scenarios such as blockchain, privacy-preserving computing, and encryption protocols.

## Docker setup

Create a Docker image. This will take a few minutes. You only have to do 
this once.
```
$ docker build -t gnark .
```

Spin up a Docker container from the image.
```
$ docker run -it --rm gnark .
```

Please note that any changes you make in the container are not persistent. 

## Running examples

Navigate to the directory of the program you would like to run.
Our examples are at the directories under `/root`.
Run the following code to execute the examples.
```
$ cd testCubitequation
$ go build -o testCubitequation
$ ./testCubitequation
```

```
$ cd testCubquation_plonk
$ go build -o testCubquation_plonk
$ ./testCubquation_plonk5
```

```
$ cd testrangeproofs
$ go build -o testrangeproofs
$ ./testrangeproofs
```

```
$ cd testrangeproofs_plonk
$ go build -o testrangeproofs_plonk
$ ./testrangeproofs_plonk
```

```
$ cd testsha256
$ go build -o testsha256
$ ./testsha256
```

```
$ cd testsha256_plonk
$ go build -o testsha256_plonk
$ ./testsha256_plonk
```

## Modifying examples
Modifying examples is straightforward. 
Create a new directory dir1 under /root, init and write your own go file xxx.go and compile.
```
$ mkdir dir1
$ go mod init dir1
//after write your go file
$ go mod tidy
$ go build -o xxx
```
Then you can run your code using
```
$ ./xxx
```


