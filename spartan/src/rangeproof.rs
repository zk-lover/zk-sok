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

    // 生成公共参数
    let gens = SNARKGens::new(num_cons, num_vars, num_inputs, num_non_zero_entries);

    // 创建对 R1CS 实例的承诺
    let (comm, decomm) = SNARK::encode(&inst, &gens);

    // 生成证明
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
    
    // 序列化证明以获取大小
    let serialized_proof = bincode::serialize(&proof).expect("Serialization failed");
    println!("Proof size: {} bytes", serialized_proof.len());

    // 验证证明
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
    //libspartan 库可能会对稀疏矩阵的非零项进行内部优化，实际的非零项可能少于理论计算值。
    let num_non_zero_entries = 64;

    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    let one = Scalar::ONE.to_bytes();

    // 1. 二进制约束：bi * bi = bi
    for i in 0..num_vars {
        A.push((i, i, one));
        B.push((i, i, one));
        C.push((i, i, one));
    }

    // 2. 和约束：sum(bi * 2^i) = x
    let mut coeff = Scalar::ONE;
    for i in 0..num_vars {
        A.push((num_vars, i, coeff.to_bytes()));
        coeff = coeff + coeff;
    }
    B.push((num_vars, num_vars, one));
    C.push((num_vars, num_vars + 1, one));

    let x = Scalar::from(1234u32);
    
    // 将 x 分解为二进制位
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

    // 创建 VarsAssignment 和 InputsAssignment
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
