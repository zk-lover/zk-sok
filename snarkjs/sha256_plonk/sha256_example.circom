pragma circom 2.0.0;

include "/app/node_modules/circomlib/circuits/sha256.circom";  // Include sha256.circom template

// Define main circuit
template Main() {
    // Input: message is 512 bits (64 bytes) input
    signal input in[512];  // SHA-256 input should be 512 bits

    // Output: hash value is 256 bits (32 bytes)
    signal output hash[256];

    // Instantiate SHA-256 template
    component hasher = Sha256(512);  // Use 512-bit SHA-256 template

    // Connect input to hasher's input
    for (var i = 0; i < 512; i++) {
        hasher.in[i] <== in[i];
    }

    // Connect output to hash
    for (var i = 0; i < 256; i++) {
        hash[i] <== hasher.out[i];
    }
}

// Use Main as the main component
component main = Main();
