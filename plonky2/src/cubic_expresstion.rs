use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use std::time::Instant;

/// An example of using Plonky2 to prove a statement of the form
/// "I know x² - 4x + 7".
fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_zk_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The arithmetic circuit.
    let x = builder.add_virtual_target();
    let a = builder.mul(x, x);
    let b = builder.mul(a, x);
    let d = builder.add(b, x);
    let e = builder.add_const(d, F::from_canonical_u32(1));

    // Public inputs are the initial value (provided below) and the result (which is generated).
    builder.register_public_input(x);
    builder.register_public_input(e);
    let mut pw = PartialWitness::new();
    pw.set_target(x, F::from_canonical_u32(3));
    println!(
        "Constructing inner proof with {} gates",
        builder.num_gates()
    );
    let data = builder.build::<C>();


    let start1 = Instant::now();
    let proof = data.prove(pw)?;
    let start2 = Instant::now();
    // 调用 `to_bytes()` 方法将 `proof` 转换为字节数组
    let proof_bytes = proof.to_bytes();
    // 获取字节数组的大小
    let size = proof_bytes.len();
    // 输出字节数组的大小
    println!("Size of proof_bytes: {}", size);

    println!(
        "x3 +x + 1 where x = {} is {}",
        proof.public_inputs[0],
        proof.public_inputs[1]
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