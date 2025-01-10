## A Cubic Expression

This code uses the `arkworks` ecosystem to implement a zero-knowledge proof system with the Groth16 proving system. It defines a circuit to compute x^3 + x + 1 = y, generates constraints, and performs setup using the BLS12-381 curve. It then creates a proof for the equation, verifies its validity, and measures the performance by calculating the sizes of keys and proof, as well as the time taken for proving and verifying.

Below, we will divide the code into code blocks and annotate them.

### 1. Imports from artworks

```rust
use ark_groth16::{prepare_verifying_key, Groth16};
use ark_crypto_primitives::snark::{CircuitSpecificSetupSNARK, SNARK};
use ark_ec::pairing::Pairing;
use ark_ff::{Field, One}; // Import the One trait for field operations
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable}, // Import Variable for R1CS
};
use ark_serialize::CanonicalSerialize;
use ark_std::{
    rand::{RngCore, SeedableRng},
    test_rng, UniformRand,
};
use std::time::Instant;
```

* These imports bring in necessary modules and traits from the `arkworks` library, which is used for cryptographic operations, specifically for SNARKs (Succinct Non-interactive Arguments of Knowledge).

* **Key Imports**:

  * `Groth16`: A SNARK protocol for zero-knowledge proofs.

  * `Field` and `One`: Traits for field arithmetic.

  * `ConstraintSynthesizer`: Trait for defining R1CS (Rank-1 Constraint System) circuits.

### 2. Circuit Construction

##### Circuit Definition

```rust
/// Define a simple circuit that computes x^3 + x + 1 = y
struct CubicPlusLinearCircuit<F: Field> {
    x: Option<F>,
}
```

* **CubicPlusLinearCircuit**: This struct represents a simple arithmetic circuit. It takes a generic field element `x` as input, which is optional (`Option<F>`).

##### Implementing the Circuit

```rust
impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for CubicPlusLinearCircuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // Define variables x, x^2, x^3, and y (i.e., x^3 + x + 1)
        let x = cs.new_witness_variable(|| self.x.ok_or(SynthesisError::AssignmentMissing))?;
```

* **generate_constraints**: This function is responsible for defining the constraints of the circuit.

* **x Variable**: A witness variable for `x` is created. It represents the input to the circuit.

```rust
        let x_squared = cs.new_witness_variable(|| {
            let mut x_val = self.x.ok_or(SynthesisError::AssignmentMissing)?;
            x_val.square_in_place(); // Compute x^2
            Ok(x_val)
        })?;
```

* **x_squared Variable**: A witness variable for `x^2` is created. It computes the square of `x`.

```rust
        let x_cubed = cs.new_witness_variable(|| {
            let mut x_val = self.x.ok_or(SynthesisError::AssignmentMissing)?;
            x_val.square_in_place(); // Compute x^2
            x_val *= &self.x.ok_or(SynthesisError::AssignmentMissing)?; // Compute x^3
            Ok(x_val)
        })?;
```

* **x_cubed Variable**: A witness variable for `x^3` is created. It computes the cube of `x`.

```rust
        let y = cs.new_input_variable(|| {
            let mut x_cubed_val = self.x.ok_or(SynthesisError::AssignmentMissing)?;
            x_cubed_val.square_in_place(); // Compute x^2
            x_cubed_val *= &self.x.ok_or(SynthesisError::AssignmentMissing)?; // Compute x^3
            
            let mut result = x_cubed_val; // Start with x^3
            result += &self.x.ok_or(SynthesisError::AssignmentMissing)?; // Add x
            result += ConstraintF::one(); // Add 1
            Ok(result) // Return y
        })?;
```

* **y Variable**: An input variable for `y` is created. It computes `y = x^3 + x + 1`.

##### Adding Constraints

```rust
        // Add constraints: x * x = x^2, x^2 * x = x^3, and x^3 + x + 1 = y
        cs.enforce_constraint(lc!() + x, lc!() + x, lc!() + x_squared)?; // x * x = x^2
        cs.enforce_constraint(lc!() + x_squared, lc!() + x, lc!() + x_cubed)?; // x^2 * x = x^3
        cs.enforce_constraint(
            lc!() + x_cubed + x + (ConstraintF::one(), Variable::One), // x^3 + x + 1
            lc!() + Variable::One, // 1 (no multiplication needed)
            lc!() + y,
        )?; // y = x^3 + x + 1
        // Print the number of constraints in the circuit
        println!("Number of constraints: {}", cs.num_constraints());
        Ok(())
    }
}
```

* **Constraints**: These lines enforce the arithmetic relationships between the variables:

  * `x * x = x^2`

  * `x^2 * x = x^3`

  * `x^3 + x + 1 = y`

* **Constraint Count**: Prints the total number of constraints in the circuit.

### 3. ZK Proof Generation and Verification

```rust
pub fn main() {
    // Use BLS12-381 elliptic curve
    test_prove_and_verify::<ark_bls12_381::Bls12_381>();
}
```

* **Main Function**: Calls the `test_prove_and_verify` function using the BLS12-381 elliptic curve.

###### Prove and Verify Function

```rust
fn test_prove_and_verify<E>()
where
    E: Pairing,
{
    // Create a random number generator
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
```

* **Random Number Generator**: Initializes a seeded random number generator for reproducibility.

```rust
    // Perform Groth16 setup (generate public and verification keys)
    let (pk, vk) = Groth16::<E>::setup(CubicPlusLinearCircuit { x: None }, &mut rng).unwrap();
    
    let pvk = prepare_verifying_key::<E>(&vk);
```

* **Setup**: Generates the public key (`pk`) and verification key (`vk`) for the circuit using Groth16. The verification key is then prepared for efficient verification.

```rust
    // Compute and print the uncompressed size of pk and vk
    let pk_size = pk.uncompressed_size();
    let vk_size = vk.uncompressed_size();
    println!("Uncompressed pk size: {} bytes", pk_size);
    println!("Uncompressed vk size: {} bytes", vk_size);
    println!("Total uncompressed size (pk + vk): {} bytes", pk_size + vk_size);
```

* **Key Sizes**: Computes and prints the uncompressed sizes of the public and verification keys.

###### Generating Random Inputs

```rust
    // Generate a random x
    let x = E::ScalarField::rand(&mut rng);
    let mut y = x;
    y.square_in_place(); // Compute x^2
    let x_cubed = y * x; // Compute x^3
    y = x_cubed + x + E::ScalarField::one(); // Compute y = x^3 + x + 1
```

* **Random Inputs**: Generates a random `x` and computes `y = x^3 + x + 1`.

###### Proving and Verifying

```rust
    let start1 = Instant::now();
    // Use Groth16 to prove x^3 + x + 1 = y
    let proof = Groth16::<E>::prove(
        &pk,
        CubicPlusLinearCircuit { x: Some(x) },
        &mut rng,
    )
    .unwrap();
    let start2 = Instant::now();
```

* **Proving**: Uses Groth16 to generate a proof that `x^3 + x + 1 = y`.

```rust
    // Output the uncompressed proof size
    let uncompressed_size = proof.uncompressed_size();
    println!("Uncompressed proof size: {} bytes", uncompressed_size);
```

* **Proof Size**: Computes and prints the uncompressed size of the proof.

```rust
    // Verify the proof
    let is_valid = Groth16::<E>::verify_with_processed_vk(&pvk, &[y], &proof).unwrap();
    let start3 = Instant::now();
    println!("Proof is valid: {}", is_valid); // Print the proof result
```

* **Verification**: Verifies the proof using the prepared verification key and prints whether the proof is valid.

###### Timing

```rust
    let duration1 = start2.duration_since(start1);
    let duration2 = start3.duration_since(start2);

    let millis1 = duration1.as_secs_f64() * 1000.0;
    let millis2 = duration2.as_secs_f64() * 1000.0;
    println!("Prove time: {:.3} milliseconds", millis1);
    println!("Verify time: {:.3} milliseconds", millis2);
}
```

* **Timing**: Measures and prints the time taken for proving and verifying in milliseconds.

