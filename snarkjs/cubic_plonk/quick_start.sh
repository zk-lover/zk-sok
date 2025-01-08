# If you already have a final.patu file, you can just follow these steps
circom --r1cs --wasm --c --sym --inspect cubic_expression.circom

snarkjs r1cs info cubic_expression.r1cs

snarkjs r1cs print cubic_expression.r1cs cubic_expression.sym

# snarkjs r1cs export json cubic_expression.r1cs cubic_expression.r1cs.json

snarkjs wtns calculate cubic_expression_js/cubic_expression.wasm input.json witness.wtns

snarkjs plonk setup circuit.r1cs pot14_final.ptau circuit_final.zkey

snarkjs zkey verify cubic_expression.r1cs pot14_final.ptau circuit_final.zkey

snarkjs zkey export verificationkey circuit_final.zkey verification_key.json

time snarkjs plonk prove circuit_final.zkey witness.wtns proof.json public.json

time snarkjs plonk verify verification_key.json public.json proof.json