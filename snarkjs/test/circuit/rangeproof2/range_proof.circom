pragma circom 2.0.0;

include "bitify.circom";

template range_proof(n) {
    assert(n <= 32);
    signal input x;
    signal input y;
    signal output out;

    component n2b = Num2Bits(n+1);

    n2b.in <== x+ (1<<n) - y;

    out <== 1-n2b.out[n];
}

// 主组件实例化
component main = range_proof(32);