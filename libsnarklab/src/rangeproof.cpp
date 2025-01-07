#include <stdlib.h>
#include <iostream>
#include <chrono>

#include <libsnark/common/default_types/r1cs_se_ppzksnark_pp.hpp>
#include <libsnark/zk_proof_systems/ppzksnark/r1cs_se_ppzksnark/r1cs_se_ppzksnark.hpp>
#include <libsnark/gadgetlib1/pb_variable.hpp>
#include <libsnark/gadgetlib1/gadgets/basic_gadgets.hpp>

using namespace libsnark;
using namespace std;

int main () {
    // 禁用详细的 profiling 输出
    libff::inhibit_profiling_info = true;
    
    typedef libff::Fr<default_r1cs_se_ppzksnark_pp> FieldT;

    // Initialize the curve parameters
    default_r1cs_se_ppzksnark_pp::init_public_params();
  
    // Create protoboard
    protoboard<FieldT> pb;

    pb_variable<FieldT> x, max;
    pb_variable<FieldT> less, less_or_eq;

    x.allocate(pb, "x");
    max.allocate(pb, "max");
    less.allocate(pb, "less");
    less_or_eq.allocate(pb, "less_or_eq");
    
    pb.val(max) = FieldT::one();
    FieldT two = FieldT::one() + FieldT::one();
    for (int i = 0; i < 32; ++i) {
        pb.val(max) = pb.val(max) * two;
    }

    auto clock1 = std::chrono::high_resolution_clock::now();
    comparison_gadget<FieldT> cmp(pb, 32, x, max, less, less_or_eq, "cmp");
    cmp.generate_r1cs_constraints();

    const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();

    // generate keypair
    const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(constraint_system);

    // Add witness values
    pb.val(x) = 18;
    cmp.generate_r1cs_witness();

    // generate proof
    const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input());
    auto clock2 = std::chrono::high_resolution_clock::now();
    auto proving_time = std::chrono::duration_cast<std::chrono::milliseconds>(clock2 - clock1).count();

    // verify
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