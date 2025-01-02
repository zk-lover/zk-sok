use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::{Fp, EqAffine},
    plonk::{Circuit, ConstraintSystem, Error},
};
use halo2_gadgets::sha256::{Sha256, Table16Chip, Table16Config, BlockWord};
use rand_core::OsRng;

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
        let table16_chip = Table16Chip::construct(config);

        // 构造输入块，填充至 64 字节的倍数
        let mut input_bytes = self.input.clone();
        
        // SHA-256 填充
        let len = input_bytes.len();
        let len_bits = len * 8;
        
        // 添加 1 位
        input_bytes.push(0x80);
        
        // 填充 0 直到长度满足要求
        while (input_bytes.len() + 8) % 64 != 0 {
            input_bytes.push(0);
        }
        
        // 添加长度（大端序，64位）
        input_bytes.extend_from_slice(&((len_bits as u64).to_be_bytes()));

        println!("Padded input length: {} bytes", input_bytes.len());
        println!("Padded input: {:?}", input_bytes);

        // 将字节转换为 BlockWord
        let mut input_blocks = Vec::new();
        for chunk in input_bytes.chunks(4) {
            let mut buf = [0u8; 4];
            buf[..chunk.len()].copy_from_slice(chunk);
            input_blocks.push(BlockWord(Value::known(u32::from_be_bytes(buf))));
        }

        // 确保我们有足够的块（16个32位字）
        let mut block = [BlockWord(Value::known(0u32)); 16];
        for (i, word) in input_blocks.iter().take(16).enumerate() {
            block[i] = *word;
        }

        println!("Number of message blocks: 1");

        // 初始化 SHA256 实例并处理消息
        let mut sha256 = Sha256::new(
            table16_chip,
            layouter.namespace(|| "sha256"),
        )?;

        // 处理消息块
        sha256.update(
            layouter.namespace(|| "update"),
            &block,
        )?;

        // 完成哈希计算
        let _digest = sha256.finalize(layouter.namespace(|| "finalize"))?;

        Ok(())
    }
}

fn main() {
    // 增加 k 的值以容纳更多约束
    let k = 20;

    let message = b"Hello, ZK!";
    println!("Input message: {:?}", String::from_utf8_lossy(message));
    println!("Message length: {} bytes", message.len());

    let circuit = Sha256Circuit {
        input: message.to_vec(),
    };

    println!("Generating proving key...");
    let params = halo2_proofs::poly::commitment::Params::<EqAffine>::new(k);

    println!("Generating verification key...");
    let vk = halo2_proofs::plonk::keygen_vk(&params, &circuit).expect("keygen_vk failed");
    
    println!("Generating proving key...");
    let pk = halo2_proofs::plonk::keygen_pk(&params, vk.clone(), &circuit).expect("keygen_pk failed");

    println!("Creating proof...");
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
    println!("Proof size: {} bytes", proof.len());

    // 验证证明
    println!("Verifying proof...");
    let mut transcript = halo2_proofs::transcript::Blake2bRead::<_, _, halo2_proofs::transcript::Challenge255<_>>::init(&proof[..]);
    let strategy = halo2_proofs::plonk::SingleVerifier::new(&params);
    
    let result = halo2_proofs::plonk::verify_proof(
        &params,
        &vk,
        strategy,
        &[&[]], 
        &mut transcript,
    );

    match result {
        Ok(_) => println!("Proof verification successful!"),
        Err(e) => println!("Proof verification failed: {:?}", e),
    }
}
