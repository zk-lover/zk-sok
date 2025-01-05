use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::{Fp, EqAffine},
    plonk::{Circuit, ConstraintSystem, Error},
};
use halo2_gadgets::sha256::{Sha256, Table16Chip, Table16Config, BlockWord};
use rand_core::OsRng;
use std::time::Instant;

#[derive(Default)]
struct Sha256Circuit {
    pub input: Vec<u8>,
}

impl Circuit<Fp> for Sha256Circuit {
    type Config = Table16Config;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        Table16Chip::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        let table16_chip = Table16Chip::construct(config.clone());

        Table16Chip::load(config, &mut layouter.namespace(|| "load_table"))?;

        let mut input_bytes = self.input.clone();
        
        let len = input_bytes.len();
        let len_bits = len * 8;
        
        input_bytes.push(0x80);
        while (input_bytes.len() + 8) % 64 != 0 {
            input_bytes.push(0);
        }
        input_bytes.extend_from_slice(&((len_bits as u64).to_be_bytes()));

        println!("Padded input length: {} bytes", input_bytes.len());

        let mut input_blocks = Vec::new();
        for chunk in input_bytes.chunks(4) {
            let mut buf = [0u8; 4];
            buf[..chunk.len()].copy_from_slice(chunk);
            input_blocks.push(BlockWord(Value::known(u32::from_be_bytes(buf))));
        }

        let mut block = [BlockWord(Value::known(0u32)); 16];
        for (i, word) in input_blocks.iter().take(16).enumerate() {
            block[i] = *word;
        }

        let mut sha256 = Sha256::new(
            table16_chip,
            layouter.namespace(|| "sha256"),
        )?;

        sha256.update(
            layouter.namespace(|| "update"),
            &block,
        )?;

        let _digest = sha256.finalize(layouter.namespace(|| "finalize"))?;

        Ok(())
    }
}

fn main() {
    let k = 17;

    let message = b"Hello, ZK!";
    println!("Input message: {:?}", String::from_utf8_lossy(message));
    println!("Message length: {} bytes", message.len());

    let circuit = Sha256Circuit {
        input: message.to_vec(),
    };

    println!("Creating parameters with k = {}...", k);
    let start = Instant::now();
    let params = halo2_proofs::poly::commitment::Params::<EqAffine>::new(k);
    println!("Parameters created in: {:?}", start.elapsed());

    println!("Generating verification key...");
    let start = Instant::now();
    let vk = halo2_proofs::plonk::keygen_vk(&params, &circuit).expect("keygen_vk failed");
    println!("Verification key generated in: {:?}", start.elapsed());
    
    println!("Generating proving key...");
    let start = Instant::now();
    let pk = halo2_proofs::plonk::keygen_pk(&params, vk.clone(), &circuit).expect("keygen_pk failed");
    println!("Proving key generated in: {:?}", start.elapsed());

    println!("Creating proof...");
    let start = Instant::now();
    let mut transcript = halo2_proofs::transcript::Blake2bWrite::<_, _, halo2_proofs::transcript::Challenge255<_>>::init(vec![]);
    
    halo2_proofs::plonk::create_proof(
        &params,
        &pk,
        &[circuit],
        &[&[]],
        OsRng,
        &mut transcript,
    ).expect("proof generation failed");

    let proof = transcript.finalize();
    let proof_time = start.elapsed();
    println!("Proof created in: {:?}", proof_time);
    println!("Proof size: {} bytes", proof.len());

    println!("Verifying proof...");
    let start = Instant::now();
    let mut transcript = halo2_proofs::transcript::Blake2bRead::<_, _, halo2_proofs::transcript::Challenge255<_>>::init(&proof[..]);
    let strategy = halo2_proofs::plonk::SingleVerifier::new(&params);
    
    let result = halo2_proofs::plonk::verify_proof(
        &params,
        &vk,
        strategy,
        &[&[]], 
        &mut transcript,
    );
    let verify_time = start.elapsed();

    match result {
        Ok(_) => println!("Proof verification successful in: {:?}", verify_time),
        Err(e) => println!("Proof verification failed in: {:?} - Error: {:?}", verify_time, e),
    }

    println!("\nPerformance Summary:");
    println!("- Proof Generation Time: {:?}", proof_time);
    println!("- Proof Verification Time: {:?}", verify_time);
}
