use anyhow::Result;
use log::{Level, LevelFilter};
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::util::timing::TimingTree;
use plonky2_sha256::circuit::{array_to_bits, make_circuits};
use sha2::{Digest, Sha256};

pub fn prove_sha256(msg: &[u8]) -> Result<()> {
    let mut hasher = Sha256::new();
    hasher.update(msg);
    let hash = hasher.finalize();

    let msg_bits = array_to_bits(msg);
    let len = msg.len() * 8;
    println!("block count: {}", (len + 65 + 511) / 512);
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;
    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());
    let targets = make_circuits(&mut builder, len as u64);
    let mut pw = PartialWitness::new();

    for i in 0..len {
        pw.set_bool_target(targets.message[i], msg_bits[i]);
    }

    let expected_res = array_to_bits(hash.as_slice());
    for i in 0..expected_res.len() {
        if expected_res[i] {
            builder.assert_one(targets.digest[i].target);
        } else {
            builder.assert_zero(targets.digest[i].target);
        }
    }

    println!("Circuit statistics:");
    println!("- Number of gates: {}", builder.num_gates());
    println!("- Number of public inputs: {}", builder.num_public_inputs());
    
    // 打印每种门类型的统计信息
    builder.print_gate_counts(0);

    // 手动计算约束总数
    let base_sum_63_constraints = 368 * (63 + 2); // num_limbs + 2 约束
    let base_sum_32_constraints = 616 * (32 + 2); // num_limbs + 2 约束
    let arithmetic_constraints = 4647 * 20;  // num_ops 约束
    let u32_arithmetic_constraints = 200 * 3 * 2; // 两个 U32ArithmeticGate，每个有 3 个约束

    let total_constraints = base_sum_63_constraints + 
                          base_sum_32_constraints + 
                          arithmetic_constraints + 
                          u32_arithmetic_constraints;

    println!("Constraints breakdown:");
    println!("- BaseSumGate(63): {} gates * {} constraints = {}", 368, 65, base_sum_63_constraints);
    println!("- BaseSumGate(32): {} gates * {} constraints = {}", 616, 34, base_sum_32_constraints);
    println!("- ArithmeticGate: {} gates * {} constraints = {}", 4647, 20, arithmetic_constraints);
    println!("- U32ArithmeticGate: {} gates * {} constraints = {}", 200, 6, u32_arithmetic_constraints);
    println!("Total constraints: {}", total_constraints);

    let data = builder.build::<C>();
    let timing = TimingTree::new("prove", Level::Debug);
    let proof = data.prove(pw).unwrap();
    let proof_bytes = proof.to_bytes();
    let size = proof_bytes.len();
    println!("Size of proof_bytes: {}", size);
    timing.print();

    let timing = TimingTree::new("verify", Level::Debug);
    let res = data.verify(proof);
    timing.print();

    res
}

fn main() -> Result<()> {
    // Initialize logging
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp(None);
    builder.filter_level(LevelFilter::Debug);
    builder.try_init()?;

    let msg = b"I love zk-sok";
    prove_sha256(msg)
}
