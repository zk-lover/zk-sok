## 1. 约束与电路构建

### 1.1 `frontend.API`（电路定义和操作）

`frontend.API` 是用来定义电路约束的接口，它提供了多种操作（如加法、乘法、常量、变量等），用于表达电路中各个变量之间的关系。

#### 常见方法：

- **`api.Add(a, b frontend.Variable) frontend.Variable`**
  - **输入**: `a` 和 `b` 为电路中的变量或常量（`frontend.Variable`）。
  - **输出**: 返回 `a + b` 的计算结果。
  - **功能**: 实现加法操作。

- **`api.Mul(a, b frontend.Variable) frontend.Variable`**
  - **输入**: `a` 和 `b` 为电路中的变量或常量（`frontend.Variable`）。
  - **输出**: 返回 `a * b` 的计算结果。
  - **功能**: 实现乘法操作。

- **`api.Constant(c int64) frontend.Variable`**
  - **输入**: `c` 是一个整数常量。
  - **输出**: 返回常量 `c` 的 `frontend.Variable`。
  - **功能**: 用于生成电路中的常量。

- **`api.AssertIsEqual(a, b frontend.Variable)`**
  - **输入**: `a` 和 `b` 为电路中的变量。
  - **输出**: 无返回值（如果约束不成立则抛出错误）。
  - **功能**: 强制 `a` 和 `b` 相等，用于建立约束。

- **`rangecheck.New(api frontend.API)`**
  - **输入**: 一个`frontend.API`类型的参数，用于提供Api接口；
  - **输出**:一个新的rangcheck实例
  - **功能**:创建并返回一个新的rangecheck实例；
- **`rangecheck.Check(Vals frontend.Variable,bits int)`**
  - **输入**: 一个电路中的变量和一个整数变量
  - **输出**: 验证结果
  - **功能**: 检验电路中的变量是否是一个bits长的数;
- ** 


### 1.2 `frontend.Compile`（电路编译）

#### 功能：
`frontend.Compile` 将电路结构编译为 R1CS（Rank-1 Constraint System）表示，这是零知识证明协议的核心。R1CS 是一个用于描述电路约束的标准化格式，零知识证明系统使用它来验证电路中的关系。

#### 输入：
- **curve**：使用的曲线（例如 `curve.BN254`）。`gnark` 支持不同的椭圆曲线，您可以根据需求选择。
- **电路定义**：包含约束的电路结构。通常是实现了 `frontend.Circuit` 接口的结构体。这个结构体定义了电路中各个变量之间的关系和约束。

#### 输出：
- **R1CS**：编译后的电路表示。`R1CS` 是零知识证明中的电路表示形式，它定义了电路中所有的线性约束。

#### 使用方法：
调用 `frontend.Compile` 方法时，您需要提供两个参数：

1. **curve**：指定要使用的曲线（例如 `curve.BN254`）。
2. **电路定义**：一个实现了 `frontend.Circuit` 接口的结构体。

### 1.3 `frontend.NewWitness`（创建电路见证）

#### 功能：
`frontend.NewWitness` 用于创建电路的完整见证。见证是电路输入的赋值集合，包含私有输入和公共输入，供零知识证明协议使用。完整见证描述了电路所有变量的具体值。

#### 输入：
- **assignment**：电路输入赋值，可以是电路对应的结构体实例，包含所有输入的具体值。
- **curve**：使用的曲线（例如 `curve.BN254`）。`gnark` 支持多种椭圆曲线，用户需根据电路的约束选择相应的曲线。

#### 输出：
- **完整见证（witness）**：包含电路所有输入的值，包括公共和私有部分。
- **错误（error）**：如果见证创建失败，返回对应的错误信息。