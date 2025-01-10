# libsnark Circuit Documentation
In this section, we will introduce the circuit building APIs of libsnark, including Protoboard, Gadget for circuit, and ConstraintSystem.

## 1. Protoboard (from libsnark/gadgetlib1/protoboard.hpp)

**Purpose:** The core tool in libsnark for building and managing constraint systems. It allows defining circuit variables, adding constraints, and provides the necessary interfaces to verify whether the constraints are satisfied. Usually, we declare circuit variables in protoboard, add constraints to these variables, and finally use it to generate and verify zero-knowledge proofs.  
**Main variables:** val(const pb_variable`<FieldT>` &var);

**Functions:**

### 1.1. pb_variable::allocate

```
void allocate(protoboard<FieldT> &pb, const std::string &annotation="");
```

**Purpose:**
allocate is used to allocate storage space for a given variable in protoboard. It registers a new variable and adds it to the protoboard as part of the circuit  
**Input:**
pb: The current protoboard instance.
name: The name of the variable, used for debugging and identification.  
**Output:**
No return value, directly modify the protoboard object and register the variable to the circuit.  
**Example:**

```
pb_variable<FieldT> x;
x.allocate(pb, "x");
```

### 1.2. Protoboard::set_input_sizes

```
void set_input_sizes(const size_t primary_input_size);
```

**Purpose:**
set_input_sizes is used to set the input size of the circuit, that is, the number of input variables. Usually, in zero-knowledge proofs, input variables are those that need to be provided to the proof generator or verifier.  
**Input:**
input_size: The number of input variables. This value determines the size of the input part of the circuit.  
**Output:**
No return value, directly modify the input size in the protoboard object.   
**Example:**

```
pb.set_input_sizes(1);
```

### 1.3. Protoboard::add_r1cs_constraint

```
void add_r1cs_constraint(const r1cs_constraint<FieldT> &constr, const std::string &annotation="");
```

**Purpose:** add_r1cs_constraint is used to add a new R1CS constraint to the protoboard. This is one of the core operations for building a circuit. The constraint defines how to ensure the consistency of the circuit through the relationship between variables.  
**Input:**
`constraint`: The R1CS constraint that needs to be added to the circuit, which is usually represented by the r1cs_constraint`<FieldT>` type. This constraint defines the linear relationship between variables.  
**Output:**
No return value, directly modify the constraint system of the protoboard object.  
**Example:**

```
pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x, x, x_squared));
```

### 1.4. Protoboard::primary_input()

```
r1cs_primary_input<FieldT> primary_input() const;
```

**Purpose:**
The primary_input function returns the primary input of a circuit. Primary inputs usually include public variables. In zero-knowledge proofs, primary inputs are the public part shared by the prover and the verifier.  
**Input:**
None.  
**Output:**
Returns an object of type r1cs_primary_input`<FieldT>`, representing all primary inputs. r1cs_primary_input is an array containing fields of type FieldT, which are usually the public input data of the circuit.  
**Example:**

```
r1cs_primary_input<FieldT> primary_inputs = pb.primary_input();
```

### 1.5. Protoboard::auxiliary_input()

```
r1cs_auxiliary_input<FieldT> auxiliary_input() const;
```

**Purpose:**
The auxiliary_input function returns the auxiliary inputs of the circuit. Auxiliary inputs are inputs that are private to the prover, they remain private during the proof and are not disclosed to the verifier. Auxiliary inputs are usually secret parts of variables associated with the primary inputs and are used to generate the proof.  
**Inputs:**
None.  
**Outputs:**
Returns an object of type r1cs_auxiliary_input`<FieldT>` representing all auxiliary inputs. r1cs_auxiliary_input is an array of fields of type FieldT, usually containing data that the prover needs to keep private.  
**Example:**

```
r1cs_auxiliary_input<FieldT> auxiliary_inputs = pb.auxiliary_input();
```

### 1.6. Protoboard::get_constraint_system()

```
r1cs_constraint_system<FieldT> get_constraint_system() const;
```

**Purpose:**
The get_constraint_system function returns the complete constraint system in the current protoboard. The constraint system is a collection of all constraints in the circuit, including all R1CS constraints added by add_r1cs_constraint.  
**Input:**
None.  
**Output:**
Returns an object of type r1cs_constraint_system`<FieldT>`, representing all constraints of the current circuit. This object contains all constraints registered by the add_r1cs_constraint method.  
**Example:**

```
r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();
```

## 2.Gadgets for circuit

- Gadgets can support developers to package common r1cs constraints for easy calling. For callers, when using gadegts, we only need to pay attention to its input and output, without paying attention to the internal details; for developers, it is necessary to implement some interfaces of gadgets, such as constructors, `generate_r1cs_constraints()`, `generate_r1cs_witness()`;

### 2.1. Constructor: gadget(protoboard`<FieldT>` &pb, const std::string &annotation_prefix="")

- Gadget must be bound to a protoboard, annotation_prefix is ​​an optional parameter used to identify the gadget;
- You can customize gadgets through inheritance in C++ and perform different constructions according to needs;

### 2.2 generate_r1cs_constraints()

- Used to add r1cs constraints to the circuit, just encapsulated;
- Requires developers to implement the corresponding logic of adding constraints within the function;

### 2.3 generate_r1cs_witness()

- This function assumes that we have set the secret variables and public variables of the circuit;
- Used to generate the intermediate variable values ​​required for proof derivation;
