pragma circom 2.0.0;

template cubic_expression() {
    // Declare input and output
    signal input x;
    signal output out;

    // Calculate x^3 + x + 5
    signal xx <== x*x;
    signal xxx <== xx*x;
    signal val1 <== xxx+x;
    signal val2 <== val1+5;

    // Connect calculation result to output
    out <== val2;
}

component main = cubic_expression();