use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    plonk::{Circuit, ConstraintSystem, Error},
    dev::MockProver,
    pasta::Fp,
};
use halo2_gadget::sha256::{Sha256, Sha256Instructions};
use halo2_gadget::sha256::table16::{Table16Chip, BlockWord};
struct MySha256Circuit {
    data: Vec<u8>, // 输入数据
}

impl Circuit<Fp> for MySha256Circuit {
    type Config = Sha256Config;
    type FloorPlanner = SimpleFloorPlanner;

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let sha256_chip = Table16Chip::configure(meta);
        Sha256Config { sha256_chip }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        let chip = Table16Chip::construct(config.sha256_chip);
        let mut sha256 = Sha256::new(chip, layouter.namespace(|| "init"))?;
        
        // 将输入数据转换为BlockWord并更新哈希状态
        let data_words: Vec<BlockWord> = self.data.iter().map(|&b| BlockWord::from(b)).collect();
        sha256.update(layouter.namespace(|| "update"), &data_words)?;

        // 最终计算哈希摘要
        let _digest = sha256.finalize(layouter.namespace(|| "finalize"))?;

        Ok(())
    }
}

fn main() {
    // 创建一个数据输入的电路实例
    let circuit = MySha256Circuit {
        data: b"hello world".to_vec(),
    };

    // 为电路设置公共输入（这里为电路输出）
    let k = 17; // 设置电路的规模参数
    let public_inputs = vec![];

    // 创建一个mock prover来验证电路
    let prover = MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
    prover.assert_satisfied();
}
