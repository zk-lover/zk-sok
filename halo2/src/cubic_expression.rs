use std::marker::PhantomData;
use std::{alloc::{GlobalAlloc, Layout, System}, sync::{atomic::{AtomicUsize, Ordering}, Once}};

use std::time::Instant;
use group::ff::Field;
use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Expression, Instance, Selector},
    poly::Rotation,
};
use pasta_curves::Fp;

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

#[global_allocator]
static GLOBAL: CountingAllocatorWrapper = CountingAllocatorWrapper;
// ANCHOR: field-instructions
/// A variable representing a number.
#[derive(Clone)]
struct Number<F: Field>(AssignedCell<F, F>);

trait FieldInstructions<F: Field>: AddInstructions<F> + MulInstructions<F> {
    /// Variable representing a number.
    type Num;

    /// Loads a number into the circuit as a private input.
    fn load_private(
        &self,
        layouter: impl Layouter<F>,
        a: Value<F>,
    ) -> Result<<Self as FieldInstructions<F>>::Num, Error>;

    fn load_constant(&self, layouter: impl Layouter<F>, value: F) -> Result<<Self as FieldInstructions<F>>::Num, Error>;

    /// Returns `y = x**3 + x + 5`.
    fn cubic_expression(
        &self,
        layouter: &mut impl Layouter<F>,
        x: <Self as FieldInstructions<F>>::Num,
    ) -> Result<<Self as FieldInstructions<F>>::Num, Error>;

    /// Exposes a number as a public input to the circuit.
    fn expose_public(
        &self,
        layouter: impl Layouter<F>,
        num: <Self as FieldInstructions<F>>::Num,
        row: usize,
    ) -> Result<(), Error>;
}
// ANCHOR_END: field-instructions

// ANCHOR: add-instructions
trait AddInstructions<F: Field>: Chip<F> {
    /// Variable representing a number.
    type Num;

    /// Returns `c = a + b`.
    fn add(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error>;
}
// ANCHOR_END: add-instructions

// ANCHOR: mul-instructions
trait MulInstructions<F: Field>: Chip<F> {
    /// Variable representing a number.
    type Num;

    /// Returns `c = a * b`.
    fn mul(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error>;
}
// ANCHOR_END: mul-instructions

// ANCHOR: field-config
// The top-level config that provides all necessary columns and permutations
// for the other configs.
#[derive(Clone, Debug)]
struct FieldConfig {
    /// For this chip, we will use two advice columns to implement our instructions.
    /// These are also the columns through which we communicate with other parts of
    /// the circuit.
    advice: [Column<Advice>; 2],

    /// Public inputs
    instance: Column<Instance>,

    add_config: AddConfig,
    mul_config: MulConfig,
}
// ANCHOR END: field-config

// ANCHOR: add-config
#[derive(Clone, Debug)]
struct AddConfig {
    advice: [Column<Advice>; 2],
    s_add: Selector,
}
// ANCHOR_END: add-config

// ANCHOR: mul-config
#[derive(Clone, Debug)]
struct MulConfig {
    advice: [Column<Advice>; 2],
    s_mul: Selector,
}
// ANCHOR END: mul-config

// ANCHOR: field-chip
/// The top-level chip that will implement the `FieldInstructions`.
struct FieldChip<F: Field> {
    config: FieldConfig,
    _marker: PhantomData<F>,
}
// ANCHOR_END: field-chip

// ANCHOR: add-chip
struct AddChip<F: Field> {
    config: AddConfig,
    _marker: PhantomData<F>,
}
// ANCHOR END: add-chip

// ANCHOR: mul-chip
struct MulChip<F: Field> {
    config: MulConfig,
    _marker: PhantomData<F>,
}
// ANCHOR_END: mul-chip

// ANCHOR: add-chip-trait-impl
impl<F: Field> Chip<F> for AddChip<F> {
    type Config = AddConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}
// ANCHOR END: add-chip-trait-impl

// ANCHOR: add-chip-impl
impl<F: Field> AddChip<F> {
    fn construct(config: <Self as Chip<F>>::Config, _loaded: <Self as Chip<F>>::Loaded) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 2],
    ) -> <Self as Chip<F>>::Config {
        let s_add = meta.selector();

        // Define our addition gate!
        meta.create_gate("add", |meta| {
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let rhs = meta.query_advice(advice[1], Rotation::cur());
            let out = meta.query_advice(advice[0], Rotation::next());
            let s_add = meta.query_selector(s_add);

            vec![s_add * (lhs + rhs - out)]
        });

        AddConfig { advice, s_add }
    }
}
// ANCHOR END: add-chip-impl

// ANCHOR: add-instructions-impl
impl<F: Field> AddInstructions<F> for FieldChip<F> {
    type Num = Number<F>;
    fn add(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let config = self.config().add_config.clone();

        let add_chip = AddChip::<F>::construct(config, ());
        add_chip.add(layouter, a, b)
    }
}

impl<F: Field> AddInstructions<F> for AddChip<F> {
    type Num = Number<F>;

    fn add(
        &self,
        mut layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "add",
            |mut region: Region<'_, F>| {
                // We only want to use a single addition gate in this region,
                // so we enable it at region offset 0; this means it will constrain
                // cells at offsets 0 and 1.
                config.s_add.enable(&mut region, 0)?;

                // The inputs we've been given could be located anywhere in the circuit,
                // but we can only rely on relative offsets inside this region. So we
                // assign new cells inside the region and constrain them to have the
                // same values as the inputs.
                a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
                b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;

                // Now we can compute the addition result, which is to be assigned
                // into the output position.
                let value = a.0.value().copied() + b.0.value();

                // Finally, we do the assignment to the output, returning a
                // variable to be used in another part of the circuit.
                region
                    .assign_advice(|| "lhs + rhs", config.advice[0], 1, || value)
                    .map(Number)
            },
        )
    }
}
// ANCHOR END: add-instructions-impl

// ANCHOR: mul-chip-trait-impl
impl<F: Field> Chip<F> for MulChip<F> {
    type Config = MulConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}
// ANCHOR END: mul-chip-trait-impl

// ANCHOR: mul-chip-impl
impl<F: Field> MulChip<F> {
    fn construct(config: <Self as Chip<F>>::Config, _loaded: <Self as Chip<F>>::Loaded) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 2],
    ) -> <Self as Chip<F>>::Config {
        for column in &advice {
            meta.enable_equality(*column);
        }
        let s_mul = meta.selector();

        // Define our multiplication gate!
        meta.create_gate("mul", |meta| {
            // To implement multiplication, we need three advice cells and a selector
            // cell. We arrange them like so:
            //
            // | a0  | a1  | s_mul |
            // |-----|-----|-------|
            // | lhs | rhs | s_mul |
            // | out |     |       |
            //
            // Gates may refer to any relative offsets we want, but each distinct
            // offset adds a cost to the proof. The most common offsets are 0 (the
            // current row), 1 (the next row), and -1 (the previous row), for which
            // `Rotation` has specific constructors.
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let rhs = meta.query_advice(advice[1], Rotation::cur());
            let out = meta.query_advice(advice[0], Rotation::next());
            let s_mul = meta.query_selector(s_mul);

            // The polynomial expression returned from `create_gate` will be
            // constrained by the proving system to equal zero. Our expression
            // has the following properties:
            // - When s_mul = 0, any value is allowed in lhs, rhs, and out.
            // - When s_mul != 0, this constrains lhs * rhs = out.
            vec![s_mul * (lhs * rhs - out)]
        });

        MulConfig { advice, s_mul }
    }
}
// ANCHOR_END: mul-chip-impl

// ANCHOR: mul-instructions-impl
impl<F: Field> MulInstructions<F> for FieldChip<F> {
    type Num = Number<F>;
    fn mul(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let config = self.config().mul_config.clone();
        let mul_chip = MulChip::<F>::construct(config, ());
        mul_chip.mul(layouter, a, b)
    }
}

impl<F: Field> MulInstructions<F> for MulChip<F> {
    type Num = Number<F>;

    fn mul(
        &self,
        mut layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "mul",
            |mut region: Region<'_, F>| {
                // We only want to use a single multiplication gate in this region,
                // so we enable it at region offset 0; this means it will constrain
                // cells at offsets 0 and 1.
                config.s_mul.enable(&mut region, 0)?;

                // The inputs we've been given could be located anywhere in the circuit,
                // but we can only rely on relative offsets inside this region. So we
                // assign new cells inside the region and constrain them to have the
                // same values as the inputs.
                a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
                b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;

                // Now we can compute the multiplication result, which is to be assigned
                // into the output position.
                let value = a.0.value().copied() * b.0.value();

                // Finally, we do the assignment to the output, returning a
                // variable to be used in another part of the circuit.
                region
                    .assign_advice(|| "lhs * rhs", config.advice[0], 1, || value)
                    .map(Number)
            },
        )
    }
}
// ANCHOR END: mul-instructions-impl

// ANCHOR: field-chip-trait-impl
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
// ANCHOR_END: field-chip-trait-impl

// ANCHOR: field-chip-impl
impl<F: Field> FieldChip<F> {
    fn construct(config: <Self as Chip<F>>::Config, _loaded: <Self as Chip<F>>::Loaded) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 2],
        instance: Column<Instance>,
    ) -> <Self as Chip<F>>::Config {
        let add_config = AddChip::configure(meta, advice);
        let mul_config = MulChip::configure(meta, advice);

        meta.enable_equality(instance);

        FieldConfig {
            advice,
            instance,
            add_config,
            mul_config,
        }
    }
}
// ANCHOR_END: field-chip-impl

// ANCHOR: field-instructions-impl
impl<F: Field> FieldInstructions<F> for FieldChip<F> {
    type Num = Number<F>;

    fn load_constant(
        &self,
        mut layouter: impl Layouter<F>,
        value: F,
    ) -> Result<<Self as FieldInstructions<F>>::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "load constant",
            |mut region| {
                region
                    .assign_advice(|| "constant", config.advice[0], 0, || Value::known(value))
                    .map(Number)
            },
        )
    }

    fn load_private(
        &self,
        mut layouter: impl Layouter<F>,
        value: Value<F>,
    ) -> Result<<Self as FieldInstructions<F>>::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "load private",
            |mut region| {
                region
                    .assign_advice(|| "private input", config.advice[0], 0, || value)
                    .map(Number)
            },
        )
    }

    /// Returns `y = x**3 + x + 5`.
    fn cubic_expression(
        &self,
        layouter: &mut impl Layouter<F>,
        x: <Self as FieldInstructions<F>>::Num,
    ) -> Result<<Self as FieldInstructions<F>>::Num, Error> {
        // Calculate x^2
        let x_square = self.mul(layouter.namespace(|| "x * x"), x.clone(), x.clone())?;

        // Calculate x^3
        let x_cube = self.mul(layouter.namespace(|| "x^2 * x"), x_square, x.clone())?;

        // Calculate x^3 + x
        let x_cube_plus_x = self.add(layouter.namespace(|| "x^3 + x"), x_cube, x)?;
        let five = F::ONE + F::ONE + F::ONE + F::ONE + F::ONE;
        let constant_5 = self.load_constant(
            layouter.namespace(|| "load constant 5"),
            five
        )?;
        
        // Calculate y = x^3 + x + 5
        self.add(layouter.namespace(|| "x^3 + x + 5"), x_cube_plus_x, constant_5)
    }

    fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        num: <Self as FieldInstructions<F>>::Num,
        row: usize,
    ) -> Result<(), Error> {
        let config = self.config();

        layouter.constrain_instance(num.0.cell(), config.instance, row)
    }
}
// ANCHOR_END: field-instructions-impl

// ANCHOR: circuit
/// The full circuit implementation.
///
/// In this struct we store the private input variables. We use `Value<F>` because
/// they won't have any value during key generation. During proving, if any of these
/// were `Value::unknown()` we would get an error.
#[derive(Default)]
struct MyCircuit<F: Field> {
    x: Value<F>,
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
        FieldChip::configure(meta, advice, instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let field_chip = FieldChip::<F>::construct(config, ());

        // Load x as a private input.
        let x = field_chip.load_private(layouter.namespace(|| "load x"), self.x)?;

        // Compute y = x^3 + x + 5.
        let y = field_chip.cubic_expression(&mut layouter, x)?;

        // Expose the result y as a public input.
        field_chip.expose_public(layouter.namespace(|| "expose y"), y, 0)
    }
}
// ANCHOR_END: circuit

#[allow(clippy::many_single_char_names)]
fn main() {
    use halo2_proofs::{dev::MockProver, pasta::Fp};
    use rand_core::OsRng;

    let k = 4;
    let rng = OsRng;

    let x = Fp::random(rng);
    let y = x * x * x + x + Fp::from(5u64);

    let circuit = MyCircuit {
        x: Value::known(x),
    };

    let mut public_inputs = vec![y];

    // Given the correct public input, our circuit will verify.
    let start1 = Instant::now();
    let allocator_before = unsafe { GLOBAL_ALLOCATOR.as_ref().unwrap().get_total() };
    let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
    let allocator_after = unsafe { GLOBAL_ALLOCATOR.as_ref().unwrap().get_total() };
    let start2 = Instant::now();
    let verification_result = prover.verify();
    let start3 = Instant::now();
    let duration1 = start2.duration_since(start1);
    let duration2 = start3.duration_since(start2);
    let memory_used = allocator_after - allocator_before;
    let millis1 = duration1.as_secs_f64() * 1000.0;
    let millis2 = duration2.as_secs_f64() * 1000.0;
    println!("Prove time: {:.3} milliseconds", millis1);
    println!("Verify time: {:.3} milliseconds", millis2);
    println!("Memory used by prover: {} bytes", memory_used);
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
