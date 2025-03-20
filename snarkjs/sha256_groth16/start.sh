snarkjs powersoftau new bn128 17 pot17_0000.ptau -v

snarkjs powersoftau contribute pot17_0000.ptau pot17_0001.ptau --name="First contribution" -v

snarkjs powersoftau contribute pot17_0001.ptau pot17_0002.ptau --name="Second contribution" -v -e="some random text"

snarkjs powersoftau export challenge pot17_0002.ptau challenge_0003
snarkjs powersoftau challenge contribute bn128 challenge_0003 response_0003 -e="some random text"
snarkjs powersoftau import response pot17_0002.ptau response_0003 pot17_0003.ptau -n="Third contribution name"

snarkjs powersoftau verify pot17_0003.ptau

snarkjs powersoftau beacon pot17_0003.ptau pot17_beacon.ptau 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon"

snarkjs powersoftau prepare phase2 pot17_beacon.ptau pot17_final.ptau -v

snarkjs powersoftau verify pot17_final.ptau

circom --r1cs --wasm --c --sym --inspect sha256_example.circom

snarkjs r1cs info sha256_example.r1cs

snarkjs r1cs print sha256_example.r1cs sha256_example.sym

snarkjs r1cs export json sha256_example.r1cs sha256_example.r1cs.json

snarkjs wtns calculate sha256_example_js/sha256_example.wasm input.json witness.wtns

snarkjs groth16 setup sha256_example.r1cs pot17_final.ptau circuit_0000.zkey

snarkjs zkey contribute circuit_0000.zkey circuit_0001.zkey --name="1st Contributor Name" -v

snarkjs zkey contribute circuit_0001.zkey circuit_0002.zkey --name="Second contribution Name" -v -e="Another random entropy"

snarkjs zkey export bellman circuit_0002.zkey  challenge_phase2_0003
snarkjs zkey bellman contribute bn128 challenge_phase2_0003 response_phase2_0003 -e="some random text"
snarkjs zkey import bellman circuit_0002.zkey response_phase2_0003 circuit_0003.zkey -n="Third contribution name"

snarkjs zkey verify sha256_example.r1cs pot17_final.ptau circuit_0003.zkey

snarkjs zkey beacon circuit_0003.zkey circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"

snarkjs zkey verify sha256_example.r1cs pot17_final.ptau circuit_final.zkey

snarkjs zkey export verificationkey circuit_final.zkey verification_key.json

time snarkjs groth16 prove circuit_final.zkey witness.wtns proof.json public.json

time snarkjs groth16 verify verification_key.json public.json proof.json
