/**@file
 *****************************************************************************
 R1CS zk-STARK obtained by combining our holographic IOP for R1CS
 and the BCS16 transformation.
 *****************************************************************************
 * @author     This file is part of libiop (see AUTHORS)
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
#ifndef LIBIOP_SNARK_FRACTAL_SNARK_HPP_
#define LIBIOP_SNARK_FRACTAL_SNARK_HPP_

#include <cstddef>
#include <iostream>

#include "libiop/protocols/fractal_hiop.hpp"
#include "libiop/protocols/ldt/fri/fri_ldt.hpp"
#include "libiop/protocols/ldt/ldt_reducer.hpp"
#include "libiop/bcs/bcs_common.hpp"
#include "libiop/bcs/bcs_indexer.hpp"
#include "libiop/bcs/bcs_prover.hpp"
#include "libiop/bcs/bcs_verifier.hpp"
#include "libiop/relations/r1cs.hpp"

namespace libiop {

template<typename FieldT, typename hash_type>
class fractal_snark_parameters {
    protected:
    size_t security_parameter_;
    LDT_reducer_soundness_type LDT_reducer_soundness_type_;
    FRI_soundness_type FRI_soundness_type_;
    size_t RS_extra_dimensions_;
    bool make_zk_;
    field_subset_type domain_type_;
    std::shared_ptr<r1cs_constraint_system<FieldT>> constraint_system_;

    std::vector<size_t> FRI_localization_parameter_array_;
    size_t FRI_localization_parameter_;

    void initialize_bcs_params(const bcs_hash_type hash_enum);
    void initialize_iop_params();
    public:
    fractal_snark_parameters(
        const size_t security_parameter,
        const LDT_reducer_soundness_type ldt_reducer_soundness_type,
        const FRI_soundness_type fri_soundness_type,
        const bcs_hash_type hash_enum,
        const std::vector<size_t> FRI_localization_parameter_array,
        const size_t RS_extra_dimensions,
        const bool make_zk,
        const field_subset_type domain_type,
        const std::shared_ptr<r1cs_constraint_system<FieldT>> constraint_system);

    fractal_snark_parameters(
        const size_t security_parameter,
        const LDT_reducer_soundness_type ldt_reducer_soundness_type,
        const FRI_soundness_type fri_soundness_type,
        const bcs_hash_type hash_enum,
        const size_t FRI_localization_parameter,
        const size_t RS_extra_dimensions,
        const bool make_zk,
        const field_subset_type domain_type,
        const std::shared_ptr<r1cs_constraint_system<FieldT>> constraint_system);

    void reset_fri_localization_parameters(const std::vector<size_t> FRI_localization_parameter_array);
    void print() const;

    bcs_transformation_parameters<FieldT, hash_type> bcs_params_;
    fractal_iop_parameters<FieldT> iop_params_;
};

template<typename FieldT, typename hash_type>
using fractal_snark_argument = bcs_transformation_transcript<FieldT, hash_type>;

template<typename FieldT, typename hash_type>
std::pair<bcs_prover_index<FieldT, hash_type>, bcs_verifier_index<FieldT, hash_type>>
fractal_snark_indexer(
    const fractal_snark_parameters<FieldT, hash_type> &parameters);

/** This mutates the bcs_prover_index.
 *  It is expected that the end user makes a copy before passing it in here */
template<typename FieldT, typename hash_type>
fractal_snark_argument<FieldT, hash_type> fractal_snark_prover(
    bcs_prover_index<FieldT, hash_type> &index,
    const r1cs_primary_input<FieldT> &primary_input,
    const r1cs_auxiliary_input<FieldT> &auxiliary_input,
    const fractal_snark_parameters<FieldT, hash_type> &parameters);

template<typename FieldT, typename hash_type>
bool fractal_snark_verifier(
    const bcs_verifier_index<FieldT, hash_type> &index,
    const r1cs_primary_input<FieldT> &primary_input,
    const fractal_snark_argument<FieldT, hash_type> &proof,
    const fractal_snark_parameters<FieldT, hash_type> &parameters);

} // namespace libiop

#include "libiop/snark/fractal_snark.tcc"

#endif // LIBIOP_SNARK_FRACTAL_SNARK_HPP_
