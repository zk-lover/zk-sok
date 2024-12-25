# 1.aurora
## 1.aurora_snark_parameters<FieldT, hash_type> params
- 功能:重要的变量类型,用于配置 SNARK 证明和验证的参数。
- 构造函数输入:
	1. security_parameter: 安全参数，定义系统的抗攻击能力。
	2. ldt_reducer_soundness_type: LDT（低度测试）简化器的健全性类型。
	3. fri_soundness_type: FRI（快速交互式证明）健全性类型。
	4. blake2b_type: 哈希函数。
	5. FRI_localization_parameter: FRI 本地化参数。
	6. RS_extra_dimensions: Reed-Solomon 码的额外维度。
	7. make_zk: 布尔值，是否启用零知识特性。
    8. domain_type: 域类型（如仿射子空间）。
	9. num_constraints: R1CS 中的约束数量。
    10. num_variables: R1CS 的变量数量。
# 2.is_satisfied
- 功能:约束系统的成员函数,验证 R1CS 系统是否满足输入数据（主输入和辅助输入）。
- 输入:
    1. primary_input_: 主输入变量值。
	2. auxiliary_input_: 辅助输入变量值。
- 输出: 一个bool变量,表示证明是否正确。
## 3.aurora_snark_prover
- 功能:生成aurora_snark证明，用于证明输入数据满足指定的 R1CS 系统。
- 输入:
    1. constraint_system_: R1CS 的约束系统。
	2. primary_input_: 主输入变量值。
	3. auxiliary_input_: 辅助输入变量值。
	4. params: SNARK 参数，包括安全性参数、域类型等。
- 输出:一个 aurora_snark_argument 对象
## 4.aurora_snark_verifier
- 功能:验证证明的正确性，确保证明满足给定的 R1CS 系统。
- 输入:
    1. constraint_system_: R1CS 的约束系统。
	2. primary_input_: 主输入变量值。
	3. argument: 由 aurora_snark_prover 生成的 SNARK 证明。
	4. params: SNARK 参数。
- 输出: 一个bool变量,表示证明是否正确。

# 2.3.同1