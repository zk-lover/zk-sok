# 1.Groth16::setup
- 功能:根据输入的电路生成一组用于证明和验证的密钥;
- 输入:
    1.一个实现了<ConstraintF: Field> ConstraintSynthesizer<ConstraintF>中generate_constraints方法的电路；
    2.&mut rng:一个伪随机数生成器;
- 输出:一个元组 (pk, vk):
    1. pk (Proving Key):类型为ark_groth16::ProvingKey<E>
    2. vk (Verifying Key):类型为ark_groth16::VerifyingKey<E>
# 2.Groth16::prove
- 功能: 用于根据电路约束、证明密钥、公钥和输入生成一个零知识证明（proof）。
- 输入:	
    1.	&pk:类型为ProvingKey<E>,证明密钥，由 Groth16::setup 生成;
	2.  实现了 ConstraintSynthesizer 的电路实例。
	3.	&mut rng:伪随机数生成器。
- 输出:proof:类型为Proof<E>,一个零知识证明对象，表示输入x和输出y满足电路约束。
# 3.prepare_verifying_key
- 功能:对验证密钥（VerifyingKey）进行预处理，生成优化后的验证密钥（PreparedVerifyingKey）
- 输入:验证密钥（VerifyingKey）
- 输出:优化后的验证密钥（PreparedVerifyingKey）
# 4.Groth16::verify_with_processed_vk
- 功能：用于验证一个零知识证明的正确性。
- 输入:
    1.	&pvk:类型为&PreparedVerifyingKey<E>,优化后的验证密钥，由 prepare_verifying_key 生成。
	2.	&[y]:类型为切片数组（&[E::ScalarField]）,表示电路的公共输出y.
	3.	&proof:类型为&Proof<E>，零知识证明，由 Groth16::prove 生成。