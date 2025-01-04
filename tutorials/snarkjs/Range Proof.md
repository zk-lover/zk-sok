### Range Proof

Let's introduce the circuit computation logic of the code below. The circuit proves that `in[0]` is within the range [0, `in[1]`]. We want to prove that `x` is within the range [0, 2^32], so our input `in[1]` is 2^32:

* Define n<=32. You need to ensure that your input does not exceed 2^32.

* Our n is 32.

* Compute `in[0] + (1 << n) - in[1]`, which is `in[0] - in[1] + 2^32`. We can understand it this way: if `in[0]` is less than `in[1]`, then this expression is less than 2^32. Thus, `n2b.in`, which is the 33-bit binary of the above expression, has its 33rd bit as 0, making `out` equal to 1, indicating that `in[0]` is within the specified range.

* Conversely, if `in[0]` is greater than `in[1]`, then this expression is greater than 2^32. Thus, `n2b.in`, which is the 33-bit binary of the above expression, has its 33rd bit as 1, making `out` equal to 0, indicating that `in[0]` exceeds the specified range.

```markup
pragma circom 2.0.0;
include "bitify.circom";
template range_proof(n) {
    assert(n <= 32);
    signal input in[2];
    signal output out;
    component n2b = Num2Bits(n+1);
    n2b.in <== in[0]+ (1<<n) - in[1];
    out <== 1-n2b.out[n];
}
component main = range_proof(32);
```

