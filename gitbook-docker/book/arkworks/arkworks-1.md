# 1. 结构体定义：CubicPlusLinearCircuit
- 	定义一个电路，提供输入变量x,输出y通过约束系统计算得出。
# 2. generate_constraints
- 上述定义的结构体需要实现此方法;
- 输入：cs:引用类型 ConstraintSystemRef,表示约束系统的引用，用于注册变量和约束。
- 输出:	Ok(()) 表示成功生成约束;Err(SynthesisError) 表示约束生成过程中出错。
# 3. cs.new_witness_variable
- 功能:用于在约束系统中定义一个 见证变量（witness variable），表示一个由证明者提供的秘密输入值。
- 输入:一个闭包：定义该变量的返回逻辑，例如:|| self.x.ok_or(SynthesisError::AssignmentMissing):若 self.x 存在值，则返回  x ；否则返回 SynthesisError::AssignmentMissing 错误，表示变量未被赋值。
- 输出:	返回一个标识符，表示新定义的见证变量x,该变量将被注册到约束系统中，供后续约束使用。
# 4. cs.new_input_variable
- 功能:用于在约束系统中定义一个 公共输入变量（input variable），表示电路的公开输入值,公开输入变量会暴露给验证者，验证者可以直接验证其值是否符合约束。
- 输入:同cs.new_witness_variable的输入，一个闭包;
- 输出:	返回一个标识符，表示新定义的公共输入变量y,y 的值可由验证者直接提供，用于验证证明是否有效。
# 5. lc!()
- lc!() 是一个宏，用于在约束系统中创建线性组合,例如：lc!() + (2, x) + (3, y)表示2x+3y
# 6. cs.enforce_constraint
- 功能: 用于在约束系统中添加一个约束，定义两个线性组合的乘积等于另一个线性组合。
- 输入:三个线性组合a,b,c,表示a*b=c;
- 输出:Ok(()): 表示成功添加约束;Err(SynthesisError): 表示添加约束时出错;
