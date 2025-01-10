## Sample programs using the spartan library

We have implemented the three example programs mentioned in our paper based on the spartan library, including:

* A Cubic Expression

* Range Proof

* SHA256

The program source code can be found under

> spartan/src/

This tutorial will provide an introduction to our example program codes to demonstrate the use of spartan.

Blow, we will introduce the implementation logic of each example programs based on the spartan library.

**You can find detailed annotations of the program codes in the markdown files within this directory.**

### 1. A Cubic Expression

This code demonstrates the implementation of a Rank-1 Constraint System (R1CS) proof using the `libspartan` library for the cubic equation `x^3 + x + 5 = y`. R1CS is a constraint system used in zero-knowledge proofs, particularly in SNARKs (Succinct Non-interactive Arguments of Knowledge). Here's a breakdown of the code's logic:

1. **R1CS Constraint Definition**:

   * The code defines four constraints:

     * `Z0 * Z0 - Z1 = 0`

     * `Z1 * Z0 - Z2 = 0`

     * `(Z2 + Z0) * 1 - Z3 = 0`

     * `(Z3 + 5) * 1 - I0 = 0`

   * These constraints correspond to the step-by-step computation of the equation `x^3 + x + 5 = y`.

2. **R1CS Instance Creation**:

   * Three sparse matrices `A`, `B`, and `C` are used to represent the constraints.

   * Each matrix element is a tuple `(row, column, value)`, indicating the value at a specific position in the matrix.

3. **Generating Satisfying Variables and Inputs**:

   * A random scalar `z0` is generated, and then `z1`, `z2`, `z3`, and `i0` are computed sequentially to satisfy the defined constraints.

   * These values are converted to byte arrays and used to create `VarsAssignment` and `InputsAssignment`.

4. **Checking R1CS Instance Satisfiability**:

   * The method `inst.is_sat` is used to verify if the generated variables and inputs satisfy the R1CS instance.

5. **SNARK Proof Generation and Verification**:

   * Public parameters `gens` are generated.

   * A commitment to the R1CS instance is created using `SNARK::encode`.

   * A proof of satisfiability `proof` is generated.

   * The proof is serialized, and its size is printed.

   * The proof's correctness is verified to ensure it satisfies all constraints.

6. **Performance Measurement**:

   * The time taken to generate and verify the proof is recorded and printed in milliseconds.

### 2. Range Proof

This code implements a zero-knowledge proof system using the `libspartan` library to demonstrate that a scalar `x` can be decomposed into a 32-bit binary number and satisfies specific R1CS (Rank-1 Constraint System) constraints. Here's a breakdown of the implementation logic:

1. **Generate R1CS Instance and Variable Assignment**:

   * The function `produce_rangeproof_r1cs` generates an R1CS instance, including the number of constraints, variables, inputs, and non-zero entries.

   * It also generates assignments for variables and inputs, where the variable assignment decomposes a scalar `x` into its binary bits.

2. **Generate Public Parameters**:

   * Public parameters are generated using the `SNARKGens::new` method, which are used in the proving and verification processes.

3. **Create Commitment to R1CS Instance**:

   * The R1CS instance is encoded using `SNARK::encode`, producing a commitment and decommitment information.

4. **Generate Proof**:

   * A proof is generated using the `SNARK::prove` method. The proving process utilizes a `Transcript` object to record information during the proof generation.

5. **Serialize Proof**:

   * The proof is serialized using `bincode::serialize` to calculate its size.

6. **Verify Proof**:

   * The proof's correctness is verified using the `proof.verify` method, which also uses a `Transcript` object.

7. **Output Results**:

   * The code prints the proof size, the time taken to generate the proof, and the time taken to verify the proof.

### 3. SHA256

This code proves the satisfiability of a synthetic R1CS (Rank-1 Constraint System) instance and verifies the correctness of that proof. Here's the implementation logic:

1. **Import Libraries**:

   * `libspartan`: Used for handling SNARK (Succinct Non-interactive Arguments of Knowledge) related operations.

   * `merlin`: Used for creating and managing transcript objects, which ensure the security of interactions.

   * `std::time::Instant`: Used for measuring the time taken by operations.

2. **Define R1CS Instance Size**:

   * `num_vars`, `num_cons`, `num_inputs`, `num_non_zero_entries`: These variables define the size and complexity of the R1CS. R1CS is a standard form used in SNARKs to represent constraints.

3. **Generate Public Parameters**:

   * `SNARKGens::new`: Generates public parameters based on the given number of constraints, variables, inputs, and non-zero entries.

4. **Generate Synthetic R1CS Instance**:

   * `Instance::produce_synthetic_r1cs`: Generates a synthetic R1CS instance, including variables and inputs.

5. **Create Commitment to the R1CS Instance**:

   * `SNARK::encode`: Encodes the R1CS instance to generate a commitment and decommitment information.

6. **Generate Proof of Satisfiability**:

   * `Transcript::new`: Creates a new transcript object to record the proving process.

   * `SNARK::prove`: Generates a proof of satisfiability and records the time taken.

7. **Serialize the Proof**:

   * `bincode::serialize`: Serializes the generated proof into a byte array for storage or transmission.

8. **Verify the Proof**:

   * `Transcript::new`: Creates a new transcript object for the verification process.

   * `proof.verify`: Verifies the correctness of the generated proof and records the time taken.

9. **Output Results**:

   * Prints the size of the serialized proof.

   * Prints the time taken to generate and verify the proof.

   * Confirms successful proof verification.

