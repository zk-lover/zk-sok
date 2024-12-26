# 1.init_public_params()
- Input: None
- Output: None
- Function: Initializes the required public and private parameters for generating the subsequent R1CS (Rank-1 Constraint System) constraints.

# 2. protoboard<FieldT> pb
- Description: A core structure in libsnark — represents the circuit’s protoboard (analogous to a breadboard in electronics).

# 3. pb_variable<FieldT> x
- Description: A basic structure in libsnark — represents a fundamental variable on the protoboard.

# 4. x.allocate(pb, "x")
- Member Function of pb_variable:
- Input: A protoboard variable pb and a string representing the name (“x”).
- Output: None
- Function: Adds the variable x to the protoboard and allocates space for it. After this, the variable x can be used on the protoboard, such as referencing it when adding constraints.

# 5. pb.set_input_sizes(int size)
- Member Function of protoboard:
- Input: An integer size representing the number of public inputs.
- Output: None
- Function: Sets the number of public inputs. The first size variables on the protoboard are considered public inputs, and the remaining ones are considered private (secret) variables.

# 6. pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x, x, sym_1))
- Member Function of protoboard:
- Input: Three pb_variable variables, representing the multiplication constraint x * x = sym_1.
- Output: None
- Function: Adds an R1CS constraint representing x * x = sym_1.

# 7. pb.val(x) = 3
- Member Function of protoboard:
- Input: An integer value.
- Output: None
- Function: Sets the value of x on the protoboard to 3.

# 8. pb.get_constraint_system()
- Member Function of protoboard:
- Input: None
- Output: r1cs_constraint_system
- Function: Retrieves the system of all R1CS constraints that have been created, which will be used for generating the subsequent proof.