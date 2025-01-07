# 1.data.prove(pw)
- 功能:使用电路数据和见证，生成一个零知识证明。
- 输入:pw,类型为PartialWitness,包含电路中所有目标变量的具体值;data,类型为CircuitData,电路数据，包含约束和操作信息,作为隐式输入。
- 输出:类型为Proof,返回一个零知识证明对象。
# 2.data.verify(proof)
- 功能:验证一个零知识证明是否有效。
- 输入:proof:类型为Proof,由 data.prove 生成的证明对象;
- 输出: Result<()>,表示验证是否成功。
