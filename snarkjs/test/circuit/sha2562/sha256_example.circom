pragma circom 2.0.0;

include "sha256.circom";  // 包含 sha256.circom 模板
include "bitify.circom";  // 包含 Num2Bits 模板

// 定义将整数转换为 512 位输入的组件
template IntToBits512() {
    // 输入：一个 256 位的整数
    signal input int;

    // 输出：512 位的二进制表示
    signal output bits[512];

    // 使用 Num2Bits 将整数转换为 256 位的二进制表示
    component n2b = Num2Bits(256);
    n2b.in <== int;

    // 填充 bits 数组，前 256 位是整数的二进制表示，后 256 位为 0
    for (var i = 0; i < 256; i++) {
        bits[i] <== n2b.out[i];  // 复制转换结果
    }
    for (var i = 256; i < 512; i++) {
        bits[i] <== 0;  // 填充为 0
    }
}

// 定义主电路
template Main() {
    // 输入：一个整数（假设为 256 位整数）
    signal input int;

    // 输出：哈希值是 256 位（32 字节）
    signal output hash[256];

    // 实例化 IntToBits512 和 SHA-256 模板
    component converter = IntToBits512();
    component hasher = Sha256(512);  // 使用 512 位的 SHA-256 模板

    // 连接整数输入
    converter.int <== int;

    // 将转换后的 512 位二进制连接到 hasher 的输入
    for (var i = 0; i < 512; i++) {
        hasher.in[i] <== converter.bits[i];
    }

    // 将输出连接到 hash
    for (var i = 0; i < 256; i++) {
        hash[i] <== hasher.out[i];
    }
}

// 使用 Main 作为主组件
component main = Main();
