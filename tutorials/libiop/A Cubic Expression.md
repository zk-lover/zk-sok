## A Cubic Expression

### 1. Aurora ZKP systems

#### Circuit Setup

```Objective-C++
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

#### Proof Generation and Verification

Proof Generation

```Objective-C++
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

```Objective-C++
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

### 2. Fractal ZKP systems

Below, we will demonstrate the differences between cubic_expression under Fractal ZKP systems and cubic_expression under Aurora ZKP systems.

##### Setting of Circuit Parameters

One of the difference from cubic_expression under Aurora ZKP systems is the setting of circuit parameters.

```Objective-C++
const size_t num_constraints = 1 << 6;  
const size_t num_inputs = (1 << 5)-1;    
const size_t num_variables = (1 << 6)-1;   
const size_t security_parameter = 128;
const size_t RS_extra_dimensions = 3;
const size_t FRI_localization_parameter = 3;
const LDT_reducer_soundness_type ldt_reducer_soundness_type = LDT_reducer_soundness_type::optimistic_heuristic;
const FRI_soundness_type fri_soundness_type = FRI_soundness_type::heuristic;
const field_subset_type domain_type = multiplicative_coset_type;
```

num_constraints is the number of constraints for the circuit. The circuit constraints for cubic_expression built in libsnark are 3, but the number of circuits supported by Aurora and Fractal must be a power of 2, so it is set to 64 here.

##### SNARK Parameters Setup

This program uses `fractal_snark_parameters` and requires index generation.

```Objective-C++
fractal_snark_parameters<FieldT, hash_type> params(
    security_parameter,
    ldt_reducer_soundness_type,
    fri_soundness_type,
    blake2b_type,
    FRI_localization_parameter,
    RS_extra_dimensions,
    make_zk,
    domain_type,
    cs);
// ......
std::pair<bcs_prover_index<FieldT, hash_type>, bcs_verifier_index<FieldT, hash_type>> index =
    fractal_snark_indexer(params);
```

##### Proof Generation and Verification

This program uses `fractal_snark_prover` and `fractal_snark_verifier`, and requires the use of the previously generated index.

```Objective-C++
const fractal_snark_argument<FieldT, hash_type> argument = fractal_snark_prover<FieldT, hash_type>(
    index.first,
    r1cs_params.primary_input_,
    r1cs_params.auxiliary_input_,
    params);
// ......
const bool verification_result = fractal_snark_verifier<FieldT, hash_type>(
    index.second,
    r1cs_params.primary_input_,
    argument,
    params);
```

### 3. Ligero ZKP systems

Also, we will demonstrate the differences between cubic_expression under Ligero ZKP systems and cubic_expression under Aurora ZKP systems.

##### Setting of Circuit Parameters

One of the difference from cubic_expression under Aurora ZKP systems is the setting of circuit parameters.

```Objective-C++
const size_t num_constraints = 3;
const size_t constraint_dim = 2;
const size_t num_inputs = 1;
const size_t num_variables = 3 - 1;
```

num_constraints is the number of constraints for the circuit. The circuit constraints for cubic_expression built in libsnark are 3, it is set to 3 here.

##### Set Ligero SNARK Parameters

```Objective-C++
ligero_snark_parameters<FieldT, binary_hash_digest> parameters;
parameters.security_level_ = 128;
parameters.height_width_ratio_ = 0.001;
parameters.RS_extra_dimensions_ = 2;
parameters.make_zk_ = true;
parameters.domain_type_ = multiplicative_coset_type;
parameters.LDT_reducer_soundness_type_ = LDT_reducer_soundness_type::proven;
parameters.bcs_params_ = default_bcs_params<FieldT, binary_hash_digest>(
       blake2b_type, parameters.security_level_, constraint_dim);
```

##### Use the ligero_snark_prover and ligero_snark_verifier functions

```Objective-C++
const ligero_snark_argument<FieldT, binary_hash_digest> argument = 
   ligero_snark_prover<FieldT, binary_hash_digest>(
       r1cs_params.constraint_system_,
       r1cs_params.primary_input_,
       r1cs_params.auxiliary_input_,
       parameters);
// ......
const bool verification_result = ligero_snark_verifier<FieldT, binary_hash_digest>(
    r1cs_params.constraint_system_,
    r1cs_params.primary_input_,
    argument,
    parameters);
```

