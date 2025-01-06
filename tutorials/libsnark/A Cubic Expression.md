### A Cubic Expression

This code implements a simple zero-knowledge proof system using the libsnark library. It defines an algebraic circuit with the formula x^3 + x + 1 = out and describes this circuit using R1CS (Rank-1 Constraint System) constraints. The code initializes curve parameters and creates a protoboard to manage variables. It allocates variables and adds constraints to represent the circuit logic. Then, it performs a trusted setup to generate a keypair and creates a proof for a given input value (e.g, x = 3). Finally, the code verifies the proof's validity and outputs the verification result and time taken. It also demonstrates a case where verification fails when the output value is incorrect.

Below, we will divide the code into code blocks and annotate them.

First, the initial code block primarily performs some initialization and variable definitions, including:

* Define **FieldT** as the finite field element type;

* Initialize the curve parameters used in libsnark;

* Create a protoboard object **pb** to store variables and constraints;

* Define four variables: **x**, **x_squared**, **x_cubed**, and **out**, which will be used in the constraint system;

```
typedef libff::Fr<default_r1cs_se_ppzksnark_pp> FieldT;
// Initialize the curve parameters
default_r1cs_se_ppzksnark_pp::init_public_params();

// Create protoboard
protoboard<FieldT> pb;

// Define variables
pb_variable<FieldT> x;
pb_variable<FieldT> x_squared;
pb_variable<FieldT> x_cubed;
pb_variable<FieldT> out;
```

The second code block assigns the four defined variables to the **protoboard**, then sets the input size to 1, meaning the first variable assigned to the protoboard, **out**, is public, while the others are private.

```markup
// Allocate variables to protoboard
// The strings (like "x") are only for debugging purposes
out.allocate(pb, "out");  // y is the output (public)
x.allocate(pb, "x");      // x is the input (private)
x_squared.allocate(pb, "x_squared");  // x^2 (intermediate)
x_cubed.allocate(pb, "x_cubed");  // x^3 (intermediate)

// This sets up the protoboard variables
// so that the first one (out) represents the public
// input and the rest is private input
pb.set_input_sizes(1);
```

The following code block implements the addition of R1CS constraints:

1. x^2 = x * x

2. x^3 = x^2 * x

3. out = x^3 + x + 1

```markup
pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x, x, x_squared));
pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x_squared, x, x_cubed));
pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x_cubed + x + 1, 1, out));
```

Then, obtain the constraint system and generate the key pair:

```markup
const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();
const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(constraint_system);
```

Then, set the witness values and create the proof, recording the time required to generate the proof:

```markup
// Add witness values
pb.val(x) = 3;  // x = 3
pb.val(x_squared) = pb.val(x) * pb.val(x);  // x^2 = 9
pb.val(x_cubed) = pb.val(x_squared) * pb.val(x);  // x^3 = 27
pb.val(out) = pb.val(x_cubed) + pb.val(x) + FieldT(1);  // y = 27 + 3 + 1 = 31

const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input());

auto clock2 = std::chrono::high_resolution_clock::now();
auto duration1 = std::chrono::duration_cast<std::chrono::duration<double, std::milli>>(clock2 - clock1).count();
```

Verify the proof, calculate the verification time, and output the results, including the number of constraints, inputs, verification status, as well as the proof time and verification time:

```markup
// Verify proof
bool verified = r1cs_se_ppzksnark_verifier_strong_IC<default_r1cs_se_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof);
auto clock3 = std::chrono::high_resolution_clock::now();
auto duration2 = std::chrono::duration_cast<std::chrono::duration<double, std::milli>>(clock3 - clock2).count();
cout << "FOR SUCCESSFUL VERIFICATION" << endl;
cout << "Number of R1CS constraints: " << constraint_system.num_constraints() << endl;
cout << "Primary (public) input: " << pb.primary_input() << endl;
cout << "Auxiliary (private) input: " << pb.auxiliary_input() << endl;
cout << "Verification status: " << verified << endl;
cout << "Satisfied status: " << pb.is_satisfied() << endl;
cout << "Total proving time (milliseconds): " << duration1<< endl;
cout << "Total verification time (milliseconds): " << duration2<< endl;
```

Then, we set incorrect witness values and output the results:

```markup
// Add witness values
pb.val(x) = 3;  // x = 3
pb.val(x_squared) = pb.val(x) * pb.val(x);  // x^2 = 9
pb.val(x_cubed) = pb.val(x_squared) * pb.val(x);  // x^3 = 27
pb.val(out) = 28;  // Set an incorrect output (should be 31)

// Create proof
const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof1 = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input());

// Verify proof
bool verified1 = r1cs_se_ppzksnark_verifier_strong_IC<default_r1cs_se_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof1);

cout << "FOR UNSUCCESSFUL VERIFICATION" << endl;
cout << "Number of R1CS constraints: " << constraint_system.num_constraints() << endl;
cout << "Primary (public) input: " << pb.primary_input() << endl;
cout << "Auxiliary (private) input: " << pb.auxiliary_input() << endl;
cout << "Verification status: " << verified1 << endl;
cout << "Satisfied status: " << pb.is_satisfied() << endl;
```

