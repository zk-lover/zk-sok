# 1.init_public_params()
- 输入:无
- 输出:无
- 功能:初始化所需的公私参数，用于后续的 R1CS（Rank-1 Constraint System）约束生成。

# 1. Protoboard
- protooard用于构建基础的r1cs约束电路，具体用法如下:
## 1.protoboard<FieldT> pb
- libsnark中的基础结构——表示电路的面包板

## 2.pb_variable<FieldT> x
- libsnark中的基础结构——电路面包板上的基础变量

## 3. x.allocate(pb, "x")
- pb_variable变量的成员函数
- 输入:protoboard变量pb和代表名称的字符串;
- 输出:无
- 功能:将变量 x 添加到 protoboard 中，并为其分配空间，在此之后，变量 x 就可以在 protoboard 中使用，例如在约束添加过程中引用。

## 4. pb.set_input_sizes(int size)
- protoboard变量的成员函数
- 输入:int变量size
- 输出:无
- 功能:这里设置了公开输入的数量size,在面包板上申请变量首先申请公开的变量,面包板上的前size个变量为公开的，其余为秘密变量。

## 5. pb.add_r1cs_constraint(r1cs_constraint<FieldT>(x, x, sym_1))
- protoboard变量的成员函数
- 输入:三个pb_variable变量，表示乘法约束关系 x*x=sym_1
- 输出:无
- 功能:添加一个 R1CS 约束，表示 x * x = sym_1。

## 6. pb.val(x) = 3
- protoboard变量的成员函数
- 输入:int变量
- 输出:无
- 功能:将面包板上x的值设置为3.


## 7. pb.get_constraint_system()
- protoboard变量的成员函数
- 输入:无
- 输出:r1cs_constraint_system
- 功能:获取已创建的所有 R1CS 约束,用于后续证明生成;
# 2.Gadgets
- gadgets可以支持开发者将通用的r1cs约束进行打包,便于调用，对于调用者来说，使用gadegts时我们只需关注他的输入与输出，而无需关注内部的细节；对于开发者来说，要实现gadgets的一些接口，比如构造函数,`generate_r1cs_constraints()`,`generate_r1cs_witness()`;
## 1.构造函数:gadget(protoboard<FieldT> &pb, const std::string &annotation_prefix="")
- gadget必须绑定一个protoboard,annotation_prefix为可选参数，用来标识gadget;
- 可以通过C++中的继承方式来自定义gadget，根据需要进行不同的构造;
## 2. generate_r1cs_constraints()
- 用于添加电路上的r1cs约束，只是进行了封装;
- 要求开发者在函数内部实现对应的添加约束的逻辑;
## 3. generate_r1cs_witness()
- 该函数假设我们已经对电路的秘密变量和公共变量进行了设置；
- 用于生成在证明推导时所需要的中间变量值；
# 3. 常用gadgets
## 1. sha256_two_to_one_hash_gadget
- sha256_two_to_one_hash_gadget 是一个实现了 SHA256 压缩函数的组件，用作 2-to-1 哈希函数,支持两种构造函数:
- 构造函数输入1:protoboard<FieldT> &pb:约束系统的 protoboard;const digest_variable<FieldT> &left,const digest_variable<FieldT> &right:两个输入哈希值;const digest_variable<FieldT> &output:哈希输出;const std::string &annotation_prefix:gadget的标识信息;
- 构造函数输入2:protoboard<FieldT> &pb:约束系统的protoboard;const size_t block_length:输入块的长度;const block_variable<FieldT> &input_block:输入块的数据;const digest_variable<FieldT> &output:哈希输出;const std::string &annotation_prefix:gadget的标识信息;
## 2.merkle_tree_check_update_gadget
- merkle_tree_check_update_gadget 用于验证给定的两个默克尔树根（R1 和 R2）之间的更新关系：	
    1. 检查验证路径 P 是否是值 V1 作为地址 A 对应的叶节点，在以根为 R1 的默克尔树中的有效验证路径。
	2. 检查验证路径 P 是否是值 V2 作为地址 A 对应的叶节点，在以根为 R2 的默克尔树中的有效验证路径。
- 构造函数:
    1. protoboard<FieldT> &pb:用于构建 R1CS 的 protoboard
    2. const size_t tree_depth:默克尔树的深度
    3. const pb_variable_array<FieldT> &address_bits:叶节点的二进制地址
    4. const digest_variable<FieldT> &prev_leaf_digest 和const digest_variable<FieldT> &next_leaf_digest:更新前后的叶节点哈希值
    5. const digest_variable<FieldT> &prev_root_digest和const digest_variable<FieldT> &next_root_digest:更新前后的根哈希值
    6. const merkle_authentication_path_variable<FieldT, HashT> &prev_path和const merkle_authentication_path_variable<FieldT, HashT> &next_path:更新前后对应的验证路径,
    7. const pb_linear_combination<FieldT> &update_successful:布尔变量，指示更新是否成功
    8. const std::string &annotation_prefix:标识信息; 
## 3. G1_add_gadget
- G1_add_gadget 用于验证椭圆曲线 G1 群上的点加法操作是否正确(验证结果点C是否为点A和点B的加法结果:C = A + B);
- 构造函数:
    1.	protoboard &pb: 用于构建 R1CS 的 protoboard。
	2.	const G1_variable &A: 椭圆曲线上的第一个输入点  A 。
	3.	const G1_variable &B: 椭圆曲线上的第二个输入点  B 。
	4.	const G1_variable &C: 椭圆曲线加法结果点  C 。
	5.	const std::string &annotation_prefix: 标识信息;                       
                              
                              
                              