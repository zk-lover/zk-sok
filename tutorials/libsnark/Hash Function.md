### Hash Function

This code implements a zero-knowledge proof system using the libsnark library, primarily to verify the correctness of the SHA-256 hash function. It defines two main functions: `two_inputs_hash_gadget` and `one_input_hash_gadget`. The `two_inputs_hash_gadget` handles hash computation for two inputs, while `one_input_hash_gadget` deals with a single input. Each function ensures the correctness of the hash computation by generating constraints, creating proofs, and verifying those proofs. The `main` function initializes the parameters and calls `one_input_hash_gadget` for testing.

Below, we will divide the code into code blocks and annotate them.

The implementation of this code primarily uses the hash function tools in libsnark.

##### verify_proof：

Use the **r1cs_se_ppzksnark_verifier_strong_IC** function to verify the given proof:

```markup
bool verify_proof(r1cs_se_ppzksnark_verification_key<ppT> verification_key, r1cs_primary_input<FieldT> primary_input,
        r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof) {
    return r1cs_se_ppzksnark_verifier_strong_IC<default_r1cs_se_ppzksnark_pp>(verification_key, primary_input, proof);
}
```

##### setup_gadget：

This function is used to set up the SHA-256 hash gadgets and generate the constraints and key pair needed for zero-knowledge proof. The complete code is as follows:

```markup
r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> setup_gadget(protoboard<FieldT> &pb, block_variable<FieldT> *&inp, digest_variable<FieldT> *&out, sha256_two_to_one_hash_gadget<FieldT> *&g) {
    inp = new block_variable<FieldT>(pb, SHA256_block_size, "input");
    out = new digest_variable<FieldT>(pb, SHA256_block_size, "output");
    g = new sha256_two_to_one_hash_gadget<FieldT>(pb, SHA256_block_size, *inp, *out, "f");
    g->generate_r1cs_constraints();
    printf("Number of constraints for sha256_two_to_one_hash_gadget: %zu\n", pb.num_constraints());

    // Trusted setup
    const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();
    const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(
            constraint_system);
    return keypair;
}
```

**g** is a SHA-256 hash gadget that links the input and output variables and is responsible for generating the corresponding R1CS constraints:

```markup
g = new sha256_two_to_one_hash_gadget<FieldT>(pb, SHA256_block_size, *inp, *out, "f");
```

Call the **g->generate_r1cs_constraints()** to generate R1CS constraints, which are used to describe the computation process of the SHA-256 hash function:

```markup
g->generate_r1cs_constraints();
```

Obtain the constraint system and call **r1cs_se_ppzksnark_generator** to generate a key pair, which is used for the generation and verification of zero-knowledge proofs:

```markup
// Trusted setup
const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();
const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(constraint_system);
return keypair;
```

##### two_inputs_hash_gadget

This function demonstrates how to use the SHA-256 hash gadget with two inputs to compute a new hash value from two hash values. The complete code is as follows:

```markup
void two_inputs_hash_gadget() {
    typedef libff::Fr<default_r1cs_se_ppzksnark_pp> FieldT;

    protoboard<FieldT> pb;

    digest_variable<FieldT> left(pb, SHA256_digest_size, "left");
    digest_variable<FieldT> right(pb, SHA256_digest_size, "right");
    digest_variable<FieldT> output(pb, SHA256_digest_size, "output");

    sha256_two_to_one_hash_gadget<FieldT> f(pb, left, right, output, "f");
    f.generate_r1cs_constraints();
    printf("Number of constraints for sha256_two_to_one_hash_gadget: %zu\n", pb.num_constraints());

    // Trusted setup
    const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();

    const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(
            constraint_system);

    // Add witness values

    // Empty string (all 0s)
    const libff::bit_vector left_bv = libff::int_list_to_bits({0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0}, 32);
    const libff::bit_vector right_bv = libff::int_list_to_bits({0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0}, 32);
    const libff::bit_vector hash_bv = libff::int_list_to_bits({0xda5698be, 0x17b9b469, 0x62335799, 0x779fbeca, 0x8ce5d491, 0xc0d26243, 0xbafef9ea, 0x1837a9d8}, 32);

    // concatenation of bytes:
    // [197, 215, 20, 132, 248, 207, 155, 244, 183, 111, 71, 144, 71, 48, 128, 75, 158, 50, 37, 169, 241, 51, 181, 222, 161, 104, 244, 226, 133, 31, 7, 47]
    // and
    // [204, 0, 252, 170, 124, 166, 32, 97, 113, 122, 72, 229, 46, 41, 163, 250, 55, 154, 149, 63, 170, 104, 147, 227, 46, 197, 162, 123, 148, 94, 96, 95]
    /*const libff::bit_vector left_bv = libff::int_list_to_bits({0x426bc2d8, 0x4dc86782, 0x81e8957a, 0x409ec148, 0xe6cffbe8, 0xafe6ba4f, 0x9c6f1978, 0xdd7af7e9}, 32);
    const libff::bit_vector right_bv = libff::int_list_to_bits({0x038cce42, 0xabd366b8, 0x3ede7e00, 0x9130de53, 0x72cdf73d, 0xee825114, 0x8cb48d1b, 0x9af68ad0}, 32);
    const libff::bit_vector hash_bv = libff::int_list_to_bits({0xeffd0b7f, 0x1ccba116, 0x2ee816f7, 0x31c62b48, 0x59305141, 0x990e5c0a, 0xce40d33d, 0x0b1167d1}, 32);*/

    left.generate_r1cs_witness(left_bv);
    right.generate_r1cs_witness(right_bv);

    f.generate_r1cs_witness();
    output.generate_r1cs_witness(hash_bv);

    cout << "two_inputs_hash_gadget => Satisfied status: " << pb.is_satisfied() << endl;

    // Create proof
    const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof1 = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(
            keypair.pk, pb.primary_input(), pb.auxiliary_input());

    // Verify proof
    bool verified1 = verify_proof(keypair.vk, pb.primary_input(), proof1);

    cout << "two_inputs_hash_gadget => Verfied: " << verified1 << endl;
}
```

Variable definition: Three hash variables are defined: left, right, and output, each with the size of a SHA-256 hash:

```markup
digest_variable<FieldT> left(pb, SHA256_digest_size, "left");
digest_variable<FieldT> right(pb, SHA256_digest_size, "right");
digest_variable<FieldT> output(pb, SHA256_digest_size, "output");
```

Initialize a SHA-256 hash gadget **f** and generate R1CS constraints:

```markup
sha256_two_to_one_hash_gadget<FieldT> f(pb, left, right, output, "f");
f.generate_r1cs_constraints();
```

Obtain the constraint system and generate the key pair:

```markup
const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();
const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(constraint_system);
```

Defined the witness values for **left** and **right**, as well as the expected hash value **hash_bv**:

```markup
const libff::bit_vector left_bv = libff::int_list_to_bits({0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0}, 32);
const libff::bit_vector right_bv = libff::int_list_to_bits({0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0}, 32);
const libff::bit_vector hash_bv = libff::int_list_to_bits({0xda5698be, 0x17b9b469, 0x62335799, 0x779fbeca, 0x8ce5d491, 0xc0d26243, 0xbafef9ea, 0x1837a9d8}, 32);
```

Then generate the R1CS witness for **left**, **right**, and **output**:

```markup
left.generate_r1cs_witness(left_bv);
right.generate_r1cs_witness(right_bv);
f.generate_r1cs_witness();
output.generate_r1cs_witness(hash_bv);
```

Check if the constraints are satisfied and output the results:

```markup
cout << "two_inputs_hash_gadget => Satisfied status: " << pb.is_satisfied() << endl;
```

Use the key pair validity of the proof, and finally output the verification results:

```markup
const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof1 = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input());
bool verified1 = verify_proof(keypair.vk, pb.primary_input(), proof1);
cout << "two_inputs_hash_gadget => Verfied: " << verified1 << endl;
```

##### one_input_hash_gadget

This function demonstrates how to use the SHA-256 hash gadget to authenticate the hash value of a string. The complete code is as follows:

```markup
int one_input_hash_gadget(int num_iterations) {
    using namespace std::chrono;

    protoboard<FieldT> pb;
    block_variable<FieldT>* input;
    digest_variable<FieldT>* output;
    sha256_two_to_one_hash_gadget<FieldT>* f;

    r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = setup_gadget(pb, input, output, f);

    int i = 0;

    duration<double> tc(0);
    duration<double> tp(0);
    duration<double> tv(0);

    while (i < num_iterations) {
        // Hash of string "hello world"
        const libff::bit_vector hash_bv = libff::int_list_to_bits({0x605b0cd0, 0xc4f79cc4, 0x232a1c0f, 0xcdd92dd6, 0x4f0d8cd0, 0x66c610d4, 0x82ab2037, 0xb0d7c550}, 32);
        output->generate_r1cs_witness(hash_bv);

        steady_clock::time_point begin = steady_clock::now();
        // Add witness values
        // For string "hello world"
        const libff::bit_vector input_bv = libff::int_list_to_bits({0x68756461, 0x69710000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000}, 32);
        input->generate_r1cs_witness(input_bv);

        f->generate_r1cs_witness();

        steady_clock::time_point mid = steady_clock::now();
        tc += duration_cast<duration<double>>(mid - begin);

        cout << "one_input_hash_gadget => Satisfied status: " << pb.is_satisfied() << endl;

        const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();
        cout << "Number of R1CS constraints: " << constraint_system.num_constraints() << endl;
        // Create proof
        const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof1 = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(
                keypair.pk, pb.primary_input(), pb.auxiliary_input());

        steady_clock::time_point end = steady_clock::now();
        tp += duration_cast<duration<double>>(end - begin);

        steady_clock::time_point begin1 = steady_clock::now();
        // Verify proof
        bool verified1 = verify_proof(keypair.vk, pb.primary_input(), proof1);
        steady_clock::time_point end1 = steady_clock::now();
        if (verified1 == 0) return -1;
        tv += duration_cast<duration<double>>(end1 - begin1);

        cout << "one_input_hash_gadget => Verfied: " << verified1 << endl;

        /*cout << "primary_input: " << pb.primary_input() << endl;

        cout << "auxiliary_input: " << pb.auxiliary_input() << endl;*/

        i++;
    }

    cout << "Total iterations : " << num_iterations << endl;
    cout << "Total constraint generation time (seconds): " << tc.count() << endl;
    cout << "Total proving time (seconds): " << tp.count() << endl;
    cout << "Total verification time (seconds): " << tv.count() << endl;

    return 1;
}
```

Initialization and setup:

* Use **std::chrono** for time measurement

* Initialize **protoboard pb** to store constraints and variables

* Define input, output, and hash gadget

* Call the **setup_gadget** function to set up the gadget and generate the key pair

```markup
using namespace std::chrono;
protoboard<FieldT> pb;
block_variable<FieldT>* input;
digest_variable<FieldT>* output;
sha256_two_to_one_hash_gadget<FieldT>* f;
r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = setup_gadget(pb, input, output, f);
```

Define three time measurement variables: **tc** (constraint generation time), **tp** (proof generation time), and **tv** (verification time):

```markup
duration<double> tc(0);
duration<double> tp(0);
duration<double> tv(0);
```

Iterate **num_iterations** times

* Define the hash value hash_bv for the string 'hello world' and generate the R1CS witness for the output:

```markup
while (i < num_iterations) {
    // Hash of string "hello world"
    const libff::bit_vector hash_bv = libff::int_list_to_bits({0x605b0cd0, 0xc4f79cc4, 0x232a1c0f, 0xcdd92dd6, 0x4f0d8cd0, 0x66c610d4, 0x82ab2037, 0xb0d7c550}, 32);
    output->generate_r1cs_witness(hash_bv);
```

Generate the R1CS witness for the input, record the intermediate time, and calculate the constraint generation time:

```markup
steady_clock::time_point begin = steady_clock::now();
const libff::bit_vector input_bv = libff::int_list_to_bits({0x68756461, 0x69710000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000}, 32);
input->generate_r1cs_witness(input_bv);
f->generate_r1cs_witness();
steady_clock::time_point mid = steady_clock::now();
tc += duration_cast<duration<double>>(mid - begin);
```

Check if the constraints are satisfied and output the results:

* Obtain the constraint system and output the number of constraints

* Generate the zk-SNARKs proof

* Record the end time and calculate the proof generation time

```markup
cout << "one_input_hash_gadget => Satisfied status: " << pb.is_satisfied() << endl;
const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();
cout << "Number of R1CS constraints: " << constraint_system.num_constraints() << endl;
const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof1 = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input());
steady_clock::time_point end = steady_clock::now();
tp += duration_cast<duration<double>>(end - begin);
```

Verify the proof and record the verification time:

```markup
steady_clock::time_point begin1 = steady_clock::now();
bool verified1 = verify_proof(keypair.vk, pb.primary_input(), proof1);
steady_clock::time_point end1 = steady_clock::now();
if (verified1 == 0) return -1;
tv += duration_cast<duration<double>>(end1 - begin1);
cout << "one_input_hash_gadget => Verfied: " << verified1 << endl;
```

Finally, output the total number of iterations, constraint generation time, proof generation time, and verification time:

```markup
cout << "Total iterations : " << num_iterations << endl;
cout << "Total constraint generation time (seconds): " << tc.count() << endl;
cout << "Total proving time (seconds): " << tp.count() << endl;
cout << "Total verification time (seconds): " << tv.count() << endl;
```

##### main

```markup
int main() {
    // Initialize the curve parameters
    default_r1cs_se_ppzksnark_pp::init_public_params();
    int num_iterations = 1;
//    two_inputs_hash_gadget();
    if (one_input_hash_gadget(num_iterations) != 1) return -1;
    return 0;
}
```

