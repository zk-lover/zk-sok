### A Cubic Expression

Circom code is written with the following logic:

* Declare inputs and outputs

* Compute x^3 + x + 5

* Connect the computation result to the output

* Instantiate the main component

```markup
pragma circom 2.0.0;

template cubic_expression() {
    signal input x;
    signal output out;
    signal xx <== x*x;
    signal xxx <== xx*x;
    signal val1 <== xxx+x;
    signal val2 <== val1+5; // x^3 + x + 5
    out <== val2;
}
component main = cubic_expression();
```

