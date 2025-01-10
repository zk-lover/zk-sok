# Plonky2 Circuit Documentation

plonky2 provides a modular framework for building zero-knowledge proof circuits, allowing developers to define constraints using Circuit features, and circuits are composed of gadgets and constraints, enabling flexible and scalable circuit design. The framework supports efficient proof generation, is optimized for large-scale applications, and includes features such as recursive proofs and polynomial commitment schemes for high-performance zk-SNARKs.

In this section, we will introduce the circuit building APIs of plonky2, including CircuitBuilder, PartialWitness, and CircuitConfig.

## 1.CircuitBuilder

**Purpose:** Used to construct zero-knowledge proof circuits, including defining circuit operations, inputs, and outputs.

**Components:**
- **Generic Parameters:** `F` (field type), `D` (degree of polynomial extension).
- **Methods:** Used to add virtual targets, perform arithmetic operations, register public inputs, etc.

**Related Functions:**  
1.1. **new(config: CircuitConfig)**
   - **Input:** `CircuitConfig` (circuit configuration).
   - **Output:** A new `CircuitBuilder` instance.
   - **Purpose:** Initializes the circuit builder, providing a foundation for subsequent circuit design.

1.2. **add_virtual_target()**
   ```
   pub fn add_virtual_target(&mut self) -> Target;
   ```
   - **Input:** None.
   - **Output:** Returns a `Target`.
   - **Purpose:** Adds a virtual variable as a placeholder in the circuit.
   - **Example**:
      ```rust
      let target = builder.add_virtual_target();
      ```
1.3. **mul(a: Target, b: Target)**
   - **Input:** Two targets `a` and `b`.
   - **Output:** Returns the target of the multiplication result.
   - **Purpose:** Computes `a * b` and stores the result in the circuit.
   - **Example**:
      ```rust
      let result = builder.mul(target_a, target_b);
      ```
1.4. **add(a: Target, b: Target)**
   - **Input:** Two targets `a` and `b`.
   - **Output:** Returns the target of the addition result.
   - **Purpose:** Computes `a + b`.
   - **Example**:
      ```rust
      let result = builder.add(target_a, target_b);
      ```
1.5. **add_const(target: Target, constant: F)**
   - **Input:** Target `target` and constant `constant`.
   - **Output:** Returns the target after adding the constant.
   - **Purpose:** Adds a constant to the target.
   - **Example**:
      ```rust
      let result = builder.add_const(target, constant);
      ```
1.6. **register_public_input(target: Target)**
   ```
   pub fn register_public_input(&mut self, target: Target);
   ```
   - **Input:** A target.
   - **Output:** None.
   - **Purpose:** Registers the target value as a public input.
   - **Example**:
      ```rust
      builder.register_public_input(target);
      ```
## 2.PartialWitness

**Purpose:** Used to store specific values of virtual targets during circuit execution.

**Components:**
- Virtual targets and their corresponding values.

**Related Functions:**
1. **new()**
   - **Input:** None.
   - **Output:** An empty `PartialWitness` instance.
   - **Purpose:** Initializes a partial witness for assigning values to virtual targets.

2. **set_target(target: Target, value: F)**
   ```
   fn set_target(&mut self, target: Target, value: F) -> Result<()>
   ```
   - **Input:** Target `target` and its value `value`.
   - **Output:** None.
   - **Purpose:** Assigns a value to the target `target`.
   - **Example**:
      ```rust
      witness.set_target(target, value);
      ```
## 3.CircuitConfig

**Purpose:** Configures specific parameters of the circuit, such as circuit type and zero-knowledge settings.

**Components:**
- Circuit type (e.g., standard zero-knowledge recursive configuration).

**Related Functions:**
1. **standard_recursion_zk_config()**
   ```
   pub fn standard_recursion_zk_config() -> Self;
   ```
   - **Input:** None.
   - **Output:** A standard zero-knowledge recursive circuit configuration.
   - **Purpose:** Provides a default zero-knowledge circuit configuration.
   - **Example**:
      ```rust
      let config = CircuitConfig::standard_recursion_zk_config();
      ```