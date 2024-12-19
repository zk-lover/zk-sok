use std::{alloc::{GlobalAlloc, Layout, System}, marker::PhantomData, sync::{atomic::{AtomicUsize, Ordering}, Once}};
use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Fixed, Instance, Selector},
    poly::Rotation,
};
use group::ff::Field;
use std::time::Instant;

// 自定义分配器
struct CountingAllocator {
    total: AtomicUsize,
}

impl CountingAllocator {
    fn new() -> Self {
        CountingAllocator {
            total: AtomicUsize::new(0),
        }
    }

    fn get_total(&self) -> usize {
        self.total.load(Ordering::SeqCst)
    }
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            self.total.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        self.total.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

// 使用 Once 和 Lazy 实现全局分配器
static INIT: Once = Once::new();
static mut GLOBAL_ALLOCATOR: Option<CountingAllocator> = None;

fn init_allocator() {
    INIT.call_once(|| {
        unsafe {
            GLOBAL_ALLOCATOR = Some(CountingAllocator::new());
        }
    });
}

struct CountingAllocatorWrapper;

unsafe impl GlobalAlloc for CountingAllocatorWrapper {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        init_allocator();
        GLOBAL_ALLOCATOR.as_ref().unwrap().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        GLOBAL_ALLOCATOR.as_ref().unwrap().dealloc(ptr, layout)
    }
}

// 设置全局分配器
#[global_allocator]
static GLOBAL: CountingAllocatorWrapper = CountingAllocatorWrapper;
trait NumericInstructions<F: Field>: Chip<F> {
    type Num;
    fn load_private(&self, layouter: impl Layouter<F>, a: Value<F>) -> Result<Self::Num, Error>;
    fn load_constant(&self, layouter: impl Layouter<F>, constant: F) -> Result<Self::Num, Error>;
    fn mul(&self, layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error>;
    fn expose_public(&self, layouter: impl Layouter<F>, num: Self::Num, row: usize) -> Result<(), Error>;
}

struct FieldChip<F: Field> {
    config: FieldConfig,
    _marker: PhantomData<F>,
}

#[derive(Clone, Debug)]
struct FieldConfig {
    advice: [Column<Advice>; 2],
    instance: Column<Instance>,
    s_mul: Selector,
}

impl<F: Field> FieldChip<F> {
    fn construct(config: <Self as Chip<F>>::Config) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>, advice: [Column<Advice>; 2], instance: Column<Instance>, constant: Column<Fixed>) -> <Self as Chip<F>>::Config {
        meta.enable_equality(instance);
        meta.enable_constant(constant);
        for column in &advice {
            meta.enable_equality(*column);
        }
        let s_mul = meta.selector();

        meta.create_gate("mul", |meta| {
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let rhs = meta.query_advice(advice[1], Rotation::cur());
            let out = meta.query_advice(advice[0], Rotation::next());
            let s_mul = meta.query_selector(s_mul);

            vec![s_mul * (lhs * rhs - out)]
        });

        FieldConfig {
            advice,
            instance,
            s_mul,
        }
    }
}

impl<F: Field> Chip<F> for FieldChip<F> {
    type Config = FieldConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

#[derive(Clone)]
struct Number<F: Field>(AssignedCell<F, F>);

impl<F: Field> NumericInstructions<F> for FieldChip<F> {
    type Num = Number<F>;

    fn load_private(&self, mut layouter: impl Layouter<F>, value: Value<F>) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(|| "load private", |mut region| {
            region.assign_advice(|| "private input", config.advice[0], 0, || value).map(Number)
        })
    }

    fn load_constant(&self, mut layouter: impl Layouter<F>, constant: F) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(|| "load constant", |mut region| {
            region.assign_advice_from_constant(|| "constant value", config.advice[0], 0, constant).map(Number)
        })
    }

    fn mul(&self, mut layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(|| "mul", |mut region: Region<'_, F>| {
            config.s_mul.enable(&mut region, 0)?;
            a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
            b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;
            let value = a.0.value().copied() * b.0.value();
            region.assign_advice(|| "lhs * rhs", config.advice[0], 1, || value).map(Number)
        })
    }

    fn expose_public(&self, mut layouter: impl Layouter<F>, num: Self::Num, row: usize) -> Result<(), Error> {
        let config = self.config();
        layouter.constrain_instance(num.0.cell(), config.instance, row)
    }
}

#[derive(Default)]
struct MyCircuit<F: Field> {
    constant: F,
    a: Value<F>,
    b: Value<F>,
}

impl<F: Field> Circuit<F> for MyCircuit<F> {
    type Config = FieldConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = [meta.advice_column(), meta.advice_column()];
        let instance = meta.instance_column();
        let constant = meta.fixed_column();
        FieldChip::configure(meta, advice, instance, constant)
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        let field_chip = FieldChip::<F>::construct(config);
        let a = field_chip.load_private(layouter.namespace(|| "load a"), self.a)?;
        let b = field_chip.load_private(layouter.namespace(|| "load b"), self.b)?;
        let constant = field_chip.load_constant(layouter.namespace(|| "load constant"), self.constant)?;
        let ab = field_chip.mul(layouter.namespace(|| "a * b"), a, b)?;
        let absq = field_chip.mul(layouter.namespace(|| "ab * ab"), ab.clone(), ab)?;
        let mut c = field_chip.mul(layouter.namespace(|| "constant * absq"), constant.clone(), absq)?;

        for i in 0..30 {
            c = field_chip.mul(
                layouter.namespace(|| format!("constant * c iteration {}", i)),
                constant.clone(),
                c,
            )?;
        }

        field_chip.expose_public(layouter.namespace(|| "expose c"), c, 0)
    }
}

fn main() {
    use halo2_proofs::{dev::MockProver, pasta::Fp};

    let k = 8;
    let constant = Fp::from(1);
    let a = Fp::from(2);
    let b = Fp::from(3);
    let c = constant * a.square() * b.square();

    let circuit = MyCircuit {
        constant,
        a: Value::known(a),
        b: Value::known(b),
    };

    let mut public_inputs = vec![c];
    public_inputs[0] += Fp::one();
    
    init_allocator();
    
    let start1 = Instant::now();
    let allocator_before = unsafe { GLOBAL_ALLOCATOR.as_ref().unwrap().get_total() };
    let prover: MockProver<_> = MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
    let allocator_after = unsafe { GLOBAL_ALLOCATOR.as_ref().unwrap().get_total() };
    let start2 = Instant::now();
    assert!(prover.verify().is_err());
    let start3 = Instant::now();
    let duration1 = start2.duration_since(start1);
    let duration2 = start3.duration_since(start2);
    let memory_used = allocator_after - allocator_before;
    let millis1 = duration1.as_secs_f64() * 1000.0;
    let millis2 = duration2.as_secs_f64() * 1000.0;
    println!("Prove time: {:.3} milliseconds", millis1);
    println!("Verify time: {:.3} milliseconds", millis2);
    println!("Memory used by prover: {} bytes", memory_used);
}
