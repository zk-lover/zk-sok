## Sample programs using the snarkjs library

We have implemented the three example programs mentioned in our paper based on snarkjs, including:

* A Cubic Expression

* Range Proof

* SHA256

Snarkjs uses the Groth16 Protocol, PLONK, and FFLONK. Therefore, we implemented our example programs using these three schemes. The differences between these schemes can be found in our paper. The program source code is located in the `snarkjs/test/circuit` directory under nine subdirectories (for three different ZKPs) with the same name circom files: `cubic_expression`, `range_proof`, and `sha256`.

For the "A Cubic Expression" example program, the source code based on the Groth16 scheme is located at:

> snarkjs\test\circuit\cubic_expression.circom

The same applies to the other example programs.

For the snarkjs library, the example programs we wrote are circom circuits, and then ZKPs are generated through commands. Therefore, this tutorial will first annotate our example program code and then introduce the process from the example program's circuit to ZKP to demonstrate the use of snarkjs.

You can find the design logic of the circuit in each markdown file in the directory. Below, we will introduce how to run it.

### Run the Example Programs

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

```
circom cubic_expression.circom --r1cs --wasm --sym
```

This will generate `.r1cs`, `.wasm`, and `.sym` files.

##### Generate Powers of Tau

Purpose: Generate public parameters for the trusted setup of zk-SNARK.

```
snarkjs powersoftau new bn128 12 pot12_0000.ptau
snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution"
```

This will generate the initial and contributed `.ptau` files.

##### Prepare Phase 2 Powers of Tau

Purpose: Prepare the `.ptau` file for Groth16.

```
snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau
```

This will generate the prepared Phase 2 file.

##### Generate zk-SNARK Keys

Purpose: Generate the proving and verifying keys.

```
snarkjs groth16 setup cubic_expression.r1cs pot12_final.ptau cubic_expression_0000.zkey
snarkjs zkey contribute cubic_expression_0000.zkey cubic_expression_final.zkey --name="Second contribution"
snarkjs zkey export verificationkey cubic_expression_final.zkey verification_key.json
```

This will generate the initial and final zk-SNARK keys, as well as the verification key.

##### Generate the Proof

Purpose: Generate the proof using the input.

```
node cubic_expression_js/generate_witness.js cubic_expression_js/cubic_expression.wasm input.json witness.wtns
snarkjs groth16 prove cubic_expression_final.zkey witness.wtns proof.json public.json
```

This will generate the witness file, proof, and public inputs.

##### Verify the Proof

```
snarkjs groth16 verify verification_key.json public.json proof.json
```

#### A Cubic Expression Implemented Based on Plonk

##### Generate the Universal Powers of Tau

```
snarkjs powersoftau new bn128 12 pot12_0000.ptau
snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution"
snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau
```

##### Generate zk-SNARK Keys

```
snarkjs plonk setup cubic_expression.r1cs pot12_final.ptau cubic_expression_final.zkey
```

##### Generate the Proof

```
node cubic_expression_js/generate_witness.js cubic_expression_js/cubic_expression.wasm input.json witness.wtns
snarkjs plonk prove cubic_expression_final.zkey witness.wtns proof.json public.json
```

##### Verify the Proof

```
snarkjs plonk verify verification_key.json public.json proof.json
```

#### The steps for using FFLONK are similar to Plonk, but require FFLONK-specific commands and libraries.

### ZKP for Your Own Circuits

You can refer to the steps above or consult [snarkjs](https://github.com/iden3/snarkjs).



