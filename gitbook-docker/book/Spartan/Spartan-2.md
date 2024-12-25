# 1.SNARKGens::new
- 功能:初始化 SNARK 系统的公共参数，用于生成和验证零知识证明。
- 输入:	
    1.	num_cons: R1CS 中的约束数量。
	2.	num_vars: R1CS 中的变量数量。
	3.	num_inputs: R1CS 中的输入数量。
	4.	num_non_zero_entries: 约束矩阵中的非零条目总数。
- 输出:	返回一个 SNARKGens 对象，包含用于生成证明的参数与用于验证证明的参数。
# 2.Transcript::new
- 功能:创建一个新的 Transcript 对象，用于记录和追踪 SNARK 证明的交互过程,实现 Fiat-Shamir 转换以消除交互。
- 输入:字节数组，用作 Transcript 的标识或上下文。
- 输出:一个 Transcript 对象，记录证明生成过程中的交互信息。
# 3. SNARK::encode
- 功能:用于对 R1CS 实例进行承诺（commitment），生成一个可验证的承诺（comm）和解承诺信息(decomm);
- 输入:	
    1.  &inst:一个 R1CS 实例，定义了约束系统，包括稀疏矩阵  A ,  B ,  C ，约束数量、变量数量和输入数量。
	2.	&gens:SNARK 的公共参数，由 SNARKGens::new 初始化。
- 输出:
    一个二元组 (comm, decomm)：comm:一个类型为Commitment的承诺对象，表示对 R1CS 系统的承诺;decomm:一个类型为Decommitment的解承诺对象，包含验证承诺所需的附加信息。
# 3.SNARK::prove
- 功能:使用 R1CS 系统和分配的变量/输入生成一个 SNARK 证明。
- 输入:	
    1.	&inst: R1CS 实例，定义了约束系统，包括稀疏矩阵 A,B,C 。
	2.	&comm: R1CS 系统的承诺，生成自 SNARK::encode。
	3.	&decomm: 解承诺信息，与承诺一同生成。
	4.	assignment_vars: R1CS 系统中所有变量的赋值。
	5.	&assignment_inputs: R1CS 系统中所有公共输入的赋值。
	6.	&gens: SNARK 的公共参数，由 SNARKGens::new 生成。
	7.	&mut prover_transcript: 用于记录证明过程的 Transcript 对象。
- 输出:proof: 一个 SNARK 证明对象，包含满足 R1CS 的证明信息。
# 4.proof.verify
- 功能:验证 SNARK 证明的正确性，确保证明者生成的证明确实满足 R1CS 系统的约束条件。
- 输入:
    1.	&comm:对 R1CS 实例的承诺，由 SNARK::encode 生成。
	2.	&assignment_inputs:公共输入值的分配，由 InputsAssignment::new 生成。
	3.	&mut verifier_transcript:用于记录验证过程的随机挑战和响应。
	4.	&gens:SNARK 的公共参数，由 SNARKGens::new 初始化。
- 输出:	如果验证成功，返回 Ok(());如果验证失败，返回一个错误，表示证明不符合约束条件。

