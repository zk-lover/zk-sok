# 1.init_public_params()
- Input: None
- Output: None
- Function: Initializes the required public and private parameters for generating the subsequent R1CS (Rank-1 Constraint System) constraints.

# 2. Protoboard
## 1. protoboard<FieldT> pb
- Description: A core structure in libsnark — represents the circuit’s protoboard (analogous to a breadboard in electronics).

## 2. pb_variable<FieldT> x
- Description: A basic structure in libsnark — represents a fundamental variable on the protoboard.

## 3. x.allocate(pb, "x")
- Member Function of pb_variable:
- Input: A protoboard variable pb and a string representing the name (“x”).
- Output: None
- Function: Adds the variable x to the protoboard and allocates space for it. After this, the variable x can be used on the protoboard, such as referencing it when adding constraints.

## 4. pb.set_input_sizes(int size)
- Member Function of protoboard:
- Input: An integer size representing the number of public inputs.
- Output: None
- Function: Sets the number of public inputs. The first size variables on the protoboard are considered public inputs, and the remaining ones are considered private (secret) variables.

## 5. pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x, x, sym_1))
- Member Function of protoboard:
- Input: Three pb_variable variables, representing the multiplication constraint x * x = sym_1.
- Output: None
- Function: Adds an R1CS constraint representing x * x = sym_1.

## 6. pb.val(x) = 3
- Member Function of protoboard:
- Input: An integer value.
- Output: None
- Function: Sets the value of x on the protoboard to 3.

## 7. pb.get_constraint_system()
- Member Function of protoboard:
- Input: None
- Output: r1cs_constraint_system
- Function: Retrieves the system of all R1CS constraints that have been created, which will be used for generating the subsequent proof.

# 2. Gadgets
- Gadgets allow developers to package general R1CS constraints for easy invocation. For users, when using gadgets, we only need to focus on their inputs and outputs without worrying about internal details. For developers, some interfaces of gadgets need to be implemented, such as constructors, `generate_r1cs_constraints()`, `generate_r1cs_witness()`.
## 1. Constructor: gadget(protoboard<FieldT> &pb, const std::string &annotation_prefix="")
- A gadget must be bound to a protoboard. annotation_prefix is an optional parameter used to identify the gadget;
- Custom gadgets can be created using C++ inheritance as needed;
## 2. generate_r1cs_constraints()
- Used to add R1CS constraints to the circuit, just encapsulated;
- Developers need to implement the corresponding logic for adding constraints within the function;
## 3. generate_r1cs_witness()
- This function assumes that we have already set the secret and public variables of the circuit;
- Used to generate intermediate variable values needed during proof derivation;

# 3. Common gadgets
## 1. sha256_two_to_one_hash_gadget
- sha256_two_to_one_hash_gadget is a component that implements the SHA256 compression function, used as a 2-to-1 hash function, supporting two constructors:
- Constructor input 1: protoboard<FieldT> &pb: protoboard of the constraint system; const digest_variable<FieldT> &left, const digest_variable<FieldT> &right: two input hash values; const digest_variable<FieldT> &output: hash output; const std::string &annotation_prefix: gadget identifier;
- Constructor input 2: protoboard<FieldT> &pb: protoboard of the constraint system; const size_t block_length: length of the input block; const block_variable<FieldT> &input_block: input block data; const digest_variable<FieldT> &output: hash output; const std::string &annotation_prefix: gadget identifier;

## 2. merkle_tree_check_update_gadget
- merkle_tree_check_update_gadget is used to verify the update relationship between two given Merkle tree roots (R1 and R2):

    1. Check whether the verification path P is a valid verification path for value V1 as the leaf node corresponding to address A in the Merkle tree with root R1.
    2. Check whether the verification path P is a valid verification path for value V2 as the leaf node corresponding to address A in the Merkle tree with root R2.
- Constructor:

    1. protoboard<FieldT> &pb: used to construct the R1CS protoboard
    2. const size_t tree_depth: depth of the Merkle tree
    3. const pb_variable_array<FieldT> &address_bits: binary address of the leaf node
    4. const digest_variable<FieldT> &prev_leaf_digest and const digest_variable<FieldT> &next_leaf_digest: hash values of the leaf node before and after the update
    5. const digest_variable<FieldT> &prev_root_digest and const digest_variable<FieldT> &next_root_digest: hash values of the root before and after the update
    6. const merkle_authentication_path_variable<FieldT, HashT> &prev_path and const merkle_authentication_path_variable<FieldT, HashT> &next_path: verification paths before and after the update
    7. const pb_linear_combination<FieldT> &update_successful: boolean variable indicating whether the update was successful
    8. const std::string &annotation_prefix: identifier;

## 3. G1_add_gadget
- G1_add_gadget is used to verify whether the point addition operation on the elliptic curve G1 group is correct (whether the result point C is the addition result of points A and B: C = A + B);
- Constructor:
    1. protoboard &pb: used to construct the R1CS protoboard.
    2. const G1_variable &A: the first input point A on the elliptic curve.
    3. const G1_variable &B: the second input point B on the elliptic curve.
    4. const G1_variable &C: the result point C of the elliptic curve addition.
    5. const std::string &annotation_prefix: identifier;
