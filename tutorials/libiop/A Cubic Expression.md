## A Cubic Expression

### 1. Circuit Setup

```cpp
#include <libff/algebra/curves/edwards/edwards_pp.hpp>
#include "libiop/snark/aurora_snark.hpp"
#include "libiop/relations/examples/r1cs_examples.hpp"

using namespace libiop;
using namespace std;

libff::inhibit_profiling_info = true;
libff::inhibit_profiling_counters = true;

libff::edwards_pp::init_public_params();
typedef libff::edwards_Fr FieldT;

const size_t num_constraints = 1 << 2;
const size_t num_inputs = (1 << 2) - 1;
const size_t num_variables = (1 << 2) - 1;
const size_t security_parameter = 128;

r1cs_example<FieldT> r1cs_params = generate_r1cs_example<FieldT>(num_constraints, num_inputs, num_variables);

if (!r1cs_params.constraint_system_.is_satisfied(r1cs_params.primary_input_, r1cs_params.auxiliary_input_)) {
    return 1;  
}
```
Initialized the Edwards curve.
A simple R1CS instance was generated using the generate_r1cs_example method in the libiop library.

num_constraints is the number of constraints for the circuit. The circuit constraints for cubic_expression built in libsnark are 3, but the number of circuits supported by Aurora and Fractal must be a power of 2, so it is set to 4 here.

### 2. Proof Generation and Verification

Proof Generation
```cpp

aurora_snark_parameters<FieldT, hash_type> params(
    security_parameter, 
    LDT_reducer_soundness_type::optimistic_heuristic,
    FRI_soundness_type::heuristic,
    blake2b_type,
    3, 
    2, 
    true, 
    multiplicative_coset_type,
    num_constraints,
    num_variables);

auto proving_start = chrono::high_resolution_clock::now();
const aurora_snark_argument<FieldT, hash_type> argument = aurora_snark_prover<FieldT>(
    r1cs_params.constraint_system_,
    r1cs_params.primary_input_,
    r1cs_params.auxiliary_input_,
    params);
auto proving_end = chrono::high_resolution_clock::now();
auto proving_time = chrono::duration_cast<chrono::milliseconds>(proving_end - proving_start).count();
```
Set SNARK-related parameters, including security and localization parameters.
Call aurora_snark_prover to generate a SNARK proof and record the generation time.

Verification
```cpp  
auto verify_start = chrono::high_resolution_clock::now();
const bool verification_result = aurora_snark_verifier<FieldT, hash_type>(
    r1cs_params.constraint_system_,
    r1cs_params.primary_input_,
    argument,
    params);
auto verify_end = chrono::high_resolution_clock::now();
auto verify_time = chrono::duration_cast<chrono::milliseconds>(verify_end - verify_start).count();
```

Use aurora_snark_verifier to verify that the generated proof is valid and record the verification time.