# 1.r1cs_ppzksnark_generator<default_r1cs_ppzksnark_pp>(constraint_system)
- 输入:r1cs_constraint_system类型的约束电路;
- 输出:r1cs_ppzksnark_keypair类型的密钥;
- 功能:通过给定的约束系统生成零知识证明的密钥对。

# 2.r1cs_ppzksnark_prover<default_r1cs_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input())
- 输入:keypair.pk（公钥），pb.primary_input()（公开输入），pb.auxiliary_input()（辅助输入）
- 输出:r1cs_ppzksnark_proof（生成的证明）
- 功能:基于提供的输入和密钥对，生成用于证明计算正确性的零知识证明。

# 3.r1cs_ppzksnark_verifier_strong_IC<default_r1cs_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof)
- 输入: keypair.vk（验证密钥），pb.primary_input()（公开输入），proof（证明）
- 输出: bool变量verified（验证结果，true 表示验证成功）
- 功能: 使用验证密钥和证明来验证计算是否正确。