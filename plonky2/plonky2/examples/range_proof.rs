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
    builder.range_check(value, log_max);

    let mut pw = PartialWitness::new();
    pw.set_target(value, F::from_canonical_usize(10086));

    println!(
        "Constructing inner proof with {} gates",
        builder.num_gates()
    );
    let data: plonky2::plonk::circuit_data::CircuitData<plonky2_field::goldilocks_field::GoldilocksField, PoseidonGoldilocksConfig, 2> = builder.build::<C>();
    let start1 = Instant::now();
    let proof: plonky2::plonk::proof::ProofWithPublicInputs<plonky2_field::goldilocks_field::GoldilocksField, PoseidonGoldilocksConfig, 2> = data.prove(pw)?;
    let start2 = Instant::now();
    // 调用 `to_bytes()` 方法将 `proof` 转换为字节数组
    let proof_bytes = proof.to_bytes();
    // 获取字节数组的大小
    let size = proof_bytes.len();
    // 输出字节数组的大小
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
