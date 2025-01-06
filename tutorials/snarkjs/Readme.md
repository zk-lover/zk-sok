### Sample programs using the snarkjs library

We have implemented the three example programs mentioned in our paper based on snarkjs, including:

* A Cubic Expression

* Range Proof

* Hash Function

Snarkjs uses the Groth16 Protocol, PLONK, and FFLONK. Therefore, we implemented our example programs using these three schemes. The differences between these schemes can be found in our paper. The program source code is located in the `snarkjs/test/circuit` directory under nine subdirectories (for three different ZKPs) with the same name circom files: `cubic_expression`, `range_proof`, and `sha256`.

For the "A Cubic Expression" example program, the source code based on the Groth16 scheme is located at:

> snarkjs\test\circuit\cubic_expression.circom

The same applies to the other example programs.

For the snarkjs library, the example programs we wrote are circom circuits, and then ZKPs are generated through commands. Therefore, this tutorial will first annotate our example program code and then introduce the process from the example program's circuit to ZKP to demonstrate the use of snarkjs.

