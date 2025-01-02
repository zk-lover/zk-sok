# 1.RangeProof::prove_single
- 输入:
	1. &bp_gens: Bulletproofs 生成器，用于生成证明。
	2. &pc_gens: Pedersen 生成器，用于生成承诺。
	3. &mut prover_transcript: 传输对象，记录公共信息。
	4. secret_value: 要进行范围证明的秘密值。
	5. &blinding: Pedersen 承诺的盲因子。
	6. 32: 证明范围的位数。
- 输出:
	1. RangeProof: 生成的范围证明。
	2. CompressedRistretto: 对秘密值的承诺。
- 功能:为给定的秘密值生成一个范围证明，表明值位于特定范围内。
# 2.RangeProof::prove_multiple
- 输入:
    1. &bp_gens: Bulletproofs 公共参数。
    2. &pc_gens: Pedersen 承诺生成器。
    3. &mut transcript: 用于记录证明过程的 Merlin 传输。
    4. &values[0..*m]: 要证明范围的多个秘密值。
    5. &blindings[0..*m]: 每个值的盲因子。
    6. *n: 范围的位数。
- 输出:
    -类型为: (RangeProof, Vec<CompressedRistretto>)
    - RangeProof: 生成的范围证明。
    - Vec<CompressedRistretto>: 每个秘密值的承诺。
- 功能:为多个秘密值生成范围证明，确保所有值位于 [0, 2^n)  范围内。
# 3.RangeProof::verify_single
- 输入:
    1. &bp_gens: Bulletproofs 生成器，用于验证证明。
    2. &pc_gens: Pedersen 生成器，用于验证承诺。
    3. &mut verifier_transcript: 传输对象，与证明阶段保持一致。
    4. &committed_value: 提供的 Pedersen 承诺。
    5. 32: 验证范围的位数。
- 输出:Ok(()): 表示证明验证成功;或者Err(R1CSError): 表示证明验证失败。
- 功能:验证范围证明的正确性，确保秘密值的承诺确实符合范围约束。
# 4.RangeProof::verify_multiple
- 输入:
    1. &bp_gens: Bulletproofs 公共参数。
    2. &pc_gens: Pedersen 承诺生成器。
    3. &mut transcript: 用于验证过程的 Merlin 传输。
    4. &vc[0..m]: 与证明关联的值承诺。
    5. n: 范围位数。
- 输出:Ok(()): 验证成功；或者Err(R1CSError): 验证失败。
- 功能:验证多个值的范围证明，确保所有值的承诺符合给定范围。