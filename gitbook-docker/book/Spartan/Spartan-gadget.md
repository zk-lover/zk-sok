# SPARTAN-GADGET

## 1. ProductCircuit

### Core Functionality

ProductCircuit is responsible for constructing a multi-layer polynomial product circuit. Each layer generates a new polynomial by multiplying terms, and ultimately outputs a scalar result. It is mainly used in scenarios involving multi-layer polynomial calculations, such as recursive verification or accumulation operations in complex computation paths.

### Example Code
```rust
// Construct a polynomial circuit
let poly = DensePolynomial::from(vec![Scalar::from(1), Scalar::from(2), Scalar::from(3), Scalar::from(4)]);
let circuit = ProductCircuit::new(&poly);

// Evaluate the circuit's final result
let result = circuit.evaluate();
println!("Evaluation result: {:?}", result);
```
## 2. DotProductCircuit

### Core Functionality

DotProductCircuit implements a dot product circuit that multiplies two polynomials term by term, multiplies by a weight polynomial, and then sums the results. It is suitable for weighted dot product calculations or vectorized scenarios.

### Example Code
```rust
// Construct a dot product circuit
let left = DensePolynomial::from(vec![Scalar::from(1), Scalar::from(2)]);
let right = DensePolynomial::from(vec![Scalar::from(3), Scalar::from(4)]);
let weight = DensePolynomial::from(vec![Scalar::from(1), Scalar::from(1)]);
let circuit = DotProductCircuit::new(left, right, weight);

// Evaluate the dot product result
let result = circuit.evaluate();
println!("Dot product result: {:?}", result);
```

## 3. Layers

### Core Functionality

Layers is responsible for constructing hash function-based circuits for each layer of sparse polynomials, used for verifying read and write operations and auditing processes of sparse polynomials. It uses ProductCircuit to perform hash calculations for initialization, read, write, and audit polynomials.

### Example Code
```rust
let eval_table = vec![Scalar::one(); num_cells];
let addr_timestamps = AddrTimestamps::new(num_cells, num_ops, ops_addr);
let poly_ops_val = vec![DensePolynomial::random(num_ops); num_instances];
let r_mem_check = (Scalar::random(&mut csprng), Scalar::random(&mut csprng));

let layers = Layers::new(&eval_table, &addr_timestamps, &poly_ops_val, &r_mem_check);
```

## 4. PolyEvalNetwork

### Core Functionality

PolyEvalNetwork constructs a polynomial evaluation network for rows and columns, verifying the addresses stored in memory. Each network contains a set of Layers used to verify the legality of sparse polynomial operations.

### Example Code
```rust
let dense = MultiSparseMatPolynomialAsDense::from_sparse(&sparse_polys);
let derefs = dense.deref(&mem_rx, &mem_ry);
let r_mem_check = (Scalar::random(&mut csprng), Scalar::random(&mut csprng));

let network = PolyEvalNetwork::new(&dense, &derefs, &mem_rx, &mem_ry, &r_mem_check);
```

