## Sample programs using the gnark library

We have implemented the three example programs mentioned in our paper based on gnark, including:

* A Cubic Expression

* Range Proof

* SHA256

The gnark library supports both Groth16 and Plonk schemes, so we implemented our example programs using both schemes.&#x20;

The source code for the programs is located in the `test.go` files within six files under the `gnarklab` folder.

For the "A Cubic Expression" example program we implemented, the source code based on the Groth16 scheme is located at:

> gnarklab\testCubitequation\test.go

The source code based on the Plonk scheme is located at:

> gnarklab\testCubquation_plonk\test.go

The same applies to the other example programs.

This tutorial will provide an overview of our example program code to demonstrate the use of gnark.

Blow, we will introduce the implementation logic of each example programs based on the gnark library.&#x20;

**You can find detailed annotations of the program codes in the markdown files within this directory.**

##### 1. A Cubic Expression Implemented Based on Groth16

This code snippet demonstrates a simple zero-knowledge proof (zkSNARK) example using the `gnark` library based on Groth16. Here's a breakdown of the implementation logic:

1. **Circuit Definition**:

   * The `CubicCircuit` struct defines a simple circuit representing the equation `x^3 + x + 1 = y`. Here, `X` is a secret variable, and `Y` is a public variable.

   * The `Define` method specifies the circuit constraints using the `gnark` API to represent the equation.

2. **Start HTTP Server**:

   * In the `main` function, an HTTP server is started for performance profiling (`pprof`), listening on `0.0.0.0:6060`.

3. **Compile Circuit**:

   * The circuit is compiled into an R1CS (Rank-1 Constraint System) using `frontend.Compile`, which is a standard representation for zkSNARKs.

4. **Setup**:

   * The `groth16.Setup` function generates the proving key (`pk`) and verifying key (`vk`).

5. **Encode Keys**:

   * The proving and verifying keys are serialized using the `gob` encoder, and their sizes are printed.

6. **Witness Definition**:

   * A `CubicCircuit` instance is created as a witness with `X=3` and `Y=31`. The witness and public witness are generated using `frontend.NewWitness`.

7. **Prove and Verify**:

   * A proof is generated using `groth16.Prove`, serialized with the `gob` encoder, and its size is printed.

   * Finally, the proof is verified using `groth16.Verify`.

##### 2. A Cubic Expression Implemented Based on Plonk

This code is implemented using PLONK. Here's the implementation logic:

1. **Circuit Definition**:

   * The `CubicCircuit` struct defines a simple circuit with two variables, `X` and `Y`. `X` is a secret variable, while `Y` is a public variable.

   * The `Define` method sets the circuit's constraint: `x**3 + x + 1 == y`. This means the circuit's output `Y` should equal the cube of input `X` plus `X` plus 1.

2. **Circuit Compilation**:

   * In the `main` function, the `CubicCircuit` is compiled into a sparse R1CS (Rank-1 Constraint System), which is the foundation for zero-knowledge proofs.

3. **KZG Data Creation**:

   * The `unsafekzg.NewSRS` function is used to create the data needed for the KZG (Kate-Zaverucha-Goldberg) commitment scheme. This data is used for polynomial commitments, ensuring the circuit's constraints are verified without revealing secret information.

4. **Setup and Proof**:

   * The `plonk.Setup` function generates the proving key (`pk`) and the verification key (`vk`).

   * An instance of `CubicCircuit` is defined as a witness, with `X` set to 3 and `Y` set to 31.

   * The `plonk.Prove` function generates a proof.

   * The `plonk.Verify` function checks if the proof is correct.

5. **Encoding and Output**:

   * The `gob` encoder is used to encode the KZG data and the proof into byte streams, and their sizes are printed.

##### 3. Range Proof Implemented Based on Groth16

This code implements a range proof using zero-knowledge proof (zkSNARK) to verify that a variable is within a specified bit range. It is implemented using Groth16. Here's a detailed breakdown of the implementation logic:

1. **Define the Circuit**:

   * The `Circuit` struct defines a circuit with a `frontend.Variable` type `Vals` and an integer `bits`.

   * The `Define` method sets up the circuit's constraints using the `rangecheck` library. The method `r.Check(c.Vals, c.bits)` ensures that `Vals` is within the range specified by `bits`.

2. **Main Function**:

   * **Compile the Circuit**:

     * A `Circuit` instance is created with `bits` set to 32.

     * The circuit is compiled using `frontend.Compile`, generating a constraint system (ccs) for subsequent zkSNARK setup and proof generation.

   * **Groth16 zkSNARK Setup**:

     * The `groth16.Setup` function generates the proving key (pk) and verification key (vk).

     * The sizes of pk and vk are calculated and printed using a `gob` encoder to understand the storage requirements of the keys.

   * **Create and Assign Witness**:

     * A `Circuit` instance is created for assignment, with `Vals` set to a specific value (665115184).

     * A witness is created using `frontend.NewWitness`, which includes the specific value to be verified.

     * The public witness is extracted for the verification process.

   * **Generate Proof**:

     * A proof is generated using `groth16.Prove`, ensuring that the input value meets the range constraints defined by the circuit.

     * The size of the proof is calculated and printed using a `gob` encoder.

   * **Verify the Proof**:

     * The proof is verified using `groth16.Verify` to ensure its correctness.

     * If verification is successful, "Proof verified successfully" is printed, indicating that the input value is within the specified range.

##### 4. Range Proof Implemented Based on Plonk

This code implements a range proof using zero-knowledge proof (zkSNARK) to verify that a variable is within a specified bit range. It is implemented using Plonk. Here's a detailed breakdown of the implementation logic:

1. **Circuit Definition**:

   * The `Circuit` struct is defined with a `frontend.Variable` type `Vals` and an integer `bits`.

   * The `Define` method sets up the circuit's constraints using the `rangecheck` library. The method `r.Check(c.Vals, c.bits)` ensures that `Vals` is within the range specified by `bits`.

2. **Main Function**:

   * **Compile the Circuit**:

     * A `Circuit` instance is created with `bits` set to 32.

     * The circuit is compiled using `frontend.Compile`, generating a constraint system (ccs) for subsequent zkSNARK setup and proof generation.

   * **Setup zkSNARK**:

     * The `plonk.Setup` function is used to generate the proving key (pk) and verification key (vk).

     * The sizes of the structured reference string (SRS) and its Lagrange form are calculated and printed using a `gob` encoder to understand the storage requirements.

   * **Create and Assign Witness**:

     * A `Circuit` instance is created for assignment, with `Vals` set to a specific value (665115184).

     * A witness is created using `frontend.NewWitness`, which includes the specific value to be verified.

     * The public witness is extracted for the verification process.

   * **Generate Proof**:

     * A proof is generated using `plonk.Prove`, ensuring that the input value meets the range constraints defined by the circuit.

     * The size of the proof is calculated and printed using a `gob` encoder.

   * **Verify the Proof**:

     * The proof is verified using `plonk.Verify` to ensure its correctness.

     * If verification is successful, "Proof verified successfully" is printed, indicating that the input value is within the specified range.

##### 5. SHA256 Implemented Based on Groth16

This code implements a zero-knowledge proof (zkSNARK) process to verify that the hash of input data matches an expected value. It is implemented using Groth16. Here's a detailed breakdown of the implementation logic:

1. **Circuit Definition**:

   * The `Circuit` struct defines a circuit with an input array `In` and an expected hash value `Expected`.

   * In the `Define` method, a SHA-256 hash function instance is created using `sha2.New(api)`.

   * The input data `In` is written to the hash function, and the hash value `res` is computed.

   * The computed hash `res` is then checked against the `Expected` value to ensure they match.

2. **Main Function**:

   * **Step 1: Compile the Circuit**:

     * The input string is converted to a byte array, and its SHA-256 hash is calculated.

     * A `Circuit` instance is created, and the circuit is compiled using `frontend.Compile`, generating a constraint system (ccs).

   * **Step 2: Groth16 zkSNARK Setup**:

     * The `groth16.Setup` function generates the proving key (pk) and verification key (vk).

     * The sizes of pk and vk are calculated and printed using a `gob` encoder.

   * **Steps 3-5: Create and Assign Witness**:

     * A `Circuit` instance is created for assignment, with the input byte array and calculated hash assigned to `In` and `Expected`.

     * A witness is created using `frontend.NewWitness`, and the public witness is extracted.

   * **Step 7: Generate Proof**:

     * A proof is generated using `groth16.Prove`.

     * The size of the proof is calculated and printed using a `gob` encoder.

   * **Step 8: Verify the Proof**:

     * The proof is verified using `groth16.Verify`.

     * If verification is successful, "Proof verified successfully" is printed.

##### 6. SHA256 Implemented Based on Plonk

This code implements a zero-knowledge proof (zkSNARK) process to verify that the SHA-256 hash of input data matches an expected value. It is implemented using Plonk. Here's a detailed breakdown of the implementation logic:

1. **Circuit Definition**:

   * The `Circuit` struct defines a circuit with an input array `In` and an expected hash value `Expected`.

   * In the `Define` method, a SHA-256 hash function instance is created using `sha2.New(api)`.

   * The input data `In` is written to the hash function, and the hash value `res` is computed.

   * The computed hash `res` is checked against the `Expected` value using the `uapi.ByteAssertEq` method to compare each byte.

2. **Main Function**:

   * **Compile the Circuit**:

     * The input string is converted to a byte array, and its SHA-256 hash is calculated.

     * A `Circuit` instance is created, and the circuit is compiled using `frontend.Compile`, generating a constraint system (ccs).

   * **Setup zkSNARK**:

     * The `plonk.Setup` function generates the proving key (pk) and verification key (vk).

     * The sizes of the structured reference string (SRS) and its Lagrange form are calculated and printed using a `gob` encoder.

   * **Create and Assign Witness**:

     * A `Circuit` instance is created for assignment, with the input byte array and calculated hash assigned to `In` and `Expected`.

     * A witness is created using `frontend.NewWitness`, and the public witness is extracted.

   * **Step 7: Generate Proof**:

     * A proof is generated using `plonk.Prove`.

     * The size of the proof is calculated and printed using a `gob` encoder.

   * **Verify the Proof**:

     * The proof is verified using `plonk.Verify`.

     * If verification is successful, "Proof verified successfully" is printed.

