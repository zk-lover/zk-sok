snarkjs provides various cryptographic primitives and circuit gadgets through the circomlib library, which can help developers quickly build zero-knowledge proof circuits. The circomlib library contains a large number of common cryptographic primitives, allowing developers to use them directly in their circuits without having to implement these algorithms themselves. Here are some cryptographic primitives provided by circomlib:

1.**Hash Functions**

- **sha256.circom**: Provides an implementation of the SHA256 hash function, widely used in cryptography for data integrity checks, authentication, digital signatures, and more. It is used in zero-knowledge proofs to generate irreversible hash values, protecting data privacy.
- **pedersen.circom**: An implementation of the Pedersen hash circuit, particularly suitable for hashing large data sets. The Pedersen hash is fundamental to commitment schemes in cryptography and has high efficiency and security in zk-SNARK and other proof systems. It is commonly used to build commitment-based zero-knowledge proofs.
- **mimc.circom**: A cryptographic hash function based on MIMC (Minimum Input Modulo Conference), especially suitable for zero-knowledge proof circuits. Its computational complexity is low, and the circuit size is relatively small, making it very suitable for constructing zk-SNARKs, especially in performance-critical scenarios.

2.**Signature Related**

- **eddsa.circom**: Provides an implementation of the EdDSA (Edwards-curve Digital Signature Algorithm) signature algorithm in circuit form. EdDSA is a widely used digital signature scheme in modern cryptographic protocols, offering higher security and performance advantages. This component can help developers verify and generate signatures in zero-knowledge proofs, commonly used in authentication and digital currency protocols.

3.**Binary Operations**

- **binsub.circom**: This circuit template is used to implement binary subtraction operations. With this component, developers can perform binary subtraction in zero-knowledge proof circuits while handling borrows.
 - **binsum.circom**: This circuit template is used to implement binary addition operations. During addition, the circuit will handle carries, which is a common requirement in building cryptographic protocols, especially when dealing with cryptocurrency transactions or smart contracts.
  - **comparators.circom**: By combining binsub.circom and binsum.circom, comparison operations (such as greater than, less than, equal to, etc.) can be implemented. This comparator circuit is commonly used to check whether data meets certain conditions, often found in the conditional validation parts of cryptographic protocols, such as range checks.
