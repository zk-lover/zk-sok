# `libsnark` ZKP Documentation

In this section, we will introduce the relevant APIs of libsnark's proof system.

## 1.r1cs_ppzksnark_generator<default_r1cs_ppzksnark_pp>(constraint_system)
- Input: A constraint circuit of type r1cs_constraint_system.
- Output: A r1cs_ppzksnark_keypair type key pair.
- Function: Generates a zero-knowledge proof key pair based on the given constraint system.

## 2.r1cs_ppzksnark_prover<default_r1cs_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input())
- Input: keypair.pk (public key), pb.primary_input() (public input), pb.auxiliary_input() (auxiliary input).
- Output: A r1cs_ppzksnark_proof (generated proof).
- Function: Generates a zero-knowledge proof for the correctness of the computation based on the provided inputs and key pair.

## 3.r1cs_ppzksnark_verifier_strong_IC<default_r1cs_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof)
- Input: keypair.vk (verification key), pb.primary_input() (public input), proof (proof).
- Output: A boolean variable verified (verification result, true indicates successful verification).
- Function: Verifies the correctness of the computation using the verification key and proof.

libsnark supports multiple zkp, and you can select different zkp by modifying default_r1cs_ppzksnark_pp. Common zkp names and corresponding relationships are as follows:
```
BCTV14: default_r1cs_ppzksnark_pp
Groth16: default_r1cs_gg_ppzksnark_pp
GM17: default_r1cs_se_ppzksnark_pp
```