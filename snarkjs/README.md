# Snarkjs  
Snarkjs is a javascript library started in 2019 which implements Groth16 , Plonk , and FFlonk. The goal of snarkjs is to provide comprehensive zk toolchains for website and blockchain scenarios. In zero-knowledge concepts, snarkjs provides a compiler called circom, and the syntax is similar to C or javascript.  
Circom in snarkjs is a powerful compiler. It allows users to independently write their constraints in a file with a high-level language (e.g., DSL) that is very similar to C and JavaScript. When writing circom codes, the user is not required to use snarkjs APIs, which distinguishes snarkjs from all other zk-SNARK libraries. When using circom, we successfully implement our sample programs and then compile them to R1CS for testing. Other zk-SNARK libraries, such as Arkworks , are also gradually starting to support circom.  
The team also provides a Circomlib repository, which provides many circuits of cryptographic primitives, such as various comparison logics, ecdsa, sha256, etc.

Next we show how to build Docker to run our sample program based on the Snarkjs library.

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


