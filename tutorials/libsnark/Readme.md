## Sample programs using the libsnark library

We have implemented the three example programs mentioned in our paper based on the libsnark library, including:

* A Cubic Expression

* Range Proof

* SHA256

The program source code can be found under&#x20;

> libsnarklab/src/

specifically&#x20;

> cubicexpression.cpp, rangeproof.cpp, and sha256.cpp

This tutorial will provide an introduction to our example program codes to demonstrate the use of libsnark.

Blow, we will introduce the implementation logic of each example programs based on the libsnark library.

**You can find detailed annotations of the program codes in the markdown files within this directory.**

### 1. A Cubic Expression

This code implements a simple zero-knowledge proof system to prove that the result of a polynomial, specifically ( x^3 + x + 1 ), equals a certain output value. It uses the R1CS SE-ppzkSNARK from the libsnark library. Here's the implementation logic:

##### Include Libraries and Namespaces

* Includes standard and libsnark library headers.

* Uses the `libsnark` namespaces.

##### Initialization

* Disables detailed output from the libff library.

* Defines a finite field type `FieldT` for circuit construction.

* Calls `default_r1cs_se_ppzksnark_pp::init_public_params()` to initialize curve parameters.

##### Create Protoboard and Define Variables

* Creates a `protoboard` object `pb` for constructing the constraint circuit.

* Defines four variables: `x`, `x_squared`, `x_cubed`, `out`.

* Allocates these variables on the protoboard using the `allocate` method.

* Sets the input size to 1, indicating that `out` is a public input.

##### Add Constraints

* Adds three constraints:

  * `x * x = x_squared`

  * `x_squared * x = x_cubed`

  * `x_cubed + x + 1 = out`

##### Generate and Verify Proof

* Retrieves the constraint system and generates a keypair (public and verification keys).

* Sets specific witness values for the variables `x`, `x_squared`, `x_cubed`, and `out`.

* Uses the generated public key and inputs to create a proof.

* Verifies the generated proof using the verification key.

* Outputs the number of constraints, proof size, proving time, and verification time.

### 2. Range Proof

This code implements a zero-knowledge proof system using the libsnark library to prove that a given number `x` is less than a maximum value `max` (2^32). Here's a breakdown of the implementation logic:

##### Include Libraries and Namespaces

* Includes standard and libsnark library headers.

* Uses the `libsnark` namespaces.

##### Main Function `main`

* Disables detailed profiling output.

* Defines a finite field type `FieldT` for elements in the circuit.

##### Initialize Curve Parameters

* Calls `default_r1cs_se_ppzksnark_pp::init_public_params()` to initialize curve parameters.

##### Create Protoboard

* Creates a `protoboard` object `pb` to construct the constraint system.

* Defines and allocates variables `x`, `max`, `less`, and `less_or_eq` to the protoboard.

##### Set Maximum Value and Create Comparison Gadget

* Initializes `max` to 1, then sets it to 2^32 through a loop.

* Uses `comparison_gadget` to create a gadget for comparing `x` and `max`.

* Generates R1CS constraints.

##### Generate and Verify Proof

* Uses `r1cs_se_ppzksnark_generator` to generate a keypair.

* Sets the value of `x` to 18.

* Generates R1CS witness.

* Uses `r1cs_se_ppzksnark_prover` to generate a proof.

* Uses `r1cs_se_ppzksnark_verifier_strong_IC` to verify the proof.

* Outputs the number of constraints, proof size, proving time, and verification time.

### 3. SHA256

This code implements a zero-knowledge proof system that specifically proves the correctness of the SHA256 hash of a given input without revealing the input itself.

##### Library and Namespace Inclusion

* Includes standard and libsnark library headers.

* Uses the `libsnark` and `std` namespaces.

##### Type Definition

* Defines `FieldT` as `libff::Fr<default_r1cs_se_ppzksnark_pp>`, which is a finite field element type used for the R1CS constraint system.

##### Proof Verification Function

* The `verify_proof` function checks if a given proof is valid using the `r1cs_se_ppzksnark_verifier_strong_IC` function.

##### Gadget Setup Function

* The `setup_gadget` function sets up a SHA256 hash gadget. It creates input and output variables and the hash gadget, then generates R1CS constraints.

* After generating the constraint system, it calls `r1cs_se_ppzksnark_generator` to produce a keypair (public and verification keys).

##### Main Function

* The `one_input_hash_gadget` function performs multiple iterations of proof generation and verification.

* Uses a `protoboard` to manage the constraint system.

* In each iteration, it generates R1CS witnesses for the input and output, creates a proof, and verifies the proof.

* Records and outputs the number of constraints, proof size, proving time, and verification time.

* Initializes public parameters.

* Calls the `one_input_hash_gadget` function to perform one iteration of proof and verification.



