use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{
        Circuit, ConstraintSystem, Error, Expression, Selector,
        create_proof, verify_proof, keygen_pk, keygen_vk, SingleVerifier
    },
    pasta::{Fp, EqAffine},  // Add EqAffine
    poly::{commitment::Params, Rotation},
    transcript::{Blake2bWrite, Blake2bRead, Challenge255},
};
use rand_core::OsRng;  // Use rand_core instead of rand
use std::time::Instant;

#[derive(Default)]
struct RangeProofCircuit {
    input: Value<u64>,
}

#[derive(Clone)]
struct RangeProofConfig {
    input: halo2_proofs::plonk::Column<halo2_proofs::plonk::Advice>,
    bits: [halo2_proofs::plonk::Column<halo2_proofs::plonk::Advice>; 32],
    selector: Selector,
}

impl Circuit<Fp> for RangeProofCircuit {
    type Config = RangeProofConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let input = meta.advice_column();
        let bits = [(); 32].map(|_| meta.advice_column());
        let selector = meta.selector();

        meta.enable_equality(input);
        for bit in &bits {
            meta.enable_equality(*bit);
        }

        meta.create_gate("Range Proof", |meta| {
            let selector = meta.query_selector(selector);
            let input = meta.query_advice(input, Rotation::cur());

            let mut constraints = Vec::new();

            for bit in bits.iter() {
                let b = meta.query_advice(*bit, Rotation::cur());
                constraints.push(selector.clone() * b.clone() * (b - Expression::Constant(Fp::one())));
            }

            let reconstructed_input = bits.iter().enumerate().fold(
                Expression::Constant(Fp::zero()),
                |acc, (i, bit)| {
                    acc + meta.query_advice(*bit, Rotation::cur()) * Expression::Constant(Fp::from(1 << i))
                },
            );

            constraints.push(selector * (input - reconstructed_input));

            constraints
        });

        RangeProofConfig {
            input,
            bits,
            selector,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "Range Proof",
            |mut region| {
                let input_val = self.input;
                region.assign_advice(|| "input", config.input, 0, || input_val.map(Fp::from))?;
    
                // Use and_then and map methods
                let current_val = self.input.and_then(Value::known);
                let mut current = 0u64;
                
                // Use map to process value
                current_val.map(|v| current = v);
    
                for i in 0..32 {
                    let bit_val = current & 1;
                    current >>= 1;
    
                    region.assign_advice(
                        || format!("bit {}", i),
                        config.bits[i],
                        0,
                        || Value::known(Fp::from(bit_val as u64)),
                    )?;
                }
    
                config.selector.enable(&mut region, 0)?;
    
                Ok(())
            },
        )
    }
}

fn main() {
    // Parameter setup
    let k = 12;
    let params = Params::<EqAffine>::new(k);
    
    // Create circuit instance
    let circuit = RangeProofCircuit {
        input: Value::known(12345678),
    };

    // Generate verification key and proving key
    let vk = keygen_vk(&params, &circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk.clone(), &circuit).expect("keygen_pk should not fail");

    // Generate proof
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    println!("Creating proof...");
    let start1 = Instant::now();
    create_proof(
        &params,
        &pk,
        &[circuit],
        &[&[]],
        OsRng,
        &mut transcript,
    ).expect("Proof generation should not fail");
    let start2 = Instant::now();
    let proof = transcript.finalize();

    // Verify proof
    let strategy = SingleVerifier::new(&params);  // Add verification strategy
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
    let verify_result = verify_proof(
        &params,
        &vk,
        strategy,  // Use verification strategy
        &[&[]],
        &mut transcript,  // Modify parameter order
    );
    let start3 = Instant::now();

    // Calculate time and size
    let prove_time = start2.duration_since(start1).as_secs_f64() * 1000.0;
    let verify_time = start3.duration_since(start2).as_secs_f64() * 1000.0;
    let proof_size = proof.len();

    // Output results
    println!("Prove time: {:.3} ms", prove_time);
    println!("Verify time: {:.3} ms", verify_time);
    println!("Proof size: {} bytes", proof_size);
    println!("Verification result: {:?}", verify_result);
}