# `libiop` Circuit Documentation
Libiop has not provided a complete toolchain for creating an R1CScircuit. We follow the API of the libraries and test the efficiency of randomly generated circuits with roughly the same quantity of constraints compared with the sample programs using libsnark，which developed by the same team of libiop. Toolkits: To support three proving systems, libiop uses a
namespace iop which contains aurora_iop, fractal_iop, and ligero_iop as specific protocols. The user-level APIs of the
three schemes are the same, which makes them convenient to use.

1. generate_r1cs_example<FieldT>
Purpose:   
generate_r1cs_example is a function in libiop that generates R1CS (Rank-1 Constraint System) examples. 
```
r1cs_example<FieldT> generate_r1cs_example(const size_t num_constraints,
                                           const size_t num_inputs,
                                           const size_t num_variables)
```
Parameter description:
num_constraints: Number of constraints in the R1CS.
num_inputs: Number of primary inputs in the R1CS.
num_variables: Number of variables in the R1CS.
**Example:**
```
const size_t num_constraints = 1 << 15;
const size_t num_inputs = (1 << 5) - 1;
const size_t num_variables = (1 << 15) - 1;

r1cs_example<FieldT> r1cs_params = generate_r1cs_example<FieldT>(num_constraints, num_inputs, num_variables);
``` 

2.
```
void add_constraint(const r1cs_constraint<FieldT> &c);
void add_constraint(const r1cs_constraint<FieldT> &c, const std::string &annotation);
```

3.
```
bool is_satisfied(const r1cs_primary_input<FieldT> &primary_input,
                      const r1cs_auxiliary_input<FieldT> &auxiliary_input) const;
```

    