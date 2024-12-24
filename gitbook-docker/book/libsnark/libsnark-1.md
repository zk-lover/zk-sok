# 1.init_public_params()
- 输入:无
- 输出:无
- 功能:初始化所需的公私参数，用于后续的 R1CS（Rank-1 Constraint System）约束生成。

# 2.protoboard<FieldT> pb
- libsnark中的基础结构——表示电路的面包板

# 3.pb_variable<FieldT> x
- libsnark中的基础结构——电路面包板上的基础变量

# 4. x.allocate(pb, "x")
- pb_variable变量的成员函数
- 输入:protoboard变量pb和代表名称的字符串;
- 输出:无
- 功能:将变量 x 添加到 protoboard 中，并为其分配空间，在此之后，变量 x 就可以在 protoboard 中使用，例如在约束添加过程中引用。

# 5. pb.set_input_sizes(int size)
- protoboard变量的成员函数
- 输入:int变量size
- 输出:无
- 功能:这里设置了公开输入的数量size,在面包板上申请变量首先申请公开的变量,面包板上的前size个变量为公开的，其余为秘密变量。

# 6. pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x, x, sym_1))
- protoboard变量的成员函数
- 输入:三个pb_variable变量，表示乘法约束关系 x*x=sym_1
- 输出:无
- 功能:添加一个 R1CS 约束，表示 x * x = sym_1。

# 7. pb.val(x) = 3
- protoboard变量的成员函数
- 输入:int变量
- 输出:无
- 功能:将面包板上x的值设置为3.

# 8. pb.get_constraint_system()
- protoboard变量的成员函数
- 输入:无
- 输出:r1cs_constraint_system
- 功能:获取已创建的所有 R1CS 约束,用于后续证明生成;