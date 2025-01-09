# snarkjs  
snarkjs is a JavaScript library for building and verifying zero-knowledge proofs (ZKPs). It provides multiple SNARK implementations, including groth16, plonk, and fflonk, and supports developers to integrate zero-knowledge proofs into web applications, blockchain applications, and other JavaScript environments.

Library framework of snarkjs:
In terms of circuits, snarkjs provides integration with Circom, which can generate circuits through the Circom compiler and generate corresponding proofs. The language style of Circom is similar to conventional programming languages, which is convenient for developers to learn and use. In addition, snarkjs also provides a command line tool (CLI) for developers to run directly in the terminal and quickly generate circuits and proofs.

The team also provides a Circomlib repository, which provides many circuits of cryptographic primitives, such as various comparison logics, ecdsa, sha256, etc.

## Docker setup

Create a Docker image. This will take a few minutes. You only have to do 
this once.
```
$ docker build -t snarkjs .
```

Spin up a Docker container from the image.
```
$ docker run -it --rm snarkjs
```

Please note that any changes you make in the container are not persistent. 


## Running examples

Navigate to the directory of the program you would like to run.
Our examples are at `/app`. 

```
$ ls
```
You can see 9 directories and easily find out their content by their names.
for example, 'cubic_groth16' means the sample program is about zk of cubic expression using groth16.

Enter a directory, you'll find 4 files:
- a .circom file, which is the circuit file
- a input.json file, which is the input file
- a start.sh file, which contains the full steps to run the program. 
- a quick_start.sh file, which can be used when you have a final.ptau file. The final.ptau file contains the public parameters required by the circuit and is used to ensure the credibility of the circuit. It is a necessary file in the proof process. snarkjs provides a command to generate this file. As the maximum constraints of the circuit increase, the file generation time is slower. snarkjs officially provides final.ptau files ranging from 256 constraints to 256 mega constraints. You can choose to generate it yourself or download it.

tips：To testsha256 requires a big ptau file, if you generate it yourself, it will take a long time.   

You can simply run the 'start.sh' to execute the examples.
```
$ ./start.sh
```
Or when you have a final.ptau file, you can run the 'quick_start.sh' to execute the examples.
```
$ ./quick_start.sh
```

## Modifying examples
Modifying examples is straightforward. Write your own circom file xxx.circom and change the input.json file.
Then you change the file name in the start.sh file to your own file name and run the start.sh file to execute your own application.


