# If you already have a final.patu file, you can just follow these steps
circom --r1cs --wasm --c --sym --inspect sha256_example.circom

snarkjs r1cs info sha256_example.r1cs

snarkjs r1cs print sha256_example.r1cs sha256_example.sym

# snarkjs r1cs export json sha256_example.r1cs sha256_example.r1cs.json

snarkjs wtns calculate sha256_example_js/sha256_example.wasm input.json witness.wtns

snarkjs fflonk setup circuit.r1cs pot22_final.ptau circuit.zkey

snarkjs zkey verify sha256_example.r1cs pot22_final.ptau circuit.zkey

snarkjs zkey export verificationkey circuit.zkey verification_key.json

time snarkjs plonk prove circuit.zkey witness.wtns proof.json public.json

time snarkjs plonk verify verification_key.json public.json proof.json