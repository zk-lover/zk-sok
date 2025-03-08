# gnark

Gnark is a high-performance, open-source Golang library for creating zero-knowledge proofs, specifically zk-SNARK applications originating from 2022. Gnark implements two schemes, Groth16 and Plonk. One main feature is that Gnark provides a high-level language for specifying the proof’s logic, and its APIs allow developers to easily create, verify, and deploy zero-knowledge proofs. It also includes a built-in compiler that transforms the high-level language into a low-level representation that can be run on various platforms. Gnark aims to be user-friendly and has various tutorials, including an executable playground for beginners to learn its programming style.  
Gnark provides two relevant classes, frontend and backend. The output of the frontend function is a preprocessed circuit. With the circuit, backend functions choose a proving system, assign a valid witness, and output a proof. In the frontend, gnark provides a DSL specified in api class, which is convenient to add constraints. Additionally, gnark provides a set of pre-built circuit components, such as SHA256 and elliptic curve arithmetic, that can be used to build more complex circuits. 

Next we show how to build Docker to run our sample program based on the Gnark library.

## Docker setup

Create a Docker image. This will take a few minutes. You only have to do 
this once.
```
$ docker build -t gnark .
```

Spin up a Docker container from the image.
```
$ docker run -it --rm gnark
```

Please note that any changes you make in the container are not persistent. 

## Running examples

Navigate to the directory of the program you would like to run.
Our examples are at the directories under `/root`.
Run the following code to execute the examples.
```
$ cd testCubicequation_groth16  
$ go build -o testCubicequation_groth16
$ ./testCubicequation_groth16
```

```
$ cd testCubicequation_plonk
$ go build -o testCubicequation_plonk
$ ./testCubicequation_plonk
```

```
$ cd testrangeproofs_groth16
$ go build -o testrangeproofs_groth16
$ ./testrangeproofs_groth16
```

```
$ cd testrangeproofs_plonk
$ go build -o testrangeproofs_plonk
$ ./testrangeproofs_plonk
```

```
$ cd testrangeproofs_plonk
$ go build -o testrangeproofs_plonk
$ ./testrangeproofs_plonk
```

```
$ cd testsha256_groth16
$ go build -o testsha256_groth16    
$ ./testsha256_groth16
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


