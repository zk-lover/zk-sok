snarkjs powersoftau new bn128 20 pot20_0000.ptau -v

snarkjs powersoftau contribute pot20_0000.ptau pot20_0001.ptau --name="First contribution" -v

snarkjs powersoftau contribute pot20_0001.ptau pot20_0002.ptau --name="Second contribution" -v -e="some random text"

snarkjs powersoftau export challenge pot20_0002.ptau challenge_0003
snarkjs powersoftau challenge contribute bn128 challenge_0003 response_0003 -e="some random text"
snarkjs powersoftau import response pot20_0002.ptau response_0003 pot20_0003.ptau -n="Third contribution name"

snarkjs powersoftau verify pot20_0003.ptau

snarkjs powersoftau beacon pot20_0003.ptau pot20_beacon.ptau 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon"

snarkjs powersoftau prepare phase2 pot20_beacon.ptau pot20_final.ptau -v

snarkjs powersoftau verify pot20_final.ptau

circom --r1cs --wasm --c --sym --inspect sha256_example.circom

snarkjs r1cs info sha256_example.r1cs

snarkjs r1cs print sha256_example.r1cs sha256_example.sym

snarkjs r1cs export json sha256_example.r1cs sha256_example.r1cs.json

snarkjs wtns calculate sha256_example_js/sha256_example.wasm input.json witness.wtns

snarkjs plonk setup circuit.r1cs pot20_final.ptau circuit_final.zkey

snarkjs zkey verify range_proof.r1cs pot20_final.ptau circuit_final.zkey

snarkjs zkey export verificationkey circuit_final.zkey verification_key.json

time snarkjs plonk prove circuit_final.zkey witness.wtns proof.json public.json

time snarkjs plonk verify verification_key.json public.json proof.json
