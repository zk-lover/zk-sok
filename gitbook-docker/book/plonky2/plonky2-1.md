# 1. CircuitConfig::standard_recursion_zk_config()
- 功能:生成一个标准的电路配置，适用于零知识证明,包含零知识证明电路的基本参数设置，例如安全性、深度等。
- 无显示输入;
- 输出:类型为CircuitConfig的标准电路配置。
# 2.CircuitBuilder::new(config)
- 功能: 使用提供的电路配置 config 创建一个新的电路构建器，用于定义电路中的操作和约束。
- 输入:类型为CircuitConfig的config,表示电路的基本配置。
- 输出:	类型为CircuitBuilder<F, D>的一个电路构建器实例，允许添加操作、约束和变量。
# 3.builder.add_virtual_target()
- 功能:	创建一个虚拟变量，表示电路中的一个未知值,该变量的具体值将在证明阶段通过见证（witness）赋值。
- 无显示输入;
- 输出:类型为Target的一个表示未知变量的目标标识符。
# 4.builder.mul(x, y)
- 功能:定义一个乘法约束，表示  $x \cdot y$ 。
- 输入:类型为Target的x,y,表示电路中的两个变量;
- 输出:	类型为Target的一个新的目标标识符，表示乘法结果;
# 5.builder.add(x, y)
- 功能:定义一个加法约束，表示  $x + y$ 。
- 输入:类型为Target的x,y,表示电路中的两个变量;
- 输出:	类型为Target的一个新的目标标识符，表示加法结果;
# 6.builder.add_const(d, F::from_canonical_u32(1))
- 功能:在电路中,变量d增加一个常量。
- 输入:类型为Target的d，表示电路中的一个变量;一个常量值1;
- 输出:类型为Target的一个新的目标标识符，表示结果d+1；
# 7.builder.register_public_input(x)
- 功能:注册一个公共输入变量，表示电路的公开输入,公开输入将暴露给验证者，用于验证证明。
- 输入:类型为Target的x,表示需要注册为公共输入的变量;
- 无显式输出;
# 8.PartialWitness::new()
- 功能:创建一个新的见证对象，用于为电路中的变量赋值。
- 无显式输入。
- 输出:类型为PartialWitness的一个新的见证对象，供后续设置变量值。
# 9.pw.set_target(x, F::from_canonical_u32(3))
- 功能:为目标变量x设置具体值（此处为  3 ）。
- 输入:类型为Target的变量x，表示需要赋值的目标变量;一个有限域元素,表示要设置给x的具体的值;
- 无显示输出;
# 10.builder.range_check
- 功能:用于在电路中添加一个范围检查约束，确保某个值在指定的范围内。
- 输入:
	1. value:类型为Target，需要进行范围检查的目标变量。
	2. log_max:类型为usize,定义允许的最大值的对数范围。
- 无显示输出,隐式修改电路构造器。
# 11.builder.build::<C>()
- 功能:根据电路的操作和约束，构建一个电路数据对象（CircuitData）。
- 输入:无显式输入，隐式使用之前定义的电路操作和约束。
- 输出:类型为CircuitData<C>,表示返回电路的数据结构，用于证明和验证。