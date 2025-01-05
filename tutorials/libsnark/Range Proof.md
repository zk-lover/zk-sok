### Range Proof

#### rangeproof.cpp

Variables and constraints:

```markup
protoboard<FieldT> pb;
pb_variable<FieldT> x, max;
pb_variable<FieldT> less, less_or_eq;

x.allocate(pb, "x");
max.allocate(pb, "max");
less.allocate(pb, "less"); // must have
less_or_eq.allocate(pb, "less_or_eq");
```

Calculate 2^32 and store it in the variable **max**：

```markup
pb.val(max) = FieldT::one(); 
FieldT two = FieldT::one() + FieldT::one(); 
for (int i = 0; i < 32; ++i) {
    pb.val(max) = pb.val(max) * two; 
}
```

Use **comparison_gadget** to create a comparison circuit to check if **x** is less than or equal to **max**, then generate the constraints：

```markup
comparison_gadget<FieldT> cmp(pb, 32, x, max, less, less_or_eq, "cmp");
cmp.generate_r1cs_constraints();
```

Extract the constraint system from the **protoboard** and generate the **key pair**:

```markup
const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();
// generate keypair
const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(constraint_system);
```

Set a secret value of **18** for the variable **x** and generate the **R1CS witness**. Then, use the generated public key and witness to create the **proof**, recording the time required to generate the proof:

```markup
// Add witness values
pb.val(x) = 18; // secret
cmp.generate_r1cs_witness();

// generate proof
const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input());

auto clock2 = std::chrono::high_resolution_clock::now();
auto duration1 = std::chrono::duration_cast<std::chrono::duration<double, std::milli>>(clock2 - clock1).count();
```

Finally, verify the proof, calculate the verification time, and output the results.

```markup
// verify
bool verified = r1cs_se_ppzksnark_verifier_strong_IC<default_r1cs_se_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof);
auto clock3 = std::chrono::high_resolution_clock::now();
auto duration2 = std::chrono::duration_cast<std::chrono::duration<double, std::milli>>(clock3 - clock2).count();

cout << "Number of R1CS constraints: " << constraint_system.num_constraints() << endl;
cout << "Primary (public) input: " << pb.primary_input() << endl;
cout << "Auxiliary (private) input: " << pb.auxiliary_input() << endl;
cout << "Verification status: " << verified << endl;
cout << "Total proving time (milliseconds): " << duration1<< endl;
cout << "Total verification time (milliseconds): " << duration2<< endl;
```



