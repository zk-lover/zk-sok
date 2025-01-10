# snarkjs Circuit Documentation  

Unlike other zero-knowledge proof libraries, snarkjs uses the Circom language to write circuits instead of traditional programming languages like Go, C++ or Rust.
snarkjs provides powerful command-line tools that make it easy for developers to generate and verify zero-knowledge proofs using circuits written in Circom. Through these command-line tools, users can easily perform circuit compilation, key generation, proof generation and verification operations. Common commands include:
```
snarkjs compile: Compiles Circom circuit files into R1CS constraint systems and generates key files suitable for proofs.
snarkjs setup: Generates trusted setup, typically used to generate verification keys and proving keys.
snarkjs prove: Generates zero-knowledge proofs based on inputs and circuit constraints.
snarkjs verify: Verifies the correctness of zero-knowledge proofs, ensuring proofs are consistent with circuit constraints.
```
Circom is a language designed specifically for zero-knowledge proof circuits, with concise and intuitive syntax similar to C++ but more readable. It focuses on describing mathematical constraints in circuits while avoiding the complexity of low-level implementation. 

Next, we'll briefly introduce Circom's syntax, structure and API to help developers quickly get started with writing zero-knowledge proof circuits using Circom.

## 1. Basic Structure

Circom code consists of multiple **templates** and the **main circuit**. Each template defines the logic of a circuit, and templates can be instantiated in the main circuit or other templates.

### Template Declaration

```circom
template MyTemplate() {
    // Logic inside the circuit
}
```
### Main Circuit
``` circom
template MainCircuit() {
    signal input a;
    signal input b;
    signal output c;

    component myTemplate1 = MyTemplate();
    myTemplate1.a <== a;
    myTemplate1.b <== b;
    c <== myTemplate1.out;
}
component main = MainCircuit();
```

## 2. Signals

Signals are the inputs, outputs, and intermediate variables of a circuit. In Circom, all signals must be explicitly declared.

- **Input signal (input)**: The input variable of the circuit
- **Output signal (output)**: The output variable of the circuit
- **Intermediate signal**: The variable used for internal computation in the circuit

```
signal input a; // Input signal a
signal output b; // Output signal b
signal x; // Intermediate signal x
```

## 3. Constraints

Circom circuits are composed of a series of constraints, which are used to ensure that the inputs satisfy specific relationships. Circom uses a syntax similar to mathematical formulas to define constraints.

### Basic Constraints

You can define relationships between signals and constants, or between signals.

```circom
signal a;
signal b;
signal c;

a <== b + 1; // Constraint a = b + 1
c <== a * a; // Constraint c = a * a
```

### Assertion Constraints (assert)
Used to check if the circuit constraints hold true.
```
assert(a == b + 1); // Ensure a = b + 1
```

## 4. Arithmetic Operations

Circom allows basic arithmetic operations (addition, subtraction, multiplication, division) . Each `<===` can only contain one multiplication, as required by the R1CS (Rank-1 Constraint System); multiple multiplications can be split into separate calculations.

### Example

```circom
signal x, y, z;

x <== y + 5;    // Addition
z <== x * y;    // Multiplication
```

## 5. Template Instantiation

Templates are the core structure in Circom. A template can be instantiated as one or more components, and input signals within the template can be assigned values during instantiation.

### Example

```circom
template Adder() {
    signal input a;
    signal input b;
    signal output c;
    c <== a + b;
}

template Main() {
    signal input x;
    signal input y;
    signal output z;

    component adder1 = Adder(); // Instantiate the Adder template
    adder1.a <== x;
    adder1.b <== y;
    z <== adder1.c; // Get the result of the addition
}
```

## 6. Import and Modularity

Circom supports dividing the circuit into multiple files. You can include other Circom files using the `include` keyword.

### Example
```
include "somefile.circom";
```

## 7. Compilation

Once the circuit definition is complete, the Circom tool is typically used to compile the circuit into a form that can be used in a zk-SNARK system. The compilation process usually involves converting the circuit file into an R1CS file (Rank-1 Constraint System), and further generating the proof and verification algorithms.

### Example
```
circom my_circuit.circom –r1cs –wasm –sym
```
- --r1cs: Generate constraints in R1CS format
- --wasm: Generate a WebAssembly module for generating proofs
- --sym: Generate a symbol table to assist with debugging
