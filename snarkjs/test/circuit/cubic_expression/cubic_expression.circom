pragma circom 2.0.0;

template cubic_expression() {
    // 声明输入和输出
    signal input x;
    signal output out;

    // 计算x^3 + x + 5
    signal xx <== x*x;
    signal xxx <== xx*x;
    signal val1 <== xxx+x;
    signal val2 <== val1+5; // x^3 + x + 5

    // 将计算结果连接到输出
    out <== val2;
}

// 主组件实例化
component main = cubic_expression();