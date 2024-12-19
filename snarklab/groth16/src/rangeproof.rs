use ark_groth16::{prepare_verifying_key, Groth16};
use ark_crypto_primitives::snark::{CircuitSpecificSetupSNARK, SNARK};
use ark_ec::pairing::Pairing;
use ark_ff::Field;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_std::{
    rand::{RngCore, SeedableRng},
    test_rng, UniformRand,
};

/// Define a simple circuit that computes x^3 = y
struct CubicCircuit<F: Field> {
    x: Option<F>,
}

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for CubicCircuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // Define the variables x, x^2, x^3 (which is y)
        let x = cs.new_witness_variable(|| self.x.ok_or(SynthesisError::AssignmentMissing))?;
        let x_squared = cs.new_witness_variable(|| {
            let mut x_val = self.x.ok_or(SynthesisError::AssignmentMissing)?;
            x_val.square_in_place(); // Compute x^2
            Ok(x_val)
        })?;
        let y = cs.new_input_variable(|| {
            let mut x_val = self.x.ok_or(SynthesisError::AssignmentMissing)?;
            x_val.square_in_place(); // Compute x^2
            x_val *= &self.x.ok_or(SynthesisError::AssignmentMissing)?; // Compute x^3
            Ok(x_val) // Return y
        })?;

        // Add the constraints: x * x = x^2 and x^2 * x = x^3
        cs.enforce_constraint(lc!() + x, lc!() + x, lc!() + x_squared)?; // x * x = x^2
        cs.enforce_constraint(lc!() + x_squared, lc!() + x, lc!() + y)?; // x^2 * x = x^3

        Ok(())
    }
}

fn main() {
    // Use the BLS12-381 elliptic curve
    test_prove_and_verify::<ark_bls12_381::Bls12_381>();
}

/// Prove and verify function
fn test_prove_and_verify<E>()
where
    E: Pairing,
{
    // Create a random number generator
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());

    // Perform the Groth16 setup (generate the proving and verifying keys)
    let (pk, vk) = Groth16::<E>::setup(CubicCircuit { x: None }, &mut rng).unwrap();
    let pvk = prepare_verifying_key::<E>(&vk);

    // Generate a random x
    let x = E::ScalarField::rand(&mut rng);
    let mut y = x;
    y.square_in_place(); // Compute x^2
    y *= x; // Compute x^3

    // Prove that x^3 = y using Groth16
    let proof = Groth16::<E>::prove(
        &pk,
        CubicCircuit { x: Some(x) },
        &mut rng,
    )
    .unwrap();

    // Verify the proof
    let is_valid = Groth16::<E>::verify_with_processed_vk(&pvk, &[y], &proof).unwrap();
    println!("Proof is valid: {}", is_valid); // Print the proof result
}
