#include <cstdint>
#include <iostream>
#include <chrono>
#include <libff/algebra/curves/alt_bn128/alt_bn128_pp.hpp>
#include "libiop/snark/ligero_snark.hpp"
#include "libiop/relations/examples/r1cs_examples.hpp"
#include "libiop/bcs/common_bcs_parameters.hpp"

using namespace libiop;
using namespace std;

int main() {
  
    libff::inhibit_profiling_info = true;
    libff::inhibit_profiling_counters = true;

   
    libff::alt_bn128_pp::init_public_params();
    typedef libff::alt_bn128_Fr FieldT;

 
    const size_t num_constraints = 39;  
    const size_t constraint_dim = 8;   
    const size_t num_inputs = (1 << 5) - 1;
    const size_t num_variables = 39 - 1;

    try {
     
        r1cs_example<FieldT> r1cs_params = generate_r1cs_example<FieldT>(
            num_constraints, num_inputs, num_variables);

        if (!r1cs_params.constraint_system_.is_satisfied(
            r1cs_params.primary_input_, r1cs_params.auxiliary_input_)) {
            return 1;
        }

  
        ligero_snark_parameters<FieldT, binary_hash_digest> parameters;
        parameters.security_level_ = 128;
        parameters.height_width_ratio_ = 0.001;
        parameters.RS_extra_dimensions_ = 2;
        parameters.make_zk_ = true;
        parameters.domain_type_ = multiplicative_coset_type;
        parameters.LDT_reducer_soundness_type_ = LDT_reducer_soundness_type::proven;
        parameters.bcs_params_ = default_bcs_params<FieldT, binary_hash_digest>(
            blake2b_type, parameters.security_level_, constraint_dim);

     
        auto proving_start = chrono::high_resolution_clock::now();
        
        const ligero_snark_argument<FieldT, binary_hash_digest> argument = 
            ligero_snark_prover<FieldT, binary_hash_digest>(
                r1cs_params.constraint_system_,
                r1cs_params.primary_input_,
                r1cs_params.auxiliary_input_,
                parameters);

        auto proving_end = chrono::high_resolution_clock::now();
        auto proving_time = chrono::duration_cast<chrono::milliseconds>(proving_end - proving_start).count();

    
        auto verify_start = chrono::high_resolution_clock::now();
        
        const bool verification_result = ligero_snark_verifier<FieldT, binary_hash_digest>(
            r1cs_params.constraint_system_,
            r1cs_params.primary_input_,
            argument,
            parameters);

        auto verify_end = chrono::high_resolution_clock::now();
        auto verify_time = chrono::duration_cast<chrono::milliseconds>(verify_end - verify_start).count();

 
        cout << "Constraints: " << num_constraints << endl;
        cout << "Proof size: " << argument.size_in_bytes() << " bytes" << endl;
        cout << "Proving time: " << proving_time << " ms" << endl;
        cout << "Verify time: " << verify_time << " ms" << endl;

        return verification_result ? 0 : 1;

    } catch (const exception& e) {
        cerr << "Error: " << e.what() << endl;
        return 1;
    }
}
