snarkjs powersoftau new bn128 14 pot14_0000.ptau -v

snarkjs powersoftau contribute pot14_0000.ptau pot14_0001.ptau --name="First contribution" -v

snarkjs powersoftau contribute pot14_0001.ptau pot14_0002.ptau --name="Second contribution" -v -e="some random text"

snarkjs powersoftau export challenge pot14_0002.ptau challenge_0003
snarkjs powersoftau challenge contribute bn128 challenge_0003 response_0003 -e="some random text"
snarkjs powersoftau import response pot14_0002.ptau response_0003 pot14_0003.ptau -n="Third contribution name"

snarkjs powersoftau verify pot14_0003.ptau

snarkjs powersoftau beacon pot14_0003.ptau pot14_beacon.ptau 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon"

snarkjs powersoftau prepare phase2 pot14_beacon.ptau pot14_final.ptau -v

snarkjs powersoftau verify pot14_final.ptau

circom --r1cs --wasm --c --sym --inspect range_proof.circom

snarkjs r1cs info range_proof.r1cs

snarkjs r1cs print range_proof.r1cs range_proof.sym

snarkjs r1cs export json range_proof.r1cs range_proof.r1cs.json

snarkjs wtns calculate range_proof_js/range_proof.wasm input.json witness.wtns

snarkjs fflonk setup range_proof.r1cs pot14_final.ptau circuit.zkey

snarkjs zkey export verificationkey circuit.zkey verification_key.json

time snarkjs fflonk prove circuit.zkey witness.wtns proof.json public.json

time snarkjs fflonk verify verification_key.json public.json proof.json
