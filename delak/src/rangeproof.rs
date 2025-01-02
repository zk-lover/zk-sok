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
    // 1. 初始化随机数生成器
    let mut rng = OsRng;

    // 2. 生成Pedersen承诺的基础生成器
    let pedersen_gens = PedersenGens::default();

    // 3. 创建 BulletproofGens，指定最大证明的数量
    let bulletproof_gens = BulletproofGens::new(32, 1);  // 2^32 range

    // 4. 选择一个值进行范围证明（例如一个值在0到2^32-1之间）
    let value: u32 = 1234567890;  // 确保这个值在 0 到 2^32-1 之间
    
    // 5. 创建随机数用于盲化因子
    let blinding = Scalar::random(&mut rng);

    println!("Creating proof for value: {} (range: 0 to {})", value, u32::MAX);

    // 生成证明
    let proving_time = Instant::now();
    let mut prover_transcript = Transcript::new(b"range_proof");
    let (proof, committed_value) = RangeProof::prove_single(
        &bulletproof_gens,
        &pedersen_gens,
        &mut prover_transcript,
        value as u64,  // 转换为 u64
        &blinding,     // 盲化因子
        32,  // 位数改为 32
    ).expect("Proof generation failed");
    let proving_duration = proving_time.elapsed();

    // 计算证明大小
    let proof_size = std::mem::size_of_val(&proof);
    println!("Proving time: {:?}", proving_duration);
    println!("Proof size: {} bytes", proof_size);

    // 验证证明
    let verifying_time = Instant::now();
    let mut verifier_transcript = Transcript::new(b"range_proof");
    let result = proof.verify_single(
        &bulletproof_gens,
        &pedersen_gens,
        &mut verifier_transcript,
        &committed_value,
        32,  // 位数改为 32
    );
    let verifying_duration = verifying_time.elapsed();
    println!("Verification time: {:?}", verifying_duration);

    match result {
        Ok(_) => println!("Proof verification successful!"),
        Err(e) => println!("Proof verification failed: {:?}", e),
    }
}
