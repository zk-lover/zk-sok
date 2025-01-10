## Sample programs using the delak library

We have implemented the example program mentioned in our paper based on the delak library, including:

* Range Proof

The program source code can be found under

> delak/src/

This tutorial will provide an introduction to our example program codes to demonstrate the use of delak.

Blow, we will introduce the implementation logic of our example programs based on the delak library.&#x20;

**You can find detailed annotations of the program code in the markdown files within this directory.**

### Range Proof

This code implements a range proof using the Bulletproofs library. A range proof is a cryptographic technique that allows one to prove that a secret value lies within a certain range without revealing the value itself. Here's the implementation logic of the code:

1. **Initialize Random Number Generator**:

   * The code uses `OsRng` as a random number generator to ensure high-security random number generation.

2. **Generate Base Generators for Pedersen Commitment**:

   * It uses `PedersenGens::default()` to generate base generators for Pedersen commitments. Pedersen commitments are a cryptographic commitment scheme that allows verification without revealing the secret value.

3. **Create BulletproofGens**:

   * The code creates `BulletproofGens` with `BulletproofGens::new(32, 1)`, specifying the maximum number of proofs and the range (2^32). This defines the maximum range that can be proven.

4. **Choose a Value for Range Proof**:

   * A value between 0 and 2^32-1 (e.g., `1234567890`) is chosen as the secret value to be proven.

5. **Generate a Random Blinding Factor**:

   * A random blinding factor is generated using `Scalar::random(&mut rng)`, which is used to hide the secret value.

6. **Generate the Range Proof**:

   * The `RangeProof::prove_single` method is used to generate the range proof. This method requires BulletproofGens, PedersenGens, a transcript object, the value, the blinding factor, and the bit size (32 bits).

7. **Calculate Proof Size**:

   * The size of the generated proof is calculated using `std::mem::size_of_val(&proof)`.

8. **Verify the Range Proof**:

   * The `proof.verify_single` method is used to verify the generated range proof. If verification is successful, it outputs "Proof verification successful!"; otherwise, it outputs an error message.

9. **Output Proof and Verification Time**:

   * The code uses `Instant::now()` and `elapsed()` methods to calculate and output the time taken to generate and verify the proof.

