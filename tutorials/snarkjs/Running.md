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

### ZKP for Your Own Circuits

You can refer to the steps above or consult [snarkjs](https://github.com/iden3/snarkjs).
