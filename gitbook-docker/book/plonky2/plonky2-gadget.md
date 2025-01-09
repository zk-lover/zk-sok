# PLONKY2 Gadget Documentation

## 1. permute and permute_swapped

`permute` and `permute_swapped` are core methods for implementing algebraic permutations in circuits. They are commonly used in cryptographic operations, such as hash function implementations or Merkle tree verifications.

### Core Functions:
- **permute**: Directly applies the algebraic permutation to the input without swapping.
- **permute_swapped**: Decides whether to swap parts of the input based on a boolean flag before performing the algebraic permutation.
### Example Code:
```rust
let mut builder = CircuitBuilder::<F, 2>::new(cs);
let inputs = vec![builder.constant(F::from(1)), builder.constant(F::from(2))];

// Directly perform algebraic permutation
let permuted = builder.permute::<PoseidonHasher<F>>(inputs);
```

## 2. interpolate_coset

`interpolate_coset` is a method for performing coset interpolation in circuits. It interpolates a polynomial based on a given set of points and their values, and computes the value of the polynomial at a specific point. This is crucial in polynomial commitments and zero-knowledge proofs, especially when verifying FFT results or polynomial evaluations.

### Core Functions:
- **Interpolate Polynomial**: Interpolates a polynomial based on a set of points and corresponding values.
- **Compute Polynomial Value**: Returns the evaluation of the interpolated polynomial at a given point.

### Example Code:
```rust
let mut builder = CircuitBuilder::<F, 2>::new(config);
let coset_shift = builder.constant(F::rand());
let values = vec![builder.constant_extension(FF::rand()); 4]; // Example values
let evaluation_point = builder.constant_extension(FF::rand());

let eval = builder.interpolate_coset(
    CosetInterpolationGate::with_max_degree(2, 4),
    coset_shift,
    &values,
    evaluation_point,
);
```
## 3. add_lookup_table_from_pairs, add_lookup_table_from_table, and add_all_lookups

These methods implement the construction of lookup tables in circuits, used to match inputs with predefined lookup tables and generate corresponding outputs. This is useful in cryptography and polynomial constraints, such as verifying the nonlinear parts of hash functions.

### Core Functions:
- **add_lookup_table_from_pairs**: Adds a lookup table from (input, output) pairs.
- **add_lookup_table_from_table**: Adds a lookup table from input and output arrays.
- **add_lookup_table_from_fn**: Generates a lookup table from a function and input array.
- **add_lookup_from_index**: Looks up the output corresponding to an input in the circuit.
- **add_all_lookups**: Integrates all lookup table-related operations into the circuit.

### Example Code:
```rust
// Initialize CircuitBuilder
let mut builder = CircuitBuilder::<F, 2>::new(config);

// Add lookup table
let table = LookupTable::new(vec![1, 2, 3], vec![4, 5, 6]);
let lut_index = builder.add_lookup_table_from_pairs(table);

// Lookup corresponding output in the circuit
let input = builder.constant(F::from(2));
let output = builder.add_lookup_from_index(input, lut_index);
```

## 4. range_check, low_bits, and split_low_high

These methods implement range checks and bit decomposition in circuits, commonly used to construct numerical constraints in zero-knowledge proofs. They allow verification and decomposition of numerical values into bits or ranges.

### Core Functions:
- **range_check**: Verifies if the input `x` is less than `2^{n_log}`.
- **low_bits**: Retrieves the lowest `num_low_bits` bits of the input `x`.
- **split_low_high**: Splits the input `x` into `low` and `high` parts, ensuring `x = low + 2^{n_log} * high`, with `low` and `high` within specified ranges.
- **assert_bool**: Verifies if the input is a boolean value (0 or 1).

### Example Code:
```rust
let mut builder = CircuitBuilder::<F, 2>::new(config);

// Create a virtual target
let x = builder.add_virtual_target();

// Perform range check, verify x < 16
builder.range_check(x, 4);

// Retrieve the lowest 3 bits of x
let low_bits = builder.low_bits(x, 3, 8);

// Split x into low and high parts
let (low, high) = builder.split_low_high(x, 4, 8);
```
## 5. PolynomialCoeffsExtTarget and PolynomialCoeffsExtAlgebraTarget

These structures are used to handle extended polynomial coefficients in circuits and provide polynomial evaluation at specific points. They are key tools for efficiently implementing polynomial operations in zero-knowledge proof circuits.

### Core Functions:
1. **PolynomialCoeffsExtTarget**:
   - **For polynomial operations on extended fields**.
   - **eval_scalar**: Evaluates at a scalar point `Target`.
   - **eval**: Evaluates at an extended point `ExtensionTarget`.

2. **PolynomialCoeffsExtAlgebraTarget**:
   - **For polynomial operations on extended algebra**.
   - **eval_scalar**: Evaluates at an extended scalar point `ExtensionTarget`.
   - **eval**: Evaluates at an extended algebra point `ExtensionAlgebraTarget`.
   - **eval_with_powers**: Evaluates the polynomial using precomputed point powers.

### Example Code:
```rust
let mut builder = CircuitBuilder::<F, 2>::new(config);

// Create extended polynomial coefficients
let coeffs = PolynomialCoeffsExtTarget(vec![
    builder.constant_extension(FF::from(1)),
    builder.constant_extension(FF::from(2)),
    builder.constant_extension(FF::from(3)),
]);

// Evaluate at a scalar point
let point = builder.constant(F::from(5));
let result = coeffs.eval_scalar(&mut builder, point);

// Evaluate at an extended point
let ext_point = builder.constant_extension(FF::from(5));
let result_ext = coeffs.eval(&mut builder, ext_point);
```

## 6. select and select_ext

These methods provide conditional selection in circuits (similar to the ternary operator `if b { x } else { y }`), supporting both scalar and extended fields. These operations are crucial in circuit optimization and cryptographic applications, especially when branch logic is needed.


### Example Code:
```rust
let mut builder = CircuitBuilder::<F, 2>::new(config);

// Scalar conditional selection
let b = builder._true();
let x = builder.constant(F::from(5));
let y = builder.constant(F::from(10));
let result = builder.select(b, x, y); // Returns x because b = true

// Extended field conditional selection
let xt = builder.add_virtual_extension_target();
let yt = builder.add_virtual_extension_target();
let result_ext = builder.select_ext(b, xt, yt);
```
