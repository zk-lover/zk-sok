use std::marker::PhantomData;
use std::time::Instant;
use group::ff::Field;
use group::ff::PrimeField;
use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Expression, Instance, Selector},
    poly::Rotation,
};
use std::ops::Not;

// ANCHOR: range-config
#[derive(Clone, Debug)]
struct RangeConfig {
    advice: [Column<Advice>; 1],
    instance: Column<Instance>,
    s_range: Selector,
}
// ANCHOR END: range-config

// ANCHOR: range-chip
struct RangeChip<F: Field> {
    config: RangeConfig,
    _marker: PhantomData<F>,
}
// ANCHOR END: range-chip

// ANCHOR: range-chip-impl
impl<F: Field> Chip<F> for RangeChip<F> {
    type Config = RangeConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: Field> RangeChip<F> {
    fn construct(config: &<Self as Chip<F>>::Config, _loaded: <Self as Chip<F>>::Loaded) -> Self {
        Self {
            config: config.clone(),
            _marker: PhantomData,
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: Column<Advice>,
        instance: Column<Instance>,
    ) -> <Self as Chip<F>>::Config {
        let s_range = meta.selector();
        
        // 启用 Advice 列的相等性
        meta.enable_equality(advice);
        
        // 启用 Instance 列的相等性
        meta.enable_equality(instance);
        
        // 定义我们的 range check 门！
        meta.create_gate("range check", |meta| {
            let value = meta.query_advice(advice, Rotation::cur());
            let s_range = meta.query_selector(s_range);

            // 位检查：将值拆分为 8 个二进制位并检查每个位是 0 或 1
            let mut constraints = Vec::new();
            // 位检查：将值拆分为 8 个二进制位并检查每个位是 0 或 1
            let mut current_value = value;
            for i in 0..8 {
                let bit = meta.query_advice(advice, Rotation(i as i32));
                constraints.push(s_range.clone() * (bit.clone() * (bit.clone() - Expression::Constant(F::ONE))));
    
                // 使用 2 的幂来减少当前值中的位
                let constant2 = F::ONE + F::ONE;
                let power_of_two = constant2.pow_vartime(&[i as u64]);
                current_value = current_value - bit.clone() * Expression::Constant(power_of_two);
            }
            constraints.push(s_range * current_value);

            constraints
        });

        RangeConfig { advice: [advice], instance, s_range }
    }
}

// ANCHOR END: range-chip-impl

// ANCHOR: range-instructions-impl
impl<F: Field + PrimeField> RangeChip<F> {
    fn range_check(
        &self,
        layouter: &mut impl Layouter<F>,
        value: &AssignedCell<F, F>,
    ) -> Result<(), Error> {
        let config = self.config();
        layouter.assign_region(
            || "range check",
            |mut region: Region<'_, F>| {
                // 启用选择器
                config.s_range.enable(&mut region, 0)?;

                // 复制输入值到第一个单元格
                let value_cell = value.copy_advice(
                    || "value",
                    &mut region,
                    config.advice[0],
                    0
                )?;

                // 获取值
                let value_value = value_cell.value().copied();

                // 修改位提取逻辑
                value_value.map(|v| {
                    let mut num = v;
                    let two = F::from(2u64);
                    for i in 0..8 {
                        // 修改比较逻辑，使用 Field 特征的方法
                        let bit = {
                            let mut temp = num;
                            while temp.is_zero().not().into() {
                                if (temp - two).is_zero().not().into() {
                                    temp = temp - two;
                                } else {
                                    break;
                                }
                            }
                            temp
                        };
                        
                        region.assign_advice(
                            || format!("bit {}", i),
                            config.advice[0],
                            i + 1,
                            || Value::known(bit),
                        ).unwrap();
                        
                        // 使用减法和乘法更新 num
                        num = (num - bit) * (two.invert().unwrap());
                    }
                });

                Ok(())
            },
        )
    }
}
// ANCHOR END: range-instructions-impl

// ANCHOR: circuit
#[derive(Default)]
struct RangeCircuit<F: Field + PrimeField> {
    x: Value<F>,
}

impl<F: Field + PrimeField> Circuit<F> for RangeCircuit<F> {
    type Config = RangeConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = meta.advice_column();
        let instance = meta.instance_column();
        RangeChip::configure(meta, advice, instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let range_chip = RangeChip::<F>::construct(&config, ()); // 使用引用


        // Load x as a private input.
        let x = layouter.assign_region(
            || "load x",
            |mut region| {
                region.assign_advice(|| "x", config.advice[0], 0, || self.x)

            },
        )?;

        // Check if x is within the range.
        range_chip.range_check(&mut layouter, &x)?;

        // Expose the result x as a public input.
        layouter.constrain_instance(x.cell(), config.instance, 0)
    }
}
// ANCHOR END: circuit

#[allow(clippy::many_single_char_names)]
fn main() {
    use halo2_proofs::{dev::MockProver, pasta::Fp};
    use rand_core::OsRng;

    let k = 5;
    let rng = OsRng;

    let x = Fp::random(rng);

    let circuit = RangeCircuit {
        x: Value::known(x),
    };

    let mut public_inputs = vec![x];

    // Given the correct public input, our circuit will verify.
    let start1 = Instant::now();
    let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
    
    let start2 = Instant::now();
    let verification_result = prover.verify();
    let start3 = Instant::now();
    let duration1 = start2.duration_since(start1);
    let duration2 = start3.duration_since(start2);

    let millis1 = duration1.as_secs_f64() * 1000.0;
    let millis2 = duration2.as_secs_f64() * 1000.0;
    println!("Prove time: {:.3} milliseconds", millis1);
    println!("Verify time: {:.3} milliseconds", millis2);
    if verification_result.is_ok() {
        println!("Proof verification succeeded with correct public input.");
    } else {
        println!("Proof verification failed with correct public input.");
    }

    assert_eq!(verification_result, Ok(()));

    // If we try some other public input, the proof will fail.
    public_inputs[0] += Fp::one();
    let prover = MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
    assert!(prover.verify().is_err());  
}
