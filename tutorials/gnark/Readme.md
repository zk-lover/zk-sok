### Sample programs using the gnark library

We have implemented the three example programs mentioned in our paper based on gnark, including:

* A Cubic Expression

* Range Proof

* Hash Function

The gnark library supports both Groth16 and Plonk schemes, so we implemented our example programs using both schemes.&#x20;

The source code for the programs is located in the `test.go` files within six files under the `gnarklab` folder.

For the "A Cubic Expression" example program we implemented, the source code based on the Groth16 scheme is located at:

> gnarklab\testCubitequation\test.go

The source code based on the Plonk scheme is located at:

> gnarklab\testCubquation_plonk\test.go

The same applies to the other example programs.

This tutorial will provide an overview of our example program code to demonstrate the use of gnark.
