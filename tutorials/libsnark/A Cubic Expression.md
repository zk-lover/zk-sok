## A Cubic Expression

This code implements a simple zero-knowledge proof system using the libsnark library. It defines an algebraic circuit with the formula x^3 + x + 1 = out and describes this circuit using R1CS (Rank-1 Constraint System) constraints. The code initializes curve parameters and creates a protoboard to manage variables. It allocates variables and adds constraints to represent the circuit logic. Then, it performs a trusted setup to generate a keypair and creates a proof for a given input value (e.g, x = 3). Finally, the code verifies the proof's validity and outputs the verification result and time taken. It also demonstrates a case where verification fails when the output value is incorrect.

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

* **Libsnark Libraries**:

  * `#include "libsnark/zk_proof_systems/ppzksnark/r1cs_se_ppzksnark/r1cs_se_ppzksnark.hpp"`: Includes the implementation of the zk-SNARK proof system for Rank-1 Constraint Systems (R1CS) using the SE-ppzkSNARK protocol.

  * `#include "libsnark/common/default_types/r1cs_se_ppzksnark_pp.hpp"`: Provides default types and parameters for the SE-ppzkSNARK protocol.

  * `#include "libsnark/gadgetlib1/pb_variable.hpp"`: Includes the protoboard variable class, which is used to define variables in the constraint system.

* **Namespaces**:

  * `using namespace libsnark;`: Allows direct access to `libsnark` classes and functions without prefixing them with `libsnark::`.

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

* `protoboard<FieldT> pb;`: Creates a `protoboard` object `pb`, which represents the circuit. A `protoboard` is a container for circuit variables and constraints.

* `pb_variable<FieldT> x;` etc.: Defines variables in the circuit. `pb_variable` is a class in `libsnark` for representing circuit variables.

* `out.allocate(pb, "out");` etc.: Allocates space for each variable and registers them with the `protoboard`. The `allocate` method associates the variable with the `protoboard` and assigns it a unique index.

* `pb.set_input_sizes(1);`: Sets the number of public inputs to 1, indicating that the first variable `x` is a public input.

##### Adding Constraints

```cpp
    // Add constraints
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x, x, x_squared));
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x_squared, x, x_cubed));
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x_cubed + x + 1, 1, out));
```

* `pb.add_r1cs_constraint(...)`: Adds constraints to the `protoboard`. Each constraint is an `r1cs_constraint` object representing a multiplication constraint.

* `r1cs_constraint<FieldT>(x, x, x_squared)`: Represents `x * x = x_squared`, meaning `x` squared equals `x_squared`.

* `r1cs_constraint<FieldT>(x_squared, x, x_cubed)`: Represents `x_squared * x = x_cubed`, meaning `x` cubed equals `x_cubed`.

* `r1cs_constraint<FieldT>(x_cubed + x + 1, 1, out)`: Represents `x_cubed + x + 1 = out`, meaning `x` cubed plus `x` plus 1 equals `out`.

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

* `auto clock1 = std::chrono::high_resolution_clock::now();`: Records the start time for proof generation.

* `const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();`: Extracts the constraint system from the `protoboard`.

* `const r1cs_se_ppzksnark_keypair<...> keypair = r1cs_se_ppzksnark_generator<...>(constraint_system);`: Generates a keypair, including a proving key and a verification key.

* `pb.val(x) = 3;` etc.: Sets witness values, with `x` set to 3, and calculates the values for other variables.

* `const r1cs_se_ppzksnark_proof<...> proof = r1cs_se_ppzksnark_prover<...>(keypair.pk, pb.primary_input(), pb.auxiliary_input());`: Generates a proof using the proving key and inputs.

* `auto clock2 = std::chrono::high_resolution_clock::now();`: Records the end time for proof generation.

* `auto proving_time = std::chrono::duration_cast<std::chrono::milliseconds>(clock2 - clock1).count();`: Calculates the time taken to generate the proof.

##### Verifying the Proof

```cpp
    auto clock3 = std::chrono::high_resolution_clock::now();
    bool verified = r1cs_se_ppzksnark_verifier_strong_IC<default_r1cs_se_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof);
    auto clock4 = std::chrono::high_resolution_clock::now();
    auto verification_time = std::chrono::duration_cast<std::chrono::milliseconds>(clock4 - clock3).count();
```

* `auto clock3 = std::chrono::high_resolution_clock::now();`: Records the start time for verification.

* `bool verified = r1cs_se_ppzksnark_verifier_strong_IC<...>(keypair.vk, pb.primary_input(), proof);`: Verifies the proof using the verification key and returns the result.

* `auto clock4 = std::chrono::high_resolution_clock::now();`: Records the end time for verification.

* `auto verification_time = std::chrono::duration_cast<std::chrono::milliseconds>(clock4 - clock3).count();`: Calculates the time taken to verify the proof.

##### Outputting Results

```cpp
    cout << "Number of constraints: " << constraint_system.num_constraints() << endl;
    cout << "Proof size (bytes): " << proof.size_in_bits()/8 << endl;
    cout << "Proving time (ms): " << proving_time << endl;
    cout << "Verification time (ms): " << verification_time << endl;

    return 0;
}
```

