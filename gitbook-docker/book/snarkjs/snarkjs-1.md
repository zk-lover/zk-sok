# 1.基本结构

Circom 代码由多个 **模板**（template）和 **主电路**（main circuit）组成。每个模板定义了一个电路的逻辑，模板可以在主电路或其他模板中进行实例化。

## 模板声明

```circom
template MyTemplate() {
    // 电路内部的逻辑
}
```
## 主电路
``` circom
template MainCircuit() {
    signal input a;
    signal input b;
    signal output c;

    component myTemplate1 = MyTemplate();
    myTemplate1.a <== a;
    myTemplate1.b <== b;
    c <== myTemplate1.out;
}
component main = MainCircuit();
```

# 2.信号

信号是电路的输入、输出和中间变量。在 Circom 中，所有的信号必须明确声明。

- **输入信号（input）**：电路的输入变量
- **输出信号（output）**：电路的输出变量
- **中间信号**：电路内部计算的变量

## 示例

```circom
signal input a; // 输入信号 a
signal output b; // 输出信号 b
signal x; // 中间信号 x
```

# 3.约束

Circom 电路由一系列约束构成，约束用于确保输入满足特定的关系。Circom 使用一种类似数学公式的语法来定义约束。

## 基本约束

可以将信号与常数值、其他信号进行关系定义。

```circom
signal a;
signal b;
signal c;

a <== b + 1; // 约束 a = b + 1
c <== a * a; // 约束 c = a * a
```
## 公共约束（assert）

用于检查电路约束是否成立。

```
assert(a == b + 1); // 确保 a = b + 1
```

# 4.算术运算

Circom 允许进行基本的算术运算（加、减、乘、除），每个`<===`只能包含一个乘法，这是为了满足R1CS的要求，多个乘法可以拆分计算。

## 示例

```circom
signal x, y, z;

x <== y + 5;    // 加法
z <== x * y;    // 乘法
```


# 5.模版实例化

模板是 Circom 的核心结构。一个模板可以被实例化为一个或多个组件，实例化时可以给模板中的输入信号赋值。

## 示例

```circom
template Adder() {
    signal input a;
    signal input b;
    signal output c;
    c <== a + b;
}

template Main() {
    signal input x;
    signal input y;
    signal output z;

    component adder1 = Adder(); // 实例化 Adder 模板
    adder1.a <== x;
    adder1.b <== y;
    z <== adder1.c; // 获取加法结果
}
```
# 6.导入和模块化

Circom 支持将电路划分为多个文件。可以通过 include 关键字引入其他 Circom 文件。

## 示例
```
include "somefile.circom";
```
# 7.编译
Circom 的电路定义完成后，通常使用 circom 工具将电路编译为可以在 zk-SNARK 系统中使用的形式。编译步骤通常涉及将电路文件转换为 R1CS 文件（关系约束系统），并进一步生成证明和验证算法。

# 示例

```
circom my_circuit.circom --r1cs --wasm --sym
```
- --r1cs：生成 R1CS 格式的约束
- --wasm：生成 WebAssembly 模块，供生成证明使用
- --sym：生成符号表，帮助调试
