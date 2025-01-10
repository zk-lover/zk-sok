# `libiop` Circuit Documentation
Libiop has not provided a complete toolchain for creating an R1CScircuit. We follow the API of the libraries and test the efficiency of randomly generated circuits with roughly the same quantity of constraints compared with the sample programs using libsnark，which developed by the same team of libiop. Toolkits: To support three proving systems, libiop uses a
namespace iop which contains aurora_iop, fractal_iop, and ligero_iop as specific protocols. The user-level APIs of the
three schemes are the same, which makes them convenient to use.

## 1. r1cs_constraint

**Purpose:**

`r1cs_constraint` represents a single constraint in a Rank-1 Constraint System (R1CS). It is a core building block used to express a relationship of the form:

\[
\langle A, X \rangle \cdot \langle B, X \rangle = \langle C, X \rangle
\]

where \( X \) is a vector of variables, and \( A \), \( B \), \( C \) are vectors of coefficients.

**Main variables:**

- `a_`, `b_`, `c_`: These are `linear_combination` objects that represent the coefficients for the vectors \( A \), \( B \), and \( C \) respectively.

**Main methods:**

1. r1cs_constraint::r1cs_constraint

```cpp
r1cs_constraint(const linear_combination<FieldT> &a, 
                const linear_combination<FieldT> &b, 
                const linear_combination<FieldT> &c);
```

**Purpose**:

Constructor to create a constraint using three linear combinations ( A ), ( B ), and ( C ).

**Example**:

```
r1cs_constraint<FieldT> constraint(a, b, c);
```

## 2. r1cs_constraint_system

**Purpose:**

`r1cs_constraint_system` manages a collection of R1CS constraints and provides methods to verify their validity and satisfaction.

**Main variables:**

- `primary_input_size_`: Number of input variables (public inputs).
- `auxiliary_input_size_`: Number of auxiliary variables (private inputs).
- `constraints_`: A vector of `r1cs_constraint` objects.

**Main methods:**

1. add_constraint

```cpp
void add_constraint(const r1cs_constraint<FieldT> &c);
```

**Purpose**:
Adds a new constraint to the system.

**Example**:
```
r1cs_constraint_system<FieldT> system;
system.add_constraint(constraint);
```

2. is_satisfied
```
bool is_satisfied(const r1cs_primary_input<FieldT> &primary_input, 
                  const r1cs_auxiliary_input<FieldT> &auxiliary_input) const;
```

**Purpose**:
Checks if the system’s constraints are satisfied given the primary and auxiliary inputs.

**Example**:
```
bool satisfied = system.is_satisfied(primary_input, auxiliary_input);
```

3. num_variables
```
size_t num_variables() const;
```

**Purpose**:
Returns the total number of variables (primary + auxiliary).

**Example**:
```
size_t vars = system.num_variables();
```

## 3. linear_combination and linear_term

**Purpose:**

A `linear_combination` represents a sum of terms, where each term is a coefficient-variable pair (`linear_term`).

**Key features:**

- Enables the flexible construction of \( A \), \( B \), and \( C \) for constraints.
- Provides methods like `evaluate` to compute the value of the linear combination given variable assignments.

## 4. generate_r1cs_example<FieldT>

**Purpose**:   
generate_r1cs_example is a function in libiop that generates R1CS (Rank-1 Constraint System) examples. 

```
r1cs_example<FieldT> generate_r1cs_example(const size_t num_constraints,
                                           const size_t num_inputs,
                                           const size_t num_variables)
```

**Parameter description**:

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
    