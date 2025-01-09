snarkjs powersoftau new bn128 22 pot22_0000.ptau -v

snarkjs powersoftau contribute pot22_0000.ptau pot22_0001.ptau --name="First contribution" -v

snarkjs powersoftau contribute pot22_0001.ptau pot22_0002.ptau --name="Second contribution" -v -e="some random text"

snarkjs powersoftau export challenge pot22_0002.ptau challenge_0003
snarkjs powersoftau challenge contribute bn128 challenge_0003 response_0003 -e="some random text"
snarkjs powersoftau import response pot22_0002.ptau response_0003 pot22_0003.ptau -n="Third contribution name"

snarkjs powersoftau verify pot22_0003.ptau

snarkjs powersoftau beacon pot22_0003.ptau pot22_beacon.ptau 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon"

snarkjs powersoftau prepare phase2 pot22_beacon.ptau pot22_final.ptau -v

snarkjs powersoftau verify pot22_final.ptau

circom --r1cs --wasm --c --sym --inspect sha256_example.circom

snarkjs r1cs info sha256_example.r1cs

snarkjs r1cs print sha256_example.r1cs sha256_example.sym

snarkjs r1cs export json sha256_example.r1cs sha256_example.r1cs.json

snarkjs wtns calculate sha256_example_js/sha256_example.wasm input.json witness.wtns

snarkjs fflonk setup sha256_example.r1cs pot22_final.ptau circuit.zkey

snarkjs zkey export verificationkey circuit.zkey verification_key.json

time snarkjs fflonk prove circuit.zkey witness.wtns proof.json public.json

time snarkjs fflonk verify verification_key.json public.json proof.json
