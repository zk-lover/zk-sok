# `arkworks` Gadget Documentation

`arkworks` provides a variety of commonly used gadgets that developers can use directly to build circuits for specific applications，mainly included in the `ark_crypto_primitives` crate. They cover a variety of functions, such as cryptographic primitives, hash functions, signature algorithm polynomial operations, etc. In addition, `arkworks` also provides a variety of elliptic curve implementations, including BLS12, BN, MNT curves, etc. The following introduces some specific gadgets and provides examples.

## 1.CommGadget (Pedersen Commitment Gadget)

CommGadget is an implementation of the Pedersen commitment constraint circuit. It allows verification of Pedersen commitments in zero-knowledge proof systems. CommGadget generates commitment values by combining inputs, randomness, and precomputed parameters. It implements the `CommitmentGadget` trait to convert the commitment algorithm into circuit constraints.

### Example Code:
```rust
let cs = ConstraintSystem::<Fq>::new_ref();
let input_var: Vec<_> = [1u8; 4].iter()
    .map(|&byte| UInt8::new_witness(cs.clone(), || Ok(byte)).unwrap())
    .collect();
let randomness_var = CommGadget::<JubJub, EdwardsVar, Window>::RandomnessVar::new_witness(
    ark_relations::ns!(cs, "randomness"),
    || Ok(&Randomness(Fr::rand(&mut test_rng()))),
).unwrap();
let params_var = CommGadget::<JubJub, EdwardsVar, Window>::ParametersVar::new_witness(
    ark_relations::ns!(cs, "parameters"),
    || Ok(&Commitment::<JubJub, Window>::setup(&mut test_rng()).unwrap()),
).unwrap();
let result_var = CommGadget::<JubJub, EdwardsVar, Window>::commit(
    &params_var, &input_var, &randomness_var
).unwrap();
```
## 2.Sha256Gadget

Sha256Gadget is a circuit constraint implementation of the SHA-256 hash function. It allows verification of SHA-256 hash computations in zero-knowledge proofs.

### Core Functions:
- `update`: Updates the hash state with input data.
- `finalize`: Computes and returns the final hash value.
- `digest`: Computes the hash directly from input data, simplifying the call process.

### Example Code:
```rust
let cs = ConstraintSystem::<Fr>::new_ref();
let input = vec![UInt8::constant(0x61), UInt8::constant(0x62), UInt8::constant(0x63)]; // "abc"
let hash = Sha256Gadget::<Fr>::digest(&input).unwrap(); // Compute hash
assert_eq!(
    hash.value().unwrap().to_vec(),
    hex::decode("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").unwrap()
); // Verify result
```
# 3.ElGamalEncGadget

ElGamalEncGadget is a circuit constraint implementation of the ElGamal encryption scheme. It allows verification of ElGamal encryption operations in zero-knowledge proofs.

### Core Functions:
- `encrypt`: Implements the circuit constraints for ElGamal encryption, including generating ciphertext (c1, c2).

### Example Code:
```rust
let rng = &mut test_rng();
let cs = ConstraintSystem::<Fq>::new_ref();
let params = ElGamal::<JubJub>::setup(rng).unwrap();
let (pk, _) = ElGamal::<JubJub>::keygen(&params, rng).unwrap();
let msg = JubJub::rand(rng).into();
let rand = Randomness::rand(rng);

let params_var = ElGamalEncGadget::<JubJub, EdwardsVar>::ParametersVar::new_constant(cs.clone(), &params).unwrap();
let pk_var = ElGamalEncGadget::<JubJub, EdwardsVar>::PublicKeyVar::new_witness(cs.clone(), || Ok(&pk)).unwrap();
let msg_var = ElGamalEncGadget::<JubJub, EdwardsVar>::PlaintextVar::new_witness(cs.clone(), || Ok(&msg)).unwrap();
let rand_var = ElGamalEncGadget::<JubJub, EdwardsVar>::RandomnessVar::new_witness(cs.clone(), || Ok(&rand)).unwrap();

let ciphertext_var = ElGamalEncGadget::<JubJub, EdwardsVar>::encrypt(&params_var, &msg_var, &rand_var, &pk_var).unwrap();
```
# 4.PathVar

PathVar is a circuit constraint gadget for Merkle tree path verification and updates. It allows verification of whether a leaf node belongs to a given Merkle tree in zero-knowledge proofs and supports updating leaf nodes.

### Core Functions:
1. `calculate_root`: Computes the Merkle tree root from a leaf node and its path.
2. `verify_membership`: Verifies if a given leaf node belongs to the Merkle tree.
3. `update_leaf`: Updates a leaf node in the Merkle tree and computes the new root.
4. `update_and_check`: Verifies if the updated tree root matches the expected value.

### Example Code:
```rust
let cs = ConstraintSystem::<Fq>::new_ref();
let leaf = UInt8::constant(42); // Example leaf
let root = UInt8::constant(123); // Example root
let path_var = PathVar::new_witness(cs.clone(), || Ok(example_merkle_path())).unwrap();

let leaf_params = ...; // Leaf hash parameters
let two_to_one_params = ...; // Two-to-one hash parameters

// Verify if the leaf is in the tree
let is_valid = path_var.verify_membership(&leaf_params, &two_to_one_params, &root, &leaf).unwrap();
assert!(is_valid.value().unwrap());
```
# 5.PRFGadget

PRFGadget is a circuit constraint implementation of a pseudorandom function (PRF). It allows verification of PRF computations in zero-knowledge proofs, constraining the PRF logic within the circuit.

### Core Functions:
1. `new_seed`: Converts the PRF seed into circuit variables (`UInt8`).
2. `evaluate`: Computes the PRF output within the circuit.

### Example Code:
```rust
let cs = ConstraintSystem::<F>::new_ref();
let seed = vec![UInt8::constant(0x12), UInt8::constant(0x34)]; // Example seed
let input = vec![UInt8::constant(0x56), UInt8::constant(0x78)]; // Example input

type MyPRFGadget = ...; // Specific PRFGadget implementation

// Compute PRF output
let output_var = MyPRFGadget::evaluate(&seed, &input).unwrap();
```
# 6.SchnorrRandomizePkGadget

SchnorrRandomizePkGadget is a circuit constraint implementation for public key randomization in Schnorr signatures. It allows verification of the correctness of Schnorr public key randomization operations in zero-knowledge proofs.

### Core Functions:
1. `randomize`:
   - Randomizes the public key using randomness.
   - This method adds constraints to the circuit to verify the correctness of the randomization operation.
2. `PublicKeyVar` and `ParametersVar`:
   - `PublicKeyVar`: Represents the constraint variable for the Schnorr public key in the circuit.
   - `ParametersVar`: Represents the constraint variable for Schnorr signature parameters (e.g., generators) in the circuit.

### Example Code:
```rust
let cs = ConstraintSystem::<Fq>::new_ref();
let randomness = vec![UInt8::constant(1), UInt8::constant(2)]; // Example randomness
let parameters_var = ParametersVar::new_constant(cs.clone(), &example_parameters).unwrap();
let public_key_var = PublicKeyVar::new_witness(cs.clone(), || Ok(example_public_key)).unwrap();

// Randomize public key
let randomized_pk = SchnorrRandomizePkGadget::randomize(&parameters_var, &public_key_var, &randomness).unwrap();
```