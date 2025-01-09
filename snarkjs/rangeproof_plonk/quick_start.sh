# If you already have a final.patu file, you can just follow these steps
circom --r1cs --wasm --c --sym --inspect range_proof.circom

snarkjs r1cs info range_proof.r1cs

snarkjs r1cs print range_proof.r1cs range_proof.sym

# snarkjs r1cs export json range_proof.r1cs range_proof.r1cs.json

snarkjs wtns calculate range_proof_js/range_proof.wasm input.json witness.wtns

snarkjs plonk setup range_proof.r1cs pot14_final.ptau circuit_final.zkey

snarkjs zkey export verificationkey circuit_final.zkey verification_key.json

time snarkjs plonk prove circuit_final.zkey witness.wtns proof.json public.json

time snarkjs plonk verify verification_key.json public.json proof.json