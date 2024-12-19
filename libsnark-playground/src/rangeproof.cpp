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
    typedef libff::Fr<default_r1cs_se_ppzksnark_pp> FieldT;

    // Initialize the curve parameters
    default_r1cs_se_ppzksnark_pp::init_public_params();
  
    // Create protoboard
    protoboard<FieldT> pb;

    pb_variable<FieldT> x, max;
    pb_variable<FieldT> less, less_or_eq;

    x.allocate(pb, "x");
    max.allocate(pb, "max");
    less.allocate(pb, "less"); // must have
    less_or_eq.allocate(pb, "less_or_eq");
    
    pb.val(max) = FieldT::one(); // 从 1 开始
    FieldT two = FieldT::one() + FieldT::one(); // 计算 2
    for (int i = 0; i < 32; ++i) {
        pb.val(max) = pb.val(max) * two; // 重复乘以 2，共 32 次
    }

    auto clock1 = std::chrono::high_resolution_clock::now();
    comparison_gadget<FieldT> cmp(pb, 32, x, max, less, less_or_eq, "cmp");
    cmp.generate_r1cs_constraints();

    const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();

    // generate keypair
    const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(constraint_system);

    // Add witness values
    pb.val(x) = 18; // secret
    cmp.generate_r1cs_witness();

    // generate proof
    const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input());
    auto clock2 = std::chrono::high_resolution_clock::now();
    auto duration1 = std::chrono::duration_cast<std::chrono::duration<double, std::milli>>(clock2 - clock1).count();

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

    return 0;
}