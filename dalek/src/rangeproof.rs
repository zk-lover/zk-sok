extern crate bulletproofs;
extern crate curve25519_dalek;
extern crate rand;

use bulletproofs::{
    BulletproofGens, RangeProof, PedersenGens,
};
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use merlin::Transcript;
use std::time::Instant;

fn main() {
    // 1. Initialize random number generator
    let mut rng = OsRng;

    // 2. Generate base generators for Pedersen commitment
    let pedersen_gens = PedersenGens::default();

    // 3. Create BulletproofGens, specify maximum number of proofs
    let bulletproof_gens = BulletproofGens::new(32, 1);  // 2^32 range

    // 4. Choose a value for range proof (e.g. a value between 0 and 2^32-1)
    let value: u32 = 1234567890;  // Make sure this value is between 0 and 2^32-1
    
    // 5. Create random number for blinding factor
    let blinding = Scalar::random(&mut rng);

    println!("Creating proof for value: {} (range: 0 to {})", value, u32::MAX);

    // Generate proof
    let proving_time = Instant::now();
    let mut prover_transcript = Transcript::new(b"range_proof");
    let (proof, committed_value) = RangeProof::prove_single(
        &bulletproof_gens,
        &pedersen_gens,
        &mut prover_transcript,
        value as u64,  // Convert to u64
        &blinding,     // Blinding factor
        32,  // Change bit size to 32
    ).expect("Proof generation failed");
    let proving_duration = proving_time.elapsed();

    // Calculate proof size
    let proof_size = std::mem::size_of_val(&proof);
    println!("Proving time: {:?}", proving_duration);
    println!("Proof size: {} bytes", proof_size);

    // Verify proof
    let verifying_time = Instant::now();
    let mut verifier_transcript = Transcript::new(b"range_proof");
    let result = proof.verify_single(
        &bulletproof_gens,
        &pedersen_gens,
        &mut verifier_transcript,
        &committed_value,
        32,  // Change bit size to 32
    );
    let verifying_duration = verifying_time.elapsed();
    println!("Verification time: {:?}", verifying_duration);

    match result {
        Ok(_) => println!("Proof verification successful!"),
        Err(e) => println!("Proof verification failed: {:?}", e),
    }
}
