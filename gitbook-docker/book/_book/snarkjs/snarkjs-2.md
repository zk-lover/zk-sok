# 1.生成公共和私有参数

零知识证明需要一个可信的初始化步骤，通常称为 可信设置。使用 snarkjs 可以生成一个可信设置的公共和私有参数。

首先，使用 snarkjs 来生成所需的参数：
```
snarkjs plonk setup circuit.r1cs pot14_final.ptau circuit_final.zkey
```
该命令以编译circom文件生成的r1cs约束为输入,生成ptau公共参数文件和circuit_final密钥文件;

# 2. 计算见证
在创建任何证明之前，我们需要计算与（所有）电路约束匹配的所有电路信号。这些信号就是“见证”。

首先，创建一个```input.json```文件：
```
{
  "x": 3
}
```
运行命令计算见证
```
snarkjs wtns calculate circuit.wasm input.json witness.wtns
```
该命令以编译circom文件生成的wasm文件和input.json文件为输入,生成witness.wtns文件;

# 3. 密钥检查
```
snarkjs zkey verify circuit.r1cs pot14_final.ptau circuit_final.zkey
```
该命令以circuit.r1cs约束文件,pot14_final.ptau公共参数和circuit_final.zkey密钥文件为输入，验证密钥的正确性；
# 4. 密钥导出
```
snarkjs zkey export verificationkey circuit_final.zkey verification_key.json
``` 
验证密钥之后，将circuit_final.zkey转换为json格式的verification_key.json;
# 5. 生成证明
```
snarkjs plonk prove circuit_final.zkey witness.wtns proof.json public.json
```
该命令以circuit_final.zkey密钥和witness.wtns见证为输入，生成proof.json和public.json文件；
其中,proof.json文件是实际上的证明文件；public.json文件则包含了公共的输入与输出;

# 6. 验证证明
```
snarkjs plonk verify verification_key.json public.json proof.json
```
在接收到verification_key.json、public.json和proof.json文件后，输出对proof的验证结果；