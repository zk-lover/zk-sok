#include <stdlib.h>
#include <iostream>
#include <chrono>
#include "libsnark/zk_proof_systems/ppzksnark/r1cs_se_ppzksnark/r1cs_se_ppzksnark.hpp"
#include "libsnark/common/default_types/r1cs_se_ppzksnark_pp.hpp"
#include "libsnark/gadgetlib1/pb_variable.hpp"

using namespace libsnark;
using namespace std;

int main() {
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

    // Add R1CS constraints to protoboard

    // x^3 + x + 1 = out
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x, x, x_squared));

    // Step 2: Compute x^3 = x^2 * x
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x_squared, x, x_cubed));

    // Step 3: Compute out = x^3 + x + 1
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x_cubed + x + 1, 1, out));

    auto clock1 = std::chrono::high_resolution_clock::now();
    // Trusted setup
    const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();

    const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(
            constraint_system);

    // Add witness values
    pb.val(x) = 3;  // x = 3
    pb.val(x_squared) = pb.val(x) * pb.val(x);  // x^2 = 9
    pb.val(x_cubed) = pb.val(x_squared) * pb.val(x);  // x^3 = 27
    pb.val(out) = pb.val(x_cubed) + pb.val(x) + FieldT(1);  // y = 27 + 3 + 1 = 31

    // Create proof
    const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(
            keypair.pk, pb.primary_input(), pb.auxiliary_input());
    auto clock2 = std::chrono::high_resolution_clock::now();
    auto duration1 = std::chrono::duration_cast<std::chrono::duration<double, std::milli>>(clock2 - clock1).count();

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

    // Add witness values
    pb.val(x) = 3;  // x = 3
    pb.val(x_squared) = pb.val(x) * pb.val(x);  // x^2 = 9
    pb.val(x_cubed) = pb.val(x_squared) * pb.val(x);  // x^3 = 27
    pb.val(out) = 28;  // Set an incorrect output (should be 31)

    // Create proof
    const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof1 = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(
            keypair.pk, pb.primary_input(), pb.auxiliary_input());

    // Verify proof
    bool verified1 = r1cs_se_ppzksnark_verifier_strong_IC<default_r1cs_se_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof1);

    cout << "FOR UNSUCCESSFUL VERIFICATION" << endl;
    cout << "Number of R1CS constraints: " << constraint_system.num_constraints() << endl;
    cout << "Primary (public) input: " << pb.primary_input() << endl;
    cout << "Auxiliary (private) input: " << pb.auxiliary_input() << endl;
    cout << "Verification status: " << verified1 << endl;
    cout << "Satisfied status: " << pb.is_satisfied() << endl;

    return 0;
}