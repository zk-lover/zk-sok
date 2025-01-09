# Spartan Circuit Documentation

## 1.Instance

**Purpose:** Represents an R1CS (Rank-1 Constraint System) instance, including constraint matrices and system parameters.

**Components:**
- Number of constraints (`num_constraints`)
- Number of variables (`num_variables`)
- Number of inputs (`num_inputs`)
- Matrices `A`, `B`, `C`

**Related Functions:**

1. **new**
   - **Input:**
     - `num_constraints`: Number of constraints
     - `num_variables`: Number of variables
     - `num_inputs`: Number of inputs
     - `A`, `B`, `C`: Sparse matrices
   - **Output:**`Instance`: Newly created R1CS instance.
   - **Purpose:** Creates a new R1CS instance for generating and verifying proofs.
   - **Example Code:**
     ```rust
     let instance = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();
     ```

2. **is_sat**
   - **Input:**
     - `vars`: Variable assignment (`VarsAssignment`)
     - `inputs`: Input assignment (`InputsAssignment`)
   - **Output:**`Result<bool, Error>`: Indicates whether the instance is satisfiable.
   - **Purpose:** Checks if the given variables and inputs satisfy the R1CS constraints.
   - **Example Code:**
     ```rust
     let is_satisfied = instance.is_sat(&vars_assignment, &inputs_assignment).unwrap();
     ```

## 2.VarsAssignment

**Purpose:** Represents the specific assignment of variables in an R1CS instance.

**Components:**
- Byte array of variables.

**Related Functions:**

1. **new**
   - **Input:**`vars`: Byte array of variables.
   - **Output:**`VarsAssignment`: New variable assignment instance.
   - **Purpose:** Creates a variable assignment instance for verifying the satisfiability of the R1CS.
   - **Example Code:**
     ```rust
     let vars_assignment = VarsAssignment::new(&vars).unwrap();
     ```

## 3.InputsAssignment

**Purpose:** Represents the specific assignment of public inputs in an R1CS instance.

**Components:**
- Byte array of inputs.

**Related Functions:**

1. **new**
   - **Input:**`inputs`: Byte array of inputs.
   - **Output:**`InputsAssignment`: New input assignment instance.
   - **Purpose:** Creates a public input assignment instance for verifying the satisfiability of the R1CS.
   - **Example Code:**
     ```rust
     let inputs_assignment = InputsAssignment::new(&inputs).unwrap();
     ```