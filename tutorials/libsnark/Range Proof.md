## Range Proof

This code uses the libsnark library to implement a zero-knowledge proof system. It begins by initializing curve parameters and creating a protoboard. Several variables are defined for comparison operations. The code calculates a maximum value (2 raised to the power of 32) using a loop and employs a `comparison_gadget` to generate R1CS constraints. It then generates a keypair and assigns a secret value (18) to the variable `x`. A proof is generated and subsequently verified for its validity. Finally, the code outputs the number of R1CS constraints, the primary (public) input, the auxiliary (private) input, the verification status, and the time taken for proving and verification.

Below, we will divide the code into code blocks and annotate them.

### 1. Including from libsnark

```cpp
#include <stdlib.h>
#include <iostream>
#include <chrono>
#include "libsnark/zk_proof_systems/ppzksnark/r1cs_se_ppzksnark/r1cs_se_ppzksnark.hpp"
#include "libsnark/common/default_types/r1cs_se_ppzksnark_pp.hpp"
#include "libsnark/gadgetlib1/pb_variable.hpp"

using namespace libsnark;
using namespace std;
```

* The libsnark-related header files are explained in detail in the cubic_expression tutorial, so we won't elaborate on them here.

### 2. Circuit Construction

```cpp
int main() {
    // Disable detailed output
    libff::inhibit_profiling_info = true;
    
    typedef libff::Fr<default_r1cs_se_ppzksnark_pp> FieldT;

    // Initialize the curve parameters
    default_r1cs_se_ppzksnark_pp::init_public_params();
```

* `typedef libff::Fr<default_r1cs_se_ppzksnark_pp> FieldT;`: Defines `FieldT` as the finite field element type, where `Fr` is a template class in `libff` for finite fields.

* `default_r1cs_se_ppzksnark_pp::init_public_params();`: Initializes elliptic curve parameters, a prerequisite for using the `libsnark` library.

##### Creating and Setting Up the Circuit

```cpp
    // Create protoboard
    protoboard<FieldT> pb;

    // Define variables
    pb_variable<FieldT> x;
    pb_variable<FieldT> x_squared;
    pb_variable<FieldT> x_cubed;
    pb_variable<FieldT> out;

    // Allocate variables
    out.allocate(pb, "out");
    x.allocate(pb, "x");
    x_squared.allocate(pb, "x_squared");
    x_cubed.allocate(pb, "x_cubed");

    pb.set_input_sizes(1);
```

* **Protoboard Creation**:

  * `protoboard<FieldT> pb;`: Instantiates a `protoboard` object, which acts as a container for the circuit's variables and constraints. It is a key component in defining and managing the constraint system.

* **Variable Definition**:

  * `pb_variable<FieldT> x;` etc.: Declares variables that will be used in the circuit. Each variable represents a value in the finite field `FieldT`.

* **Variable Allocation**:

  * `out.allocate(pb, "out");` etc.: Allocates each variable on the `protoboard` and assigns it a unique identifier (e.g., "x", "x_squared"). This step is necessary to register the variables with the `protoboard` and prepare them for constraint definition.

* **Input Size Setting**:

  * `pb.set_input_sizes(1);`: Specifies that the first variable (`x`) is a public input to the circuit. This is important for the proof system to distinguish between public inputs and private witness values.

##### Adding Constraints

```cpp
    // Add constraints
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x, x, x_squared));
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x_squared, x, x_cubed));
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x_cubed + x + 1, 1, out));
```

* **Constraint Addition**:

  * `pb.add_r1cs_constraint(...)`: Adds constraints to the `protoboard`. Each constraint is an instance of `r1cs_constraint`, which represents a relation of the form `A * B = C` in the finite field.

* **Specific Constraints**:

  * `r1cs_constraint<FieldT>(x, x, x_squared)`: Represents the constraint `x * x = x_squared`, ensuring that `x_squared` is the square of `x`.

  * `r1cs_constraint<FieldT>(x_squared, x, x_cubed)`: Represents the constraint `x_squared * x = x_cubed`, ensuring that `x_cubed` is the cube of `x`.

  * `r1cs_constraint<FieldT>(x_cubed + x + 1, 1, out)`: Represents the constraint `x_cubed + x + 1 = out`, ensuring that `out` is the result of the expression `x^3 + x + 1`.

### 3. ZK Proof Generation and Verification

##### Generating Keypair and Proof

```cpp
    auto clock1 = std::chrono::high_resolution_clock::now();
    
    const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();
    const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(constraint_system);

    // Add witness values
    pb.val(x) = 3;
    pb.val(x_squared) = pb.val(x) * pb.val(x);
    pb.val(x_cubed) = pb.val(x_squared) * pb.val(x);
    pb.val(out) = pb.val(x_cubed) + pb.val(x) + FieldT(1);

    const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(
            keypair.pk, pb.primary_input(), pb.auxiliary_input());
    auto clock2 = std::chrono::high_resolution_clock::now();
    auto proving_time = std::chrono::duration_cast<std::chrono::milliseconds>(clock2 - clock1).count();
```

* **Timing Start**:

  * `auto clock1 = std::chrono::high_resolution_clock::now();`: Captures the current time to measure how long the proof generation takes.

* **Constraint System Extraction**:

  * `const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();`: Extracts the constraint system from the `protoboard`, which includes all defined variables and constraints.

* **Keypair Generation**:

  * `const r1cs_se_ppzksnark_keypair<...> keypair = r1cs_se_ppzksnark_generator<...>(constraint_system);`: Generates a keypair consisting of a proving key and a verification key. This is a crucial step in setting up the zk-SNARK system.

* **Witness Assignment**:

  * `pb.val(x) = 3;` etc.: Assigns specific values to the variables, known as witness values. These values satisfy the constraints and are used to generate the proof.

* **Proof Generation**:

  * `const r1cs_se_ppzksnark_proof<...> proof = r1cs_se_ppzksnark_prover<...>(keypair.pk, pb.primary_input(), pb.auxiliary_input());`: Generates a zk-SNARK proof using the proving key and the assigned witness values. The proof can be used to verify that the witness satisfies the constraints without revealing the witness itself.

* **Timing End and Calculation**:

  * `auto clock2 = std::chrono::high_resolution_clock::now();`: Captures the time after proof generation.

  * `auto proving_time = std::chrono::duration_cast<std::chrono::milliseconds>(clock2 - clock1).count();`: Calculates the time taken to generate the proof in milliseconds.

##### Verifying the Proof

```cpp
    auto clock3 = std::chrono::high_resolution_clock::now();
    bool verified = r1cs_se_ppzksnark_verifier_strong_IC<default_r1cs_se_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof);
    auto clock4 = std::chrono::high_resolution_clock::now();
    auto verification_time = std::chrono::duration_cast<std::chrono::milliseconds>(clock4 - clock3).count();
```

* **Verification Timing Start**:

  * `auto clock3 = std::chrono::high_resolution_clock::now();`: Captures the current time to measure how long the verification process takes.

* **Proof Verification**:

  * `bool verified = r1cs_se_ppzksnark_verifier_strong_IC<...>(keypair.vk, pb.primary_input(), proof);`: Verifies the proof using the verification key. The function checks if the proof is valid for the given public inputs and returns a boolean result (`true` if valid, `false` otherwise).

* **Verification Timing End and Calculation**:

  * `auto clock4 = std::chrono::high_resolution_clock::now();`: Captures the time after verification.

  * `auto verification_time = std::chrono::duration_cast<std::chrono::milliseconds>(clock4 - clock3).count();`: Calculates the time taken to verify the proof in milliseconds.

##### Outputting Results

```cpp
    cout << "Number of constraints: " << constraint_system.num_constraints() << endl;
    cout << "Proof size (bytes): " << proof.size_in_bits()/8 << endl;
    cout << "Proving time (ms): " << proving_time << endl;
    cout << "Verification time (ms): " << verification_time << endl;

    return 0;
}
```

