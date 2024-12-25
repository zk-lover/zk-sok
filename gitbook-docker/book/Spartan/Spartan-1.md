# 1. let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
- R1CS 使用稀疏矩阵A,B,C来表示约束。每行对应一个约束，每列对应一个变量、输入或常量1。
- 该代码用来定义稀疏矩阵;
# 2. A.push((0, 0, one));
- 功能:稀疏矩阵的成员函数,用来设置矩阵中某个变量的值;
- 输入:	约束编号（行号）;变量编号（列号）;对应系数值（以字节数组形式表示）。
# 3. Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();
- 功能:使用 Instance::new 构造 R1CS 实例;
- 输入:	
    1. num_cons: 约束数量。
	2. num_vars: 变量数量。
	3. num_inputs: 输入数量。
	4. &A, &B, &C: 三个稀疏矩阵。
- 输出: 一个r1cs实例;
# 4.VarsAssignment::new(&vars)
- 功能:使用分配的变量向量创建一个 VarsAssignment 对象，表示所有变量的赋值
- 输入:&vars: 引用一个变量值的向量，类型为&Vec<[u8; 32]>,每个元素对应一个变量的值，通常使用 Curve25519 标量 (Scalar) 的字节表示来存储。
- 输出:一个 VarsAssignment 实例，表示所有变量的有效赋值。
# 5.InputsAssignment::new(&inputs).unwrap()
- 功能:用于创建一个输入赋值 (InputsAssignment) 实例，表示 R1CS 系统中所有公共输入的具体值。
- 输入:	&inputs: 引用一个变量值的向量，类型为&Vec<[u8; 32]>,每个元素对应一个变量的值，通常使用 Curve25519 标量 (Scalar) 的字节表示来存储。
- 输出:一个 InputsAssignment 实例，表示所有公共输入的有效赋值。
# 4. inst.is_sat(&assignment_vars, &assignment_inputs)
- 功能:inst的成员函数,验证 R1CS 系统的可满足性
- 输入:	
    1. assignment_vars：分配的变量值。
	2. assignment_inputs：分配的公共输入值。
- 输出:一个bool值，表示该R1CS系统是否可满足;
