## Sample programs using the halo2 library

We have implemented the three example programs mentioned in our paper based on the halo2 library, including:

* A Cubic Expression

* Range Proof

* SHA256

The program source code can be found under

> halo2/src/

This tutorial will provide an introduction to our example program codes to demonstrate the use of halo2.

Blow, we will introduce the implementation logic of each example programs based on the halo2 library.&#x20;

**You can find detailed annotations of the program codes in the markdown files within this directory.**

### 1. A Cubic Expression

This code implements a simple algebraic circuit using the Halo2 framework to compute and verify the expression `y = x^3 + x + 5`. Here's a breakdown of the implementation logic:

##### Memory Allocator

The code defines a `CountingAllocator` to track the total amount of memory allocated and deallocated. By implementing the `GlobalAlloc` trait, `CountingAllocator` can replace the default system allocator. `CountingAllocatorWrapper` is a wrapper used to apply `CountingAllocator` globally.

##### Circuit Structure

The code defines several structs and traits to implement different parts of the circuit:

* **`Number<F>`**: Represents a numerical variable.

* **`FieldInstructions`, `AddInstructions`, `MulInstructions`**: Define the basic operation interfaces in the circuit, including loading constants, private inputs, addition, and multiplication.

* **`FieldConfig`, `AddConfig`, `MulConfig`**: Define the column and selector configurations used in the circuit.

* **`FieldChip`, `AddChip`, `MulChip`**: Implement the specific logic for circuit operations.

##### Circuit Implementation

* **`FieldChip`**: Implements the `FieldInstructions` trait, providing functionalities to load constants, private inputs, and compute `y = x^3 + x + 5`.

* **`AddChip` and `MulChip`**: Implement the specific logic for addition and multiplication, respectively.

##### Circuit Synthesis

* **`MyCircuit`**: Defines the overall structure of the circuit, storing the private input `x`.

* **`Circuit` Trait Implementation**: Defines the configuration and synthesis process of the circuit. In the `synthesize` method, the circuit loads the private input `x`, computes `y`, and exposes `y` as a public input.

##### Main Function

* Uses `MockProver` to simulate proving and verifying the circuit.

* Generates a random `x` and computes the corresponding `y`.

* Runs the proof and verifies the result, outputting the time taken for proving and verifying, as well as memory usage.

* After successful verification, attempts to verify with an incorrect public input, which should fail.

### 2. Range Proof

This code implements a range proof circuit using the Halo2 framework. Here's a breakdown of the implementation logic:

##### Library and Module Imports

The code begins by importing various modules from the Halo2 framework, including circuit, constraint system, proof creation, and verification. It also imports `rand_core` for random number generation and `std::time::Instant` for timing.

##### Circuit Structure Definition

* **`RangeProofCircuit`**: This is a simple circuit structure containing an input value `input` of type `Value<u64>`.

* **`RangeProofConfig`**: Defines the circuit configuration, including an input column `input`, 32 bit columns `bits`, and a selector `selector`.

##### Circuit Configuration and Synthesis

* **`configure` Method**: In this method, the circuit's columns and selector are initialized. Equality constraints are enabled for the input column and each bit column. A gate named "Range Proof" is created to verify that the input value can be reconstructed from the 32 bits.

* **`synthesize` Method**: This method defines the actual synthesis process of the circuit. The input value is assigned to the input column, then decomposed into 32 bits using bitwise operations, and assigned to the respective bit columns. Finally, the selector is enabled to apply the constraints.

##### Main Function

* **Parameter Setup**: Defines the circuit parameter `k` and initializes the parameters using `Params::<EqAffine>::new(k)`.

* **Circuit Instantiation**: Creates an instance of `RangeProofCircuit` and sets the input value.

* **Key Generation**: Generates the verification key and proving key using `keygen_vk` and `keygen_pk`.

* **Proof Generation**: Generates a proof using `create_proof` and transcribes it with `Blake2bWrite`.

* **Proof Verification**: Uses `SingleVerifier` as the verification strategy and verifies the generated proof with `verify_proof`.

* **Result Output**: Calculates and outputs the proving time, verification time, proof size, and verification result.

### 3. SHA256

This code implements a circuit using the Halo2 framework to compute the SHA-256 hash and generate and verify a zero-knowledge proof. Here's a breakdown of the implementation logic:

##### Library and Module Imports

The code begins by importing various modules from the Halo2 framework, including circuit, constraint system, proof creation, and verification. It also imports the SHA-256 gadget module to handle hash computation.

##### Circuit Structure Definition

* **`Sha256Circuit`**: This is a simple circuit structure containing an input vector `input` of type `Vec<u8>`. It is used to store the byte data for which the hash needs to be computed.

##### Circuit Configuration and Synthesis

* **`configure` Method**: In this method, the circuit's configuration is initialized. The SHA-256 table is configured using the `Table16Chip::configure` method.

* **`synthesize` Method**: This is the core part of the circuit, defining the synthesis process.

  * **Data Padding**: The input data is first padded to meet SHA-256 requirements. Padding includes adding a `0x80` byte, the appropriate number of `0` bytes, and appending the length of the input data.

  * **Data Blocking**: The padded data is divided into 512-bit blocks, each containing 16 32-bit words.

  * **Hash Computation**: The `Sha256` gadget is used to compute the hash. It constructs the `Table16Chip`, updates the hash state, and finally computes the hash digest.

##### Main Function

* **Parameter Setup**: Defines the circuit parameter `k` and initializes the parameters using `Params::<EqAffine>::new(k)`.

* **Circuit Instantiation**: Creates an instance of `Sha256Circuit` and sets the input message.

* **Key Generation**: Generates the verification key and proving key using `keygen_vk` and `keygen_pk`.

* **Proof Generation**: Generates a proof using `create_proof` and transcribes it with `Blake2bWrite`.

* **Proof Verification**: Uses `SingleVerifier` as the verification strategy and verifies the generated proof with `verify_proof`.

* **Result Output**: Outputs the time taken for proof generation and verification, the size of the proof, and the verification result.









