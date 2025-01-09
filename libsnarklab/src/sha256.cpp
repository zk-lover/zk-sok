#include <stdlib.h>
#include <iostream>
#include <chrono>

#include <libsnark/zk_proof_systems/ppzksnark/r1cs_se_ppzksnark/r1cs_se_ppzksnark.hpp>
#include "libsnark/common/default_types/r1cs_se_ppzksnark_pp.hpp"
#include "libsnark/gadgetlib1/pb_variable.hpp"
#include "libsnark/gadgetlib1/gadgets/hashes/sha256/sha256_gadget.hpp"

using namespace libsnark;
using namespace std;

typedef libff::Fr<default_r1cs_se_ppzksnark_pp> FieldT;

template<typename ppT, typename FieldT>
bool verify_proof(r1cs_se_ppzksnark_verification_key<ppT> verification_key, r1cs_primary_input<FieldT> primary_input,
        r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof) {
    return r1cs_se_ppzksnark_verifier_strong_IC<default_r1cs_se_ppzksnark_pp>(verification_key, primary_input, proof);
}

r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> setup_gadget(protoboard<FieldT> &pb, block_variable<FieldT> *&inp, digest_variable<FieldT> *&out, sha256_two_to_one_hash_gadget<FieldT> *&g) {
    inp = new block_variable<FieldT>(pb, SHA256_block_size, "input");
    out = new digest_variable<FieldT>(pb, SHA256_digest_size, "output");
    g = new sha256_two_to_one_hash_gadget<FieldT>(pb, SHA256_block_size, *inp, *out, "f");
    g->generate_r1cs_constraints();

    const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();
    const r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = r1cs_se_ppzksnark_generator<default_r1cs_se_ppzksnark_pp>(
            constraint_system);
    return keypair;
}

int one_input_hash_gadget(int num_iterations) {
    using namespace std::chrono;

    protoboard<FieldT> pb;
    block_variable<FieldT>* input;
    digest_variable<FieldT>* output;
    sha256_two_to_one_hash_gadget<FieldT>* f;

    r1cs_se_ppzksnark_keypair<default_r1cs_se_ppzksnark_pp> keypair = setup_gadget(pb, input, output, f);

    int i = 0;
    const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();

    duration<double> tc(0);
    duration<double> tp(0);
    duration<double> tv(0);

    while (i < num_iterations) {
        const libff::bit_vector hash_bv = libff::int_list_to_bits({0x605b0cd0, 0xc4f79cc4, 0x232a1c0f, 0xcdd92dd6, 0x4f0d8cd0, 0x66c610d4, 0x82ab2037, 0xb0d7c550}, 32);
        output->generate_r1cs_witness(hash_bv);

        steady_clock::time_point begin = steady_clock::now();
        const libff::bit_vector input_bv = libff::int_list_to_bits({0x68756461, 0x69710000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000}, 32);
        input->generate_r1cs_witness(input_bv);
        f->generate_r1cs_witness();
        steady_clock::time_point mid = steady_clock::now();
        tc += duration_cast<duration<double>>(mid - begin);

        const r1cs_se_ppzksnark_proof<default_r1cs_se_ppzksnark_pp> proof = r1cs_se_ppzksnark_prover<default_r1cs_se_ppzksnark_pp>(
                keypair.pk, pb.primary_input(), pb.auxiliary_input());
        steady_clock::time_point end = steady_clock::now();
        tp += duration_cast<duration<double>>(end - mid);

        steady_clock::time_point begin1 = steady_clock::now();
        bool verified = verify_proof(keypair.vk, pb.primary_input(), proof);
        steady_clock::time_point end1 = steady_clock::now();
        if (!verified) return -1;
        tv += duration_cast<duration<double>>(end1 - begin1);

        i++;
    }

    cout << "Number of constraints: " << constraint_system.num_constraints() << endl;
    cout << "Proof size (bytes): " << 1019/8 << endl; 
    cout << "Proving time (ms): " << tp.count() * 1000 << endl;
    cout << "Verification time (ms): " << tv.count() * 1000 << endl;

    return 1;
}

int main() {
    // Disable detailed profiling output
    libff::inhibit_profiling_info = true;
    libff::inhibit_profiling_counters = true;
    
    default_r1cs_se_ppzksnark_pp::init_public_params();

    int num_iterations = 1;
    one_input_hash_gadget(num_iterations);

    return 0;
}