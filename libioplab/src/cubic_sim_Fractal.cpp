#include <cstdint>
#include <iostream>
#include <chrono>
#include <libff/algebra/curves/edwards/edwards_pp.hpp>
#include "libiop/snark/fractal_snark.hpp"
#include "libiop/relations/examples/r1cs_examples.hpp"

using namespace libiop;
using namespace std;

int main() {
    // 禁用所有输出
    libff::inhibit_profiling_info = true;
    libff::inhibit_profiling_counters = true;

    // 初始化 Edwards 曲线参数（静默模式）
    libff::edwards_pp::init_public_params();
    typedef libff::edwards_Fr FieldT;
    typedef binary_hash_digest hash_type;

    // 设置参数
    const size_t num_constraints = 1 << 6;  
    const size_t num_inputs = (1 << 5)-1;    
    const size_t num_variables = (1 << 6)-1;   
    const size_t security_parameter = 128;
    const size_t RS_extra_dimensions = 3;
    const size_t FRI_localization_parameter = 3;
    const LDT_reducer_soundness_type ldt_reducer_soundness_type = LDT_reducer_soundness_type::optimistic_heuristic;
    const FRI_soundness_type fri_soundness_type = FRI_soundness_type::heuristic;
    const field_subset_type domain_type = multiplicative_coset_type;

    try {
        // 生成 R1CS 实例（静默模式）
        r1cs_example<FieldT> r1cs_params = generate_r1cs_example<FieldT>(
            num_constraints, num_inputs, num_variables);

        if (!r1cs_params.constraint_system_.is_satisfied(
            r1cs_params.primary_input_, r1cs_params.auxiliary_input_)) {
            return 1;
        }

        std::shared_ptr<r1cs_constraint_system<FieldT>> cs =
            std::make_shared<r1cs_constraint_system<FieldT>>(r1cs_params.constraint_system_);

        // 设置 SNARK 参数
        const bool make_zk = true;
        fractal_snark_parameters<FieldT, hash_type> params(
            security_parameter,
            ldt_reducer_soundness_type,
            fri_soundness_type,
            blake2b_type,
            FRI_localization_parameter,
            RS_extra_dimensions,
            make_zk,
            domain_type,
            cs);


        // 生成索引
        std::pair<bcs_prover_index<FieldT, hash_type>, bcs_verifier_index<FieldT, hash_type>> index =
            fractal_snark_indexer(params);

        // 生成证明
        auto proving_start = chrono::high_resolution_clock::now();
        
        const fractal_snark_argument<FieldT, hash_type> argument = fractal_snark_prover<FieldT, hash_type>(
            index.first,
            r1cs_params.primary_input_,
            r1cs_params.auxiliary_input_,
            params);

        auto proving_end = chrono::high_resolution_clock::now();
        auto proving_time = chrono::duration_cast<chrono::milliseconds>(proving_end - proving_start).count();

        // 验证证明
        auto verify_start = chrono::high_resolution_clock::now();
        
        const bool verification_result = fractal_snark_verifier<FieldT, hash_type>(
            index.second,
            r1cs_params.primary_input_,
            argument,
            params);

        auto verify_end = chrono::high_resolution_clock::now();
        auto verify_time = chrono::duration_cast<chrono::milliseconds>(verify_end - verify_start).count();

        // 只输出关键信息
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
