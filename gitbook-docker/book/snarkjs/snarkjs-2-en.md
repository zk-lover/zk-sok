# 1.Generating Public and Private Parameters
Zero-knowledge proofs require a trusted initialization step, often called a trusted setup. Using snarkjs, you can generate the public and private parameters for a trusted setup.

First, use **snarkjs** to generate the necessary parameters:
```
snarkjs plonk setup circuit.r1cs pot14_final.ptau circuit_final.zkey
```
This command takes the **r1cs** constraints generated from compiling the circom file as input, and generates the ptau public parameter file and the circuit_final key file.

# 2. Computing the Witness
Before creating any proof, we need to compute all circuit signals that match the (all) circuit constraints. These signals are called the witness.

First, create an ```input.json``` file:
```
{
  "x": 3
}
```
Then run the command to compute the witness:
```
snarkjs wtns calculate circuit.wasm input.json witness.wtns
```
This command takes the **wasm** file generated from compiling the circom file and the input.json file as input, and generates the **witness.wtns** file.

# 3. Verify the final ```zkey```
```
snarkjs zkey verify circuit.r1cs pot14_final.ptau circuit_final.zkey
```
This command takes the ```circuit.r1cs``` constraint file, ```pot14_final.ptau``` public parameter, and ```circuit_final.zkey``` key file as input and verifies the correctness of the key.
# 4. Exporting the Verification Key
```
snarkjs zkey export verificationkey circuit_final.zkey verification_key.json
``` 
After verifying the key, this command converts ```circuit_final.zkey``` into the ```verification_key.json``` file in JSON format.
# 5. Generating the Proof
```
snarkjs plonk prove circuit_final.zkey witness.wtns proof.json public.json
```
This command takes the ```circuit_final.zkey``` key and ```witness.wtns``` witness as input and generates the ```proof.json``` and ```public.json``` files.
Here, the ```proof.json``` file is the actual proof, while the ```public.json``` file contains the public inputs and outputs.

# 6. Verifying the Proof
```
snarkjs plonk verify verification_key.json public.json proof.json
```
After receiving the ```verification_key.json```, ```public.json```, and ```proof.json``` files, this command outputs the verification result of the proof.