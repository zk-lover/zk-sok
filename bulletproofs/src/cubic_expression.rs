#[cfg(feature = "yoloproofs")]
#[cfg(feature = "std")]
use bulletproofs::r1cs::{Prover, Verifier, LinearCombination, Variable,ConstraintSystem};
use bulletproofs::{BulletproofGens, PedersenGens};
use curve25519_dalek::scalar::Scalar;
use merlin::Transcript;
use rand::thread_rng;



fn main() {
    // 创建生成器
let bp_gens = BulletproofGens::new(64, 1);
let pc_gens = PedersenGens::default();

// 创建变量
let x_var = Variable::new();
let y_var = Variable::new();

// 创建线性组合
let x = Scalar::from(3u64);
let five = Scalar::from(5u64);
let y_value = Scalar::from(10u64);

// 线性组合：x^3 + x + 5
let lhs = LinearCombination::from((x_var, x)) + x_var + (five, Variable::One());

// 创建证明者和验证者
let mut prover = Prover::new();
let mut verifier = Verifier::new();

// 添加约束
prover.constrain(lhs - (y_value, Variable::One()));

// 生成证明
let proof = prover.prove(&bp_gens, &pc_gens).unwrap();

// 验证证明
assert!(verifier.verify(&proof, &bp_gens, &pc_gens).is_ok());
}
