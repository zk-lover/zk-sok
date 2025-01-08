# If you already have a final.patu file, you can just follow these steps
circom --r1cs --wasm --c --sym --inspect sha256_example.circom

snarkjs r1cs info sha256_example.r1cs

snarkjs r1cs print sha256_example.r1cs sha256_example.sym

# snarkjs r1cs export json sha256_example.r1cs sha256_example.r1cs.json

snarkjs wtns calculate sha256_example_js/sha256_example.wasm input.json witness.wtns

snarkjs groth16 setup sha256_example.r1cs pot14_final.ptau circuit_0000.zkey

snarkjs zkey contribute circuit_0000.zkey circuit_0001.zkey --name="1st Contributor Name" -v

snarkjs zkey contribute circuit_0001.zkey circuit_0002.zkey --name="Second contribution Name" -v -e="Another random entropy"

snarkjs zkey export bellman circuit_0002.zkey  challenge_phase2_0003
snarkjs zkey bellman contribute bn128 challenge_phase2_0003 response_phase2_0003 -e="some random text"
snarkjs zkey import bellman circuit_0002.zkey response_phase2_0003 circuit_0003.zkey -n="Third contribution name"

snarkjs zkey verify sha256_example.r1cs pot14_final.ptau circuit_0003.zkey

snarkjs zkey beacon circuit_0003.zkey circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"

snarkjs zkey verify sha256_example.r1cs pot14_final.ptau circuit_final.zkey

snarkjs zkey export verificationkey circuit_final.zkey verification_key.json

time snarkjs groth16 prove circuit_final.zkey witness.wtns proof.json public.json

time snarkjs groth16 verify verification_key.json public.json proof.json
