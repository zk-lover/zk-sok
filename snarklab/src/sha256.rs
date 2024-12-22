use ark_groth16::{prepare_verifying_key, Groth16};
use ark_crypto_primitives::snark::{CircuitSpecificSetupSNARK, SNARK};
use ark_ec::pairing::Pairing;
use ark_ff::{Field, PrimeField, AdditiveGroup};  // 添加 AdditiveGroup
use ark_serialize::CanonicalSerialize;
use ark_relations::{
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_std::{
    rand::{RngCore, SeedableRng},
    test_rng,
    vec::Vec,
    marker::PhantomData,
    Zero,
    One,
};
use std::time::Instant;
use sha2::{Sha256, Digest};
use ark_crypto_primitives::crh::{
    sha256::constraints::{Sha256Gadget, UnitVar},
    CRHSchemeGadget,
};
use ark_r1cs_std::prelude::*;
use rand_chacha::ChaChaRng;


/// 定义一个电路来证明SHA256哈希值的知识
struct Sha256Circuit<ConstraintF: Field> {
    /// 预映像(输入)
    preimage: Option<Vec<u8>>,
    /// 哈希值(输出)
    hash: Option<Vec<u8>>,
    _phantom: PhantomData<ConstraintF>,
}


impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for Sha256Circuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        println!("Generating constraints...");
        
        // 将预映像转换为电路变量
        let preimage_var = UInt8::new_witness_vec(
            ark_relations::ns!(cs, "preimage"),
            self.preimage.as_ref().unwrap(),
        )?;
        println!("Created preimage variables");

        // 将哈希值转换为电路变量
        let hash_var = UInt8::new_input_vec(
            ark_relations::ns!(cs, "hash"),
            self.hash.as_ref().unwrap(),
        )?;
        println!("Created hash variables");

        // 在电路中计算SHA256
        println!("Computing SHA256 in circuit...");
        let computed_hash = <Sha256Gadget<ConstraintF> as CRHSchemeGadget<_, ConstraintF>>::evaluate(
            &UnitVar::default(),
            &preimage_var,
        )?;

        // 逐字节比较哈希值
        println!("Enforcing hash equality constraints...");
        for (i, (computed_byte, expected_byte)) in computed_hash.0.iter().zip(hash_var.iter()).enumerate() {
            computed_byte.enforce_equal(expected_byte)?;
            println!("Enforced constraint for byte {}", i);
        }

        println!("Finished generating constraints");
        Ok(())
    }
}

fn main() {
    test_prove_and_verify::<ark_bls12_381::Bls12_381>();
}

/// 证明和验证函数
fn test_prove_and_verify<E>()
where
    E: Pairing,
{
    let mut rng = test_rng();
    let mut prover_rng = ChaChaRng::seed_from_u64(rng.next_u32() as u64);

    // 生成随机预映像
    let preimage = b"Hello, World!".to_vec();
    
    // 计算SHA256哈希值
    let mut hasher = Sha256::new();
    hasher.update(&preimage);
    let hash = hasher.finalize().to_vec();

    println!("Preimage: {:?}", preimage);
    println!("Hash: {:?}", hash);

    // 设置电路
    let circuit = Sha256Circuit {
        preimage: Some(preimage.clone()),
        hash: Some(hash.clone()),
        _phantom: PhantomData,
    };

    println!("Setting up circuit...");
    let (pk, vk) = Groth16::<E>::setup(circuit, &mut prover_rng).unwrap();
    
    // 计算和打印密钥大小
    let pk_size = pk.uncompressed_size();
    let vk_size = vk.uncompressed_size();
    println!("Uncompressed pk size: {} bytes", pk_size);
    println!("Uncompressed vk size: {} bytes", vk_size);

    let start1 = Instant::now();
    
    println!("Generating proof...");
    // 生成证明
    let circuit = Sha256Circuit {
        preimage: Some(preimage.clone()),
        hash: Some(hash.clone()),
        _phantom: PhantomData,
    };
    
    let proof = Groth16::<E>::prove(&pk, circuit, &mut prover_rng).unwrap();
    let start2 = Instant::now();

    println!("Processing verification key...");
let pvk = prepare_verifying_key(&vk);

// 将哈希值转换为公共输入
let public_inputs: Vec<E::ScalarField> = hash
    .iter()
    .map(|&byte| E::ScalarField::from(byte as u64))
    .collect();

println!("Number of public inputs: {}", public_inputs.len());

println!("Verifying proof...");
// 修改验证方式，使用 prepare_verifying_key
let is_valid = Groth16::<E>::verify(&vk, &public_inputs, &proof).unwrap();
   
    
    let start3 = Instant::now();

    println!("Proof is valid: {}", is_valid);

    let duration1 = start2.duration_since(start1);
    let duration2 = start3.duration_since(start2);

    println!("Prove time: {:.3} ms", duration1.as_secs_f64() * 1000.0);
    println!("Verify time: {:.3} ms", duration2.as_secs_f64() * 1000.0);
}