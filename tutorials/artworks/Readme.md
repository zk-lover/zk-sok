## Sample programs using the artworks library

We have implemented the three example programs mentioned in our paper based on the artworks library, including:

* A Cubic Expression

* Range Proof

* SHA256

The program source code can be found under

> arkworkslab/src/

This tutorial will provide an introduction to our example program codes to demonstrate the use of arkworks.

Blow, we will introduce the implementation logic of each example programs based on the artworks library.&#x20;

**You can find detailed annotations of the program codes in the markdown files within this directory.**

### 1. A Cubic Expression

This code implements a simple zero-knowledge proof system using the Groth16 algorithm. The main functionality is to define a circuit, generate a proof, and verify the proof. Here's a breakdown of the implementation logic:

1. **Import Libraries**: The code begins by importing necessary libraries that provide functionalities required for implementing zero-knowledge proofs, such as the Groth16 algorithm, elliptic curves, and random number generation.

2. **Define the Circuit**: The `CubicPlusLinearCircuit` struct defines a simple circuit that computes the formula (x^3 + x + 1 = y). The circuit has a single input variable `x`.

3. **Implement Constraint Synthesizer**: The `ConstraintSynthesizer` trait is implemented for the circuit, providing the `generate_constraints` function. In this function:

   * Variables are defined for the circuit: `x`, `x^2`, `x^3`, and `y`.

   * Constraints are added: `x * x = x^2`, `x^2 * x = x^3`, and `x^3 + x + 1 = y`.

   * The number of constraints is printed.

4. **Main Function**: The `main` function calls `test_prove_and_verify`, using the BLS12-381 elliptic curve to perform proof and verification.

5. **Proof and Verification Function**: The `test_prove_and_verify` function performs the following steps:

   * Creates a random number generator.

   * Executes the Groth16 setup to generate the proving key (pk) and verifying key (vk).

   * Calculates and prints the uncompressed sizes of pk and vk.

   * Generates a random `x` and computes `y = x^3 + x + 1`.

   * Uses the Groth16 algorithm to generate a proof.

   * Prints the uncompressed size of the proof.

   * Verifies the proof's validity and prints the result.

   * Calculates and prints the time taken for proving and verifying.

### 2. Range Proof

This code implements a range proof circuit using zero-knowledge proof with the `ark-groth16` library. Specifically, it defines a circuit to prove that a number `x` is within the range `[0, 2^32]` and uses the Groth16 protocol for proving and verifying. Here's the implementation logic:

1. **Circuit Definition**:

   * A struct `RangeProofCircuit` is defined, containing an optional field `x`, which represents the number to be proven.

   * The `generate_constraints` method is implemented for the `ConstraintSynthesizer` trait to generate the circuit's constraints.

2. **Constraint Generation**:

   * The input variable `x` is declared as an input to the circuit.

   * `x` is decomposed into 32 binary bits, and a witness variable is created for each bit.

   * Constraints are added for each bit to ensure its value is either 0 or 1 (i.e., `bit * (1 - bit) = 0`).

   * A constraint is added to ensure that the combination of these bits equals `x`.

3. **Main Function**:

   * The `test_prove_and_verify` function is called to perform the proof and verification.

4. **Proof and Verification**:

   * The circuit is initialized with a random number generator.

   * The `Groth16::setup` function is called to set up the circuit, generating the proving key and verifying key.

   * The uncompressed sizes of the proving key and verifying key are calculated and printed.

   * A random number `x` is generated, and the `Groth16::prove` function is called to generate the proof.

   * The uncompressed size of the proof is printed.

   * The proof is verified using `Groth16::verify_with_processed_vk`, and the verification result is printed.

   * The time taken for proving and verifying is calculated and printed.

### 3. SHA256

This code implements a zero-knowledge succinct non-interactive argument of knowledge (zk-SNARK) using the Groth16 protocol to verify a SHA256 hash. Here's a breakdown of the implementation logic:

1. **Library** The code begins by importing necessary libraries and modules that provide functionalities for zero-knowledge proofs, SHA256 hashing, random number generation, etc.

2. **SHA256 Circuit Definition**:

   * The `Sha256Circuit` struct defines a circuit with two optional fields: `preimage` (the input to be hashed) and `hash` (the expected hash value).

   * The `generate_constraints` method implements the logic for generating constraints for the circuit. Here, both `preimage` and `hash` are defined as private inputs (witnesses) rather than public inputs.

   * It uses the `Sha256Gadget` to compute the hash of the `preimage` and compares the computed hash with the provided `hash` to ensure they are equal.

3. **Main Function**:

   * The `main` function calls the `test_prove_and_verify` function to test the proving and verification process of the circuit.

4. **Proving and Verification Process**:

   * The `test_prove_and_verify` function first initializes a random number generator and sets up the input data (`preimage` and `hash`).

   * It uses the `Groth16` setup method to generate the proving key and verifying key.

   * A `proving_circuit` instance is created with the actual `preimage` and `hash`.

   * The `Groth16` prove method is used to generate a proof.

   * The verification key is prepared, and the hash value is split into 4 groups of 8 bytes each to compute a combined hash value as the public input.

   * The `Groth16` verify_with_processed_vk method is used to verify the proof, noting that the public inputs are empty.

   * The code outputs the time taken for proving and verifying, as well as the size of the proof.

5. **Modifications**:

   * The `hash` is changed from a public input to a private input.

   * The 32-byte hash value is split into 4 groups of 8 bytes each to compute a combined hash value.

   * During verification, `gamma_abc_g1` is accessed through `pvk.vk`.

