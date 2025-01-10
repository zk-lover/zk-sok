libsnark not only provides gadgets for building circuits, which makes it easier for developers to construct circuit structures, but also offers some commonly used gadgets that developers can directly use to build circuits for specific applications. These include hash functions, cryptographic primitives, elliptic curve operations, Merkle trees, and more.

For example, in the `gadgetlib1/gadgets` directory:
- The `hashes` directory provides gadgets related to hash functions, such as `sha256_two_to_one_hash_gadget`.
- The `pairing` directory provides gadgets related to pairing functions, such as `miller`, `check`.
- The `curves` directory provides gadgets related to elliptic curve operations, such as `G1_add_gadget`, `G1_mul_gadget`.
- The `merkles` directory provides gadgets related to Merkle trees, such as `merkle_tree_check_update_gadget`, `merkle_authentication_path_variable`.

Here are examples of some commonly used gadgets:

# 3. Common Gadgets

## 1. sha256_two_to_one_hash_gadget
- `sha256_two_to_one_hash_gadget` is a component that implements the SHA256 compression function as a 2-to-1 hash function. It supports two constructors:
  - Constructor 1: 
    - `protoboard<FieldT> &pb`: The constraint system's protoboard.
    - `const digest_variable<FieldT> &left`: The first input hash value.
    - `const digest_variable<FieldT> &right`: The second input hash value.
    - `const digest_variable<FieldT> &output`: The hash output.
    - `const std::string &annotation_prefix`: The identifier for the gadget.
  - Constructor 2:
    - `protoboard<FieldT> &pb`: The constraint system's protoboard.
    - `const size_t block_length`: The length of the input block.
    - `const block_variable<FieldT> &input_block`: The input block data.
    - `const digest_variable<FieldT> &output`: The hash output.
    - `const std::string &annotation_prefix`: The identifier for the gadget.

## 2. merkle_tree_check_update_gadget
- `merkle_tree_check_update_gadget` is used to verify the update relationship between two Merkle tree roots (R1 and R2):
  1. Check if the verification path P is a valid path for the leaf node corresponding to value V1 at address A in the Merkle tree with root R1.
  2. Check if the verification path P is a valid path for the leaf node corresponding to value V2 at address A in the Merkle tree with root R2.
- Constructor:
  1. `protoboard<FieldT> &pb`: The protoboard used to construct the R1CS.
  2. `const size_t tree_depth`: The depth of the Merkle tree.
  3. `const pb_variable_array<FieldT> &address_bits`: The binary address of the leaf node.
  4. `const digest_variable<FieldT> &prev_leaf_digest` and `const digest_variable<FieldT> &next_leaf_digest`: The hash values of the leaf nodes before and after the update.
  5. `const digest_variable<FieldT> &prev_root_digest` and `const digest_variable<FieldT> &next_root_digest`: The hash values of the roots before and after the update.
  6. `const merkle_authentication_path_variable<FieldT, HashT> &prev_path` and `const merkle_authentication_path_variable<FieldT, HashT> &next_path`: The verification paths before and after the update.
  7. `const pb_linear_combination<FieldT> &update_successful`: A boolean variable indicating whether the update is successful.
  8. `const std::string &annotation_prefix`: The identifier for the gadget.

## 3. G1_add_gadget
- `G1_add_gadget` is used to verify whether the point addition operation on the elliptic curve G1 is correct (verify if the result point C is the sum of points A and B: C = A + B).
- Constructor:
  1. `protoboard &pb`: The protoboard used to construct the R1CS.
  2. `const G1_variable &A`: The first input point A on the elliptic curve.
  3. `const G1_variable &B`: The second input point B on the elliptic curve.
  4. `const G1_variable &C`: The result point C of the elliptic curve addition.
  5. `const std::string &annotation_prefix`: The identifier for the gadget.
