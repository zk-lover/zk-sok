extern crate libspartan;
extern crate merlin;
use std::time::Instant;

use libspartan::{Instance, SNARKGens, SNARK};
use merlin::Transcript;
fn main() {
    // specify the size of an R1CS instance
    let num_vars = 16;
    let num_cons = 32;
    let num_inputs = 1;
    let num_non_zero_entries = 32;

    // produce public parameters
    let gens = SNARKGens::new(num_cons, num_vars, num_inputs, num_non_zero_entries);

    // ask the library to produce a synthentic R1CS instance
    let (inst, vars, inputs) = Instance::produce_synthetic_r1cs(num_cons, num_vars, num_inputs);

    // create a commitment to the R1CS instance
    let (comm, decomm) = SNARK::encode(&inst, &gens);

    // produce a proof of satisfiability
    let mut prover_transcript = Transcript::new(b"snark_example");
    let start1 = Instant::now();
    let proof = SNARK::prove(&inst, &comm, &decomm, vars, &inputs, &gens, &mut prover_transcript);
    // 使用 bincode 序列化 proof
    let serialized_proof = bincode::serialize(&proof).expect("Serialization failed");

    // 输出序列化后的字节大小
    println!("Serialized proof size: {} bytes", serialized_proof.len());
    let start2 = Instant::now();
    // verify the proof of satisfiability
    let mut verifier_transcript = Transcript::new(b"snark_example");
    assert!(proof
      .verify(&comm, &inputs, &mut verifier_transcript, &gens)
      .is_ok());
    let start3 = Instant::now();
    let duration1 = start2.duration_since(start1);
    let duration2 = start3.duration_since(start2);

    let millis1 = duration1.as_secs_f64() * 1000.0;
    let millis2 = duration2.as_secs_f64() * 1000.0;
    println!("Prove time: {:.3} milliseconds", millis1);
    println!("Verify time: {:.3} milliseconds", millis2);
    println!("proof verification successful!");
}