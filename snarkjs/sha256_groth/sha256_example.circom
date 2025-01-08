pragma circom 2.0.0;

include "sha256.circom";  // 包含 sha256.circom 模板

// 定义主电路
template Main() {
    // 输入：消息是一个长度为 512 位（64 字节）的输入
    signal input in[512];  // SHA-256 的输入应为 512 位

    // 输出：哈希值是 256 位（32 字节）
    signal output hash[256];

    // 实例化 SHA-256 模板
    component hasher = Sha256(512);  // 使用 512 位的 SHA-256 模板

    // 将输入连接到 hasher 的输入
    for (var i = 0; i < 512; i++) {
        hasher.in[i] <== in[i];
    }

    // 将输出连接到 hash
    for (var i = 0; i < 256; i++) {
        hash[i] <== hasher.out[i];
    }
}

// 使用 Main 作为主组件
component main = Main();
