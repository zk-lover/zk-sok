### Sample programs using the snarkjs library

We have implemented the three example programs mentioned in our paper based on snarkjs, including:

* A Cubic Expression

* Range Proof

* Hash Function

Snarkjs uses the Groth16 Protocol, PLONK, and FFLONK. Therefore, we implemented our example programs using these three schemes. The differences between these schemes can be found in our paper. The program source code is located in the `snarkjs/test/circuit` directory under nine subdirectories (for three different ZKPs) with the same name circom files: `cubic_expression`, `range_proof`, and `sha256`.

For the "A Cubic Expression" example program, the source code based on the Groth16 scheme is located at:

> snarkjs\test\circuit\cubic_expression.circom

The same applies to the other example programs.

For the snarkjs library, the example programs we wrote are circom circuits, and then ZKPs are generated through commands. Therefore, this tutorial will first annotate our example program code and then introduce the process from the example program's circuit to ZKP to demonstrate the use of snarkjs.

### 1.Annotations of the example programs

Next, we will annotate the code of our example programs. For the snarkjs library, the example programs for the three methods are the same. The example programs are circuits written in circom, and using snarkjs for zero-knowledge proofs is achieved through commands based on the written circuits and input files, which will be introduced later.

#### A Cubic Expression

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

#### Range Proof

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

#### Hash Function

Circom code is written with the following logic:

* Input: The message is an input of length 512 bits (64 bytes).

* Output: The hash value is 256 bits (32 bytes).

* Instantiate the SHA-256 template.

* Connect the input to the hasher's input.

* Connect the output to the hash.

* Instantiate the main component.

```markup
pragma circom 2.0.0;
include "sha256.circom";  
template Main() {
    signal input in[512]; 
    signal output hash[256];
    component hasher = Sha256(512); 
    for (var i = 0; i < 512; i++) {
        hasher.in[i] <== in[i];
    }
    for (var i = 0; i < 256; i++) {
        hash[i] <== hasher.out[i];
    }
}
component main = Main();
```

### 2.Run the Example Programs

We recommend running our example programs in an Ubuntu virtual machine. Here, we provide a tutorial.

First, you need to install the latest version of Node.js using the following commands. If the final command displays the Node.js version, the installation was successful.

```
sudo apt update
sudo apt upgrade
curl -fsSL https://deb.nodesource.com/setup_current.x | sudo -E bash -
sudo apt install -y nodejs
node -v
```

Next, install snarkjs and circom.

```
sudo npm install -g snarkjs@latest
sudo npm install -g circom
```

Below, we use "A Cubic Expression" implemented with Groth16 and "A Cubic Expression" based on Plonk as examples to introduce how to use snarkjs to generate ZKP through our circom circuits. Fflonk is similar to Plonk.

#### A Cubic Expression Implemented Based on Groth16

##### Compile the Circuit

```markup
circom cubic_expression.circom --r1cs --wasm --sym
```

This will generate `.r1cs`, `.wasm`, and `.sym` files.

##### Generate Powers of Tau

Purpose: Generate public parameters for the trusted setup of zk-SNARK.

```markup
snarkjs powersoftau new bn128 12 pot12_0000.ptau
snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution"
```

This will generate the initial and contributed `.ptau` files.

##### Prepare Phase 2 Powers of Tau

Purpose: Prepare the `.ptau` file for Groth16.

```markup
snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau
```

This will generate the prepared Phase 2 file.

##### Generate zk-SNARK Keys

Purpose: Generate the proving and verifying keys.

```markup
snarkjs groth16 setup cubic_expression.r1cs pot12_final.ptau cubic_expression_0000.zkey
snarkjs zkey contribute cubic_expression_0000.zkey cubic_expression_final.zkey --name="Second contribution"
snarkjs zkey export verificationkey cubic_expression_final.zkey verification_key.json
```

This will generate the initial and final zk-SNARK keys, as well as the verification key.

##### Generate the Proof

Purpose: Generate the proof using the input.

```markup
node cubic_expression_js/generate_witness.js cubic_expression_js/cubic_expression.wasm input.json witness.wtns
snarkjs groth16 prove cubic_expression_final.zkey witness.wtns proof.json public.json
```

This will generate the witness file, proof, and public inputs.

##### Verify the Proof

```markup
snarkjs groth16 verify verification_key.json public.json proof.json
```

#### A Cubic Expression Implemented Based on Plonk

##### Generate the Universal Powers of Tau

```markup
snarkjs powersoftau new bn128 12 pot12_0000.ptau
snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution"
snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau
```

##### Generate zk-SNARK Keys

```markup
snarkjs plonk setup cubic_expression.r1cs pot12_final.ptau cubic_expression_final.zkey
```

##### Generate the Proof

```markup
node cubic_expression_js/generate_witness.js cubic_expression_js/cubic_expression.wasm input.json witness.wtns
snarkjs plonk prove cubic_expression_final.zkey witness.wtns proof.json public.json
```

##### Verify the Proof

```markup
snarkjs plonk verify verification_key.json public.json proof.json
```

#### The steps for using FFLONK are similar to Plonk, but require FFLONK-specific commands and libraries.

### 3. ZKP for Your Own Circuits

You can refer to the steps above or consult [snarkjs](https://github.com/iden3/snarkjs).
