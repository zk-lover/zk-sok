#![allow(non_snake_case)]
extern crate curve25519_dalek;
extern crate libspartan;
extern crate merlin;
use curve25519_dalek::scalar::Scalar;
use libspartan::{InputsAssignment, Instance, SNARKGens, VarsAssignment, SNARK};
use merlin::Transcript;
use std::time::Instant;

fn main() {
    let (
        num_cons,
        num_vars,
        num_inputs,
        num_non_zero_entries,
        inst,
        assignment_vars,
        assignment_inputs,
    ) = produce_rangeproof_r1cs();

    // Generate public parameters
    let gens = SNARKGens::new(num_cons, num_vars, num_inputs, num_non_zero_entries);

    // Create commitment to R1CS instance
    let (comm, decomm) = SNARK::encode(&inst, &gens);

    // Generate proof
    let mut prover_transcript = Transcript::new(b"rangeproof_example");
    let start1 = Instant::now();
    let proof = SNARK::prove(
        &inst,
        &comm,
        &decomm,
        assignment_vars,
        &assignment_inputs,
        &gens,
        &mut prover_transcript,
    );
    let start2 = Instant::now();
    
    // Serialize proof to get size
    let serialized_proof = bincode::serialize(&proof).expect("Serialization failed");
    println!("Proof size: {} bytes", serialized_proof.len());

    // Verify proof
    let mut verifier_transcript = Transcript::new(b"rangeproof_example");
    assert!(proof
        .verify(&comm, &assignment_inputs, &mut verifier_transcript, &gens)
        .is_ok());
    let start3 = Instant::now();

    println!("Prove time: {:.3} ms", start2.duration_since(start1).as_secs_f64() * 1000.0);
    println!("Verify time: {:.3} ms", start3.duration_since(start2).as_secs_f64() * 1000.0);
    println!("Proof verification successful!");
}

fn produce_rangeproof_r1cs() -> (
    usize,
    usize,
    usize,
    usize,
    Instance,
    VarsAssignment,
    InputsAssignment,
) {
    let num_vars = 32;
    let num_cons = 33;
    let num_inputs = 1;
    // The libspartan library may internally optimize non-zero entries in sparse matrices, actual non-zero entries may be less than theoretical calculation
    let num_non_zero_entries = 64;

    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    let one = Scalar::ONE.to_bytes();

    // 1. Binary constraint: bi * bi = bi
    for i in 0..num_vars {
        A.push((i, i, one));
        B.push((i, i, one));
        C.push((i, i, one));
    }

    // 2. Sum constraint: sum(bi * 2^i) = x
    let mut coeff = Scalar::ONE;
    for i in 0..num_vars {
        A.push((num_vars, i, coeff.to_bytes()));
        coeff = coeff + coeff;
    }
    B.push((num_vars, num_vars, one));
    C.push((num_vars, num_vars + 1, one));

    let x = Scalar::from(1234u32);
    
    // Decompose x into binary bits
    let mut vars = vec![Scalar::ZERO.to_bytes(); num_vars];
    let x_bytes = x.to_bytes();
    
    for i in 0..num_vars {
        let byte_idx = i / 8;
        let bit_idx = i % 8;
        let bit = (x_bytes[byte_idx] >> bit_idx) & 1;
        if bit == 1 {
            vars[i] = Scalar::ONE.to_bytes();
        }
    }

    // Create VarsAssignment and InputsAssignment
    let assignment_vars = VarsAssignment::new(&vars).unwrap();
    let mut inputs = vec![Scalar::ZERO.to_bytes(); num_inputs];
    inputs[0] = x.to_bytes();
    let assignment_inputs = InputsAssignment::new(&inputs).unwrap();

    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();
    assert!(inst.is_sat(&assignment_vars, &assignment_inputs).unwrap());

    (
        num_cons,
        num_vars,
        num_inputs,
        num_non_zero_entries,
        inst,
        assignment_vars,
        assignment_inputs,
    )
}
