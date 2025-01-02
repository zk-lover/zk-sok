# 1. PedersenGens::default()
- 无显示输入;
- 输出:类型为PedersenGens,表示用于生成 Pedersen 承诺的默认生成器。
- 功能：提供生成 Pedersen 承诺所需的基础参数。
# 2. BulletproofGens::new(x,y)
- 输入:x: 最大支持的证明范围，以位数为单位;y: 支持的最大证明聚合大小。
- 输出:类型为BulletproofGens的Bulletproofs 生成器，用于创建范围证明。
- 功能:生成 Bulletproofs 的公共参数，适用于单方或多方范围证明。