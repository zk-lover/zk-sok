## Sample programs using the plonky2 library

We have implemented the three example programs mentioned in our paper based on the plonky2 library, including:

* A Cubic Expression

* Range Proof

* SHA256

The program source codes can be found under

> plonky2/src/

and

> plonky2-sha256/src/

This tutorial will provide an introduction to our example program codes to demonstrate the use of plonky2.

Blow, we will introduce the implementation logic of each example programs based on the plonky2 library.

**You can find detailed annotations of the program codes in the markdown files within this directory.**

### 1. A Cubic Expression

This code uses the Plonky2 library to construct and verify a simple circuit proof. The main logic of the code is to prove a statement about a polynomial: "I know a number x such that x³ + x + 1 equals some value." Here's a breakdown of the implementation logic:

##### Import Libraries and Define Types

* Uses `anyhow::Result` for error handling.

* Utilizes various modules from the `plonky2` library to build circuits and handle proofs.

* Defines constants `D` and type aliases `C` and `F` for circuit configuration.

##### Circuit Configuration and Construction

* Creates a standard recursive zero-knowledge circuit configuration.

* Uses `CircuitBuilder` to construct the circuit, defining a virtual target `x` and building a polynomial circuit through a series of multiplication and addition operations.

##### Set Public Inputs and Partial Witness

* Registers public inputs `x` and `e`, where `e` is the result of the polynomial computation.

* Creates a `PartialWitness` instance and sets the value of `x` to 3.

##### Generate and Verify Proof

* Constructs circuit data and generates a proof.

* Calculates and outputs the size of the proof in bytes.

* Verifies the generated proof.

* Calculates and outputs the time taken to generate and verify the proof.

* Outputs the result of the polynomial computation and the time taken for proof generation and verification.

### 2. Range Proof

This code snippet uses the Plonky2 library to prove that a given value lies within a specified range. Here's a breakdown of the implementation logic:

##### Import Libraries and Define Types

* It uses `anyhow::Result` for error handling.

* It imports various modules from the `plonky2` library to build circuits and handle proofs.

* Constants `D` and type aliases `C` and `F` are defined, where `F` is a field type.

##### Circuit Configuration and Construction

* A standard recursive zero-knowledge circuit configuration `CircuitConfig` is created.

* A `CircuitBuilder` object is initialized using this configuration.

##### Add Virtual Target and Public Input

* A virtual target `value` is added, representing the secret value to be verified.

* This value is registered as a public input so it can be printed later.

##### Range Check

* The number of gates in the circuit is printed before and after adding the range check.

* The `builder.range_check(value, log_max)` ensures that `value` is within the range of `2^log_max`.

##### Generate and Verify Proof

* A `PartialWitness` object is created, and the target value is set to `10086`.

* It prints the number of gates before building the circuit.

* Circuit data is built using `builder.build::<C>()`.

* A proof is generated, and the time taken to generate the proof is measured.

* The proof is converted to a byte array, and its size is printed.

* The proof is verified, and the verification time is measured.

* Outputs the time taken to generate and verify the proof in milliseconds.

### 3. SHA256

This code implements a process to verify a SHA-256 hash using zero-knowledge proofs (ZKP). Here's the implementation logic:

##### Imports

The code imports several libraries, including `plonky2` for circuit building and proof generation, and `sha2` for computing the SHA-256 hash.

##### `prove_sha256` Function

* Takes a byte array `msg` as input.

* Uses `Sha256` to compute the hash of the input message.

* Converts the message into a bit array.

* Calculates the bit length of the message and prints the required block count.

* Defines the circuit depth `D` and configuration type `C`.

* Creates a circuit builder `builder`.

* Generates circuit targets using the `make_circuits` function.

* Creates a partial witness `pw` and sets the message bit targets.

* Converts the computed hash into a bit array and adds assertions in the circuit to verify the hash.

* Prints circuit statistics, including the number of gates and public inputs.

* Builds the circuit data and generates a proof.

* Converts the proof to bytes and prints its size.

* Verifies the generated proof.

##### `main` Function

* Initializes the logger.

* Sets the log format and filter level.

* Handles errors during logger initialization.

* Defines a message `msg`.

* Calls the `prove_sha256` function to verify the hash of the message.

