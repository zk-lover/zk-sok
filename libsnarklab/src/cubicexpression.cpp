#include <stdlib.h>
#include <iostream>
#include <chrono>
#include "libsnark/zk_proof_systems/ppzksnark/r1cs_se_ppzksnark/r1cs_se_ppzksnark.hpp"
#include "libsnark/common/default_types/r1cs_se_ppzksnark_pp.hpp"
#include "libsnark/gadgetlib1/pb_variable.hpp"

using namespace libsnark;
using namespace std;

int main() {
  
    libff::inhibit_profiling_info = true;
    
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

    // Allocate variables
    out.allocate(pb, "out");
    x.allocate(pb, "x");
    x_squared.allocate(pb, "x_squared");
    x_cubed.allocate(pb, "x_cubed");

    pb.set_input_sizes(1);

    // Add constraints
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x, x, x_squared));
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x_squared, x, x_cubed));
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x_cubed + x + 1, 1, out));

    auto clock1 = std::chrono::high_resolution_clock::now();
    
    const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();
    const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(constraint_system);

    // Add witness values
    pb.val(x) = 3;
    pb.val(x_squared) = pb.val(x) * pb.val(x);
    pb.val(x_cubed) = pb.val(x_squared) * pb.val(x);
    pb.val(out) = pb.val(x_cubed) + pb.val(x) + FieldT(1);

    const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(
            keypair.pk, pb.primary_input(), pb.auxiliary_input());
    auto clock2 = std::chrono::high_resolution_clock::now();
    auto proving_time = std::chrono::duration_cast<std::chrono::milliseconds>(clock2 - clock1).count();

    auto clock3 = std::chrono::high_resolution_clock::now();
    bool verified = r1cs_se_ppzksnark_verifier_strong_IC<default_r1cs_se_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof);
    auto clock4 = std::chrono::high_resolution_clock::now();
    auto verification_time = std::chrono::duration_cast<std::chrono::milliseconds>(clock4 - clock3).count();

    cout << "Number of constraints: " << constraint_system.num_constraints() << endl;
    cout << "Proof size (bytes): " << proof.size_in_bits()/8 << endl;
    cout << "Proving time (ms): " << proving_time << endl;
    cout << "Verification time (ms): " << verification_time << endl;

    return 0;
}