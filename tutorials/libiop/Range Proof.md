## Range Proof

### 1. Aurora ZKP systems

The only difference from cubic_expression under  Aurora ZKP systems is the setting of circuit parameters.

```Objective-C++
const size_t num_constraints = 1 << 6;
const size_t num_inputs = (1 << 5) - 1;
const size_t num_variables = (1 << 6) - 1;
const size_t security_parameter = 128;
const size_t RS_extra_dimensions = 2;
const size_t FRI_localization_parameter = 3;
const LDT_reducer_soundness_type ldt_reducer_soundness_type = LDT_reducer_soundness_type::optimistic_heuristic;
const FRI_soundness_type fri_soundness_type = FRI_soundness_type::heuristic;
const field_subset_type domain_type = multiplicative_coset_type;
```

num_constraints is the number of constraints for the circuit. The circuit constraints for rangeproof built in libsnark are 39, but the number of circuits supported by Aurora and Fractal must be a power of 2, so it is set to 64 here.

### 2. Fractal ZKP systems

The only difference from cubic_expression under Fractal ZKP systems is the setting of circuit parameters.

```Objective-C++
const size_t num_constraints = 1<<6;  
const size_t num_inputs = (1 << 5) - 1;
const size_t num_variables = (1<<6) - 1;
const size_t security_parameter = 128;
const size_t RS_extra_dimensions = 3;
const size_t FRI_localization_parameter = 3;
const LDT_reducer_soundness_type ldt_reducer_soundness_type = LDT_reducer_soundness_type::optimistic_heuristic;
const FRI_soundness_type fri_soundness_type = FRI_soundness_type::heuristic;
const field_subset_type domain_type = multiplicative_coset_type;
```

num_constraints is the number of constraints for the circuit. The circuit constraints for rangeproof built in libsnark are 39, but the number of circuits supported by Aurora and Fractal must be a power of 2, so it is set to 64 here.

### 3. Ligero ZKP systems

The only difference from cubic_expression under Ligero ZKP systems is the setting of circuit parameters.

```Objective-C++
const size_t num_constraints = 39;  
const size_t constraint_dim = 8;   
const size_t num_inputs = (1 << 5) - 1;
const size_t num_variables = 39 - 1;
```

num_constraints is the number of constraints for the circuit. The circuit constraints for rangeproof built in libsnark are 39, it is set to 39 here.
