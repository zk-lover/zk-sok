#![warn(unused)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    variant_size_differences,
    stable_features,
    non_shorthand_field_patterns,
    renamed_and_removed_lints,
    unsafe_code
)]

use ark_crypto_primitives::snark::{CircuitSpecificSetupSNARK, SNARK};
// For randomness (during paramgen and proof generation)
use ark_std::rand::{Rng, RngCore, SeedableRng};

// For benchmarking
use std::time::{Duration, Instant};

// Bring in some tools for using pairing-friendly curves
// We're going to use the BLS12-377 pairing-friendly elliptic curve.
use ark_bls12_377::{Bls12_377, Fr};
use ark_ff::Field;
use ark_std::test_rng;

// We'll use these interfaces to construct our circuit.
use ark_relations::{
    lc, ns,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};

const MIMC_ROUNDS: usize = 322;
``
fn cube_plus_one<F: Field>(x: F) -> F {
    // 计算x的立方
    let x_cubed = x.clone() * x.clone() * x;
    // 计算x的立方加上x，然后再加上1
    x_cubed + x + F::one()
}


struct CubePlusOneDemo<F: Field> {
    x: Option<F>,
}

/// Our demo circuit implements this `Circuit` trait which
/// is used during paramgen and proving in order to
/// synthesize the constraint system.
impl<F: Field> ConstraintSynthesizer<F> for CubePlusOneDemo<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Allocate the witness variable for x
        let x = cs.new_witness_variable(|| self.x.ok_or(SynthesisError::AssignmentMissing))?;

        // Allocate the public input for x^3 + x + 1
        let out = cs.new_input_variable(|| {
            self.x.map(|x| x.clone() * x.clone() * x + x + F::one())
                .ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Enforce x^3 constraint
        let x_cubed = cs.new_witness_variable(|| {
            self.x.map(|x| x.clone() * x.clone() * x)
                .ok_or(SynthesisError::AssignmentMissing)
        })?;
        cs.enforce_constraint(lc!() + x, lc!() + x, lc!() + x_cubed)?;

        // Enforce x^3 + x constraint
        let x_cubed_plus_x = cs.new_witness_variable(|| {
            self.x.map(|x| x.clone() * x.clone() * x + x)
                .ok_or(SynthesisError::AssignmentMissing)
        })?;
        cs.enforce_constraint(lc!() + x_cubed, lc!() + Variable::One, lc!() + x_cubed_plus_x)?;

        // Enforce x^3 + x + 1 = out constraint
        cs.enforce_constraint(lc!() + x_cubed_plus_x + (F::one(), Variable::One), lc!(), lc!() + out);

        Ok(())
    }
}

#[test]
fn test_cube_plus_one_groth16() {
    let mut rng = test_rng();

    // Create parameters for our circuit
    let (pk, vk) = {
        let c = CubePlusOneDemo::<Fr> { x: None };
        Groth16::<Bls12_377>::setup(c, &mut rng).unwrap()
    };

    // Generate a random x and compute x^3 + x + 1
    let x = Fr::rand(&mut rng);
    let out = x * x * x + x + Fr::one();

    // Create an instance of our circuit (with the witness)
    let c = CubePlusOneDemo { x: Some(x) };

    // Create a groth16 proof with our parameters.
    let proof = Groth16::<Bls12_377>::prove(&pk, c, &mut rng).unwrap();

    // Prepare the verification key (for proof verification)
    let pvk = Groth16::<Bls12_377>::process_vk(&vk).unwrap();

    // Check the proof
    assert!(Groth16::<Bls12_377>::verify_with_processed_vk(&pvk, &[out], &proof).unwrap());
    println!("Proof verified successfully!");
}
