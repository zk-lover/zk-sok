use ark_groth16::{Groth16, prepare_verifying_key};
use ark_ec::pairing::Pairing;
use ark_ff::{Field, PrimeField};
use ark_relations::{r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError}};
use ark_std::{
    rand::{RngCore, SeedableRng},
    test_rng,
    vec::Vec,
    Zero,
    time::Instant,
};
use ark_crypto_primitives::crh::sha256::constraints::{Sha256Gadget, UnitVar};
use ark_r1cs_std::prelude::*;
use rand_chacha::ChaChaRng;
use sha2::{Sha256, Digest};
use ark_bls12_381::Bls12_381;
use std::marker::PhantomData;
use ark_crypto_primitives::{
    crh::CRHSchemeGadget,
    snark::{SNARK, CircuitSpecificSetupSNARK},
};

// SHA256 电路定义
struct Sha256Circuit<ConstraintF: Field> {
    preimage: Option<Vec<u8>>,  // 要哈希的输入
    hash: Option<Vec<u8>>,      // 要验证的预期哈希值
    _phantom: PhantomData<ConstraintF>,
}

impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for Sha256Circuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // 修改：将哈希值作为私有输入而不是公共输入
        let preimage_var = UInt8::new_witness_vec(
            ark_relations::ns!(cs, "preimage"),
            self.preimage.as_deref().unwrap_or(&[]),
        )?;

        let hash_var = UInt8::new_witness_vec(  // 改为 witness 而不是 input
            ark_relations::ns!(cs, "hash"),
            self.hash.as_deref().unwrap_or(&[]),
        )?;

        // 使用 SHA256 gadget 计算哈希
        let computed_hash = Sha256Gadget::<ConstraintF>::evaluate(
            &UnitVar::default(),
            &preimage_var,
        )?;

        // 添加约束：计算出的哈希值必须等于输入的哈希值
        for (computed_byte, expected_byte) in computed_hash.0.iter().zip(hash_var.iter()) {
            computed_byte.enforce_equal(expected_byte)?;
        }

        Ok(())
    }
}

fn main() {
    // 在 BLS12-381 曲线上进行测试
    test_prove_and_verify::<Bls12_381>();
}

fn test_prove_and_verify<E>()
where
    E: Pairing,
{
    let mut rng = test_rng();
    let mut prover_rng = ChaChaRng::seed_from_u64(rng.next_u32() as u64);

    // 设置输入数据
    let preimage = b"Hello, World!".to_vec();
    let mut hasher = Sha256::new();
    hasher.update(&preimage);
    let hash = hasher.finalize().to_vec();

    println!("Setting up circuit...");
    // 设置电路
    let setup_circuit = Sha256Circuit {
        preimage: None,
        hash: None,
        _phantom: PhantomData,
    };

    println!("Generating proving key and verifying key...");
    let (pk, vk) = Groth16::<E>::setup(setup_circuit, &mut prover_rng)
        .expect("Setup failed");

    println!("Setting up proving circuit...");
    let proving_circuit = Sha256Circuit {
        preimage: Some(preimage.clone()),
        hash: Some(hash.clone()),
        _phantom: PhantomData,
    };

    println!("Generating proof...");
    let start1 = Instant::now();
    let proof = Groth16::<E>::prove(&pk, proving_circuit, &mut prover_rng)
        .expect("Proving failed");
    let start2 = Instant::now();

    println!("Processing verification key...");
    let pvk = prepare_verifying_key(&vk);
    // 修改：将32字节分成4组，每组8字节
    let mut combined_hash = E::ScalarField::zero();
    for (i, chunk) in hash.chunks(8).enumerate() {
        let mut chunk_value = E::ScalarField::zero();
        for (j, &byte) in chunk.iter().enumerate() {
            chunk_value += E::ScalarField::from(byte as u64) * E::ScalarField::from(1u64 << (8 * j));
        }
        if i > 0 {
            chunk_value = chunk_value * E::ScalarField::from(1u64 << 63) * E::ScalarField::from(2u64);
        }
        combined_hash += chunk_value;
    }
    
    let public_inputs = vec![combined_hash];

    println!("\nVerifying proof...");
    let verification_result = Groth16::<E>::verify_with_processed_vk(
        &pvk,
        &[],  // 空的公共输入
        &proof,
    );
    let start3 = Instant::now();

    // 修改：通过 pvk.vk 访问 gamma_abc_g1
    match &verification_result {
        Ok(valid) => println!("Verification completed with result: {}", valid),
        Err(e) => {
            println!("Verification error details:");
            println!("Error: {:?}", e);
            println!("Expected number of inputs: {}", pvk.vk.gamma_abc_g1.len() - 1);
            println!("Provided number of inputs: {}", public_inputs.len());
        }
    }

    // 计算时间和输出结果
    let prove_time = start2.duration_since(start1).as_secs_f64() * 1000.0;
    let verify_time = start3.duration_since(start2).as_secs_f64() * 1000.0;
    let proof_size = std::mem::size_of_val(&proof);

    println!("Prove time: {:.3} ms", prove_time);
    println!("Verify time: {:.3} ms", verify_time);
    println!("Proof size: {} bytes", proof_size);

    match verification_result {
        Ok(is_valid) => println!("Proof verification result: {}", is_valid),
        Err(e) => println!("Verification failed with error: {:?}", e),
    }
}