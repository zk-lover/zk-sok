use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use std::time::Instant;

/// An example of using Plonky2 to prove that a given value lies in a given range.
fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_zk_config();//或者用standard_recursion_zk_config
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The secret value.
    let value = builder.add_virtual_target();

    // Registered as a public input (even though it's secret) so we can print out the value later.
    builder.register_public_input(value);

    let log_max = 32;
    println!("Gates before range check: {}", builder.num_gates());
    builder.range_check(value, log_max);
    println!("Gates after range check: {}", builder.num_gates());

    // 添加调试信息
    println!("Number of gates before building: {}", builder.num_gates());
    println!("Number of public inputs: {}", builder.num_public_inputs());

    let mut pw = PartialWitness::new();
    let _ = pw.set_target(value, F::from_canonical_usize(10086));

    println!(
        "Constructing inner proof with {} gates",
        builder.num_gates()
    );
    
    let start1 = Instant::now();
    // 构建电路数据
    let data = builder.build::<C>();
    // 生成证明
    let proof = data.prove(pw)?;
    let start2 = Instant::now();
    
    // 转换证明为字节数组
    let proof_bytes = proof.to_bytes();
    let size = proof_bytes.len();
    println!("Size of proof_bytes: {}", size);

    println!(
        "Value {} is less than 2^{}",
        proof.public_inputs[0], log_max,
    );

    data.verify(proof);
    let start3 = Instant::now();
    let duration1 = start2.duration_since(start1);
    let duration2 = start3.duration_since(start2);

    let millis1 = duration1.as_secs_f64() * 1000.0;
    let millis2 = duration2.as_secs_f64() * 1000.0;
    println!("Prove time: {:.3} milliseconds", millis1);
    println!("Verify time: {:.3} milliseconds", millis2);
    Ok(())
}
