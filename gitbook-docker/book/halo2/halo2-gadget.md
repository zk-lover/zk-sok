# HALO2 Gadget Documentation

## 1. RangeConstrained

### Core Functions:
RangeConstrained is a general range constraint gadget used to impose bit-width restrictions on field elements, ensuring their values fall within a specified bit-width range.

### Main Methods:
- `bitrange_of`: Extracts a specific bit range from a field element, returning a range-constrained value.
- `unsound_unchecked`: Constructs a range-constrained allocation unit (unchecked range constraint) for integration into existing circuits.
- `value`: Extracts the value of the range constraint.

### Example Code:
```rust
let constrained_value = RangeConstrained::bitrange_of(value, 0..8); // Extracts the lower 8 bits of the field element
```
## 2. bool_check

### Core Functions:
bool_check ensures that the expression value is a boolean (0 or 1).Uses `range_check` to ensure the input value is within the range [0, 2), i.e., 0 or 1.

Example Code
```rust
let boolean_expression = bool_check(expression);
```
## 3. ternary

### Core Functions:
ternary implements a ternary selection operation: if `a` then `b` else `c`.
- If `a` is a boolean, returns `b` or `c`:
  \[
  \text{result} = a \times b + (1 - a) \times c
  \]

### Example Code:
```rust
let selected = ternary(condition, value_if_true, value_if_false);
```
## 4. range_check

### Core Functions:
range_check ensures that the expression value is within the range [0, range).

### Example Code:
```rust
let checked_value = range_check(expression, 16); // Ensures the value is within the range [0, 16)
```

## 5. decompose_word

### Core Functions:
Decomposes a field element into windows of less than 8 bits.

### Example Code:
```rust
let decomposed = decompose_word(&field_elem, 256, 8); // Decomposes into 8-bit windows
```
## 6. Sha256

### Core Functions:
Sha256 is a high-level SHA-256 gadget that encapsulates the logic for initialization, data processing, and final digest computation.

### Main Methods:
- `new`: Initializes the SHA-256 circuit instance and loads the initial state.
- `update`: Updates the current state and processes input data, including:
  - Padding the current block.
  - Calling `compress` if the current block is full.
  - Caching unprocessed data.
- `finalize`: Pads the remaining data, calls `compress`, and generates the final digest.
- `digest`: A shortcut method that performs the complete process from initialization to digest generation.

### Gadget Properties:
Sha256 is a composite gadget that combines multiple basic modules (IV initialization, compression function, state transition, etc.) to construct a complete hash circuit.

### Example Code:
```rust
let mut hasher = Sha256::new(chip, layouter.namespace(|| "init"))?;
hasher.update(layouter.namespace(|| "update"), data)?;
let digest = hasher.finalize(layouter.namespace(|| "finalize"))?;
let digest = Sha256::digest(chip, layouter, &data)?;
```

## 7. ECC Gadget

ECC (Elliptic Curve Cryptography) Gadget is a key component in zero-knowledge proof systems, providing operations on elliptic curve points and scalars. Below are the core functionalities and example code analysis:

### Core Functions:
1. Loading Points and Scalars
   - `witness_point`: Loads an elliptic curve point into the circuit, including the identity point.
   - `witness_point_non_id`: Loads a non-identity point, ensuring it is not the identity point.
   - `witness_scalar_var` and `witness_scalar_fixed`: Loads scalars for variable-base and fixed-base scalar multiplication, respectively.
2. Point Addition
   - `add`: Complete point addition, supports the identity point.
   - `add_incomplete`: Incomplete point addition, optimized for performance but does not handle the identity point.
3. Scalar Multiplication
   - `mul`: Variable-base point multiplication, computes [scalar]P.
   - `mul_fixed`: Fixed-base point multiplication, efficiently computed using precomputed tables.
4. Point Equality Constraint
   - `constrain_equal`: Constrains two points to be equal in the circuit.
5. Auxiliary Functions
   - `extract_p`: Extracts the x-coordinate of a point.

## 8. Poseidon Hash Function Gadget

Poseidon is an algebraic hash function suitable for zero-knowledge proofs, based on a sponge construction. Below are the core functionalities of the Poseidon gadget:

### Core Functions:
1. Input Processing and Padding
   - `PaddedWord`: Represents hash input, including message and padding values.
   - `Absorbing` and `ConstantLength`: Used to absorb input and generate fixed-length output.
2. Sponge Operations
   - `add_input`: Adds input to the current state.
   - `permute`: Performs Poseidon permutation, mixing the current state.
   - `get_output`: Extracts the output from the sponge state.
3. Initialization and Hash Operations
   - `initial_state`: Initializes the state based on the specified domain.
   - `absorb`: Absorbs an element into the sponge.
   - `squeeze`: Squeezes a hash value from the sponge.
4. Main Hash Logic
   - `Sponge`: Provides complete sponge operations, including absorption, permutation, and squeezing.
   - `Hash`: Encapsulates high-level hash operations.

## 9. Sinsemilla Gadget

Sinsemilla is a hash function optimized for efficiency in circuit implementation, particularly suitable for hashing fixed-length bit segments. Below are the core functionalities of the Sinsemilla gadget:

### Core Functions:
1. Message Segmentation and Processing
   - `MessagePiece`: Represents a message segment composed of K bits, supporting construction from bit strings or field elements.
2. Hashing to Curve Point
   - `hash_to_point`: Hashes the message to a non-identity elliptic curve point, also outputting intermediate values (running sum).
3. Hashing and Commitment
   - `hash`: Extracts the x-coordinate of the hash result, used to generate a short hash.
   - `commit`: Performs a commitment operation on the message in a specific domain, generating a commitment value.
4. Domain Support
   - `HashDomain`: Defines the domain required for hash operations.
   - `CommitDomain`: Defines the domain supporting commitments, including randomization.