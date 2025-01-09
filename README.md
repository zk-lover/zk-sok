# ZK-SoK

This repository involves many libraries related to zkSNARK. We utilized each library and implemented three sample programs with them. To facilitate user reproduction, we configured the corresponding environment in Docker and wrote usage instructions.

## Directory Structure

Each corresponding directory contains:

- The source code of three sample programs
- A `.dockerfile` file
- A `README` file that briefly introduces the library and guides users on how to enable Docker, run the sample programs, and build their own applications in detail.
  

## Project Goal

The goal of this project is to narrow the gap between academia and industry in the field of zkSNARK and assist relevant individuals in choosing suitable libraries and ZKP schemes to build their own applications according to their needs.

## Main Contributions

This repository has three main contributions:

1.**Sample Code**: We provide sample code that allows users to conduct preliminary testing of these zkSNARK libraries.
2.**Tutorial**: A detailed tutorial explaining the logical construction in the sample programs.
3.**Extended Documentation (Wiki Book)**: This document explains how to construct the circuit for each library's frontend, how to use the ZKP scheme to prove in the backend, and the related APIs involved in the gadgets provided by the library. Additionally, it explains the curves supported by each library.**Wiki Book**: [zk-sok](#)

## Paper

The paper  [zk-sok](#) attached to this repository systematically explores zkSNARK in theory and tests it experimentally.

## Sample Programs

We constructed three sample programs for each library, including:

- **cubic_expression** (single-digit circuit constraint)；
- **range_proof** (two-digit circuit constraint)；
- **sha256** (ten thousand-level circuit constraint)

These are classic scenarios using zero-knowledge proofs. We tested these three sample programs to understand the architecture, usage, and related functions of these zkSNARK libraries.

### Sample Program Descriptions

1. **cubic_expression**:

- This program demonstrates a cubic arithmetic polynomial, proving \(x^3 + x + 5 = 35\). It is a classic example used by many libraries, papers, and blogs. By constructing a zero-knowledge proof for cubic_expression, users can see the basic usage of ZKP.

2. **range_proof**:
   
- This program proves that a number \(x\) is within a specified range. In our example, it proves that \(x\) is between 0 and 2^32. This scenario is practical for zero-knowledge proofs, such as the range proof of transaction amounts on the blockchain. The usual practice involves decomposing \(x\) into binary bits, determining whether each bit is 0 or 1, and ensuring the reconstructed value equals the original value.
3. **sha256**:
   
- This program implements the SHA-256 hash, a common cryptographic primitive widely used in various scenarios. Due to its complex construction and calculation process, using zero-knowledge proof requires tens of thousands of circuit constraints. This example allows users to assess the zkSNARK library's ability to implement complex circuits.

## Curve

### Common elliptic curves
1.**Edwards curve**

- *Basic information*: Edwards curve is a curve widely used in efficient encryption protocols, especially for building encryption algorithms such as EdDSA and elliptic curve Diffie-Hellman (ECDH). Edwards curves have additive and bilinear properties, which can efficiently support curve-based encryption protocols. They provide up to 80 bits of security and have good efficiency in addition operations.  
- *Application*: Commonly used in systems that require lower computing resources, such as low-power devices and embedded systems.

2.**BN128 curve**  
- *Basic information*: BN128 curve is a pairing-friendly curve, which is widely used to build efficient zero-knowledge proof systems. It supports efficient ate pairing operations and usually provides 128 bits of security. Due to its good mathematical properties, BN128 has become one of the mainstream curves for building protocols such as Groth16 and Plonk.  
- *Application*: BN128 curve is the preferred curve in many pairing-based ZKP libraries (such as libsnark and snarkjs), suitable for application scenarios that require zero-knowledge proofs such as cryptocurrencies.  
  
3.**BLS12-381 curve**

- *Basic information*: BLS12-381 curve is a bilinear elliptic curve specially designed for BLS signature and pairing algorithms. It is one of the widely used curves in modern zero-knowledge proof protocols (such as Plonk and Groth16), providing 128-bit security. BLS12-381 supports efficient pairing operations and is suitable for building efficient and scalable zero-knowledge proof systems.  
- *Application*: Widely used in blockchain, cryptocurrency and other scenarios that require zero-knowledge proofs, such as zk-SNARKs and zk-STARKs.
  
4.**MNT curve (MNT4 and MNT6)**

- *Basic information*: MNT4 and MNT6 curves are pairing-friendly curves, especially suitable for building efficient pairing operations. They provide 298 bits and 753 bits of security respectively, and are suitable for cryptographic protocols that require large-scale data processing.  
- *Application*: MNT curves are often used in scenarios with higher security requirements, especially in some protocols that require multiple levels of encryption.
  
5.**Ristretto255 and Curve25519**

- *Basic information*: Curve25519 is an efficient elliptic curve designed for high security and low computational overhead. Ristretto255 is an acceleration protocol on Curve25519, providing a more secure and efficient way to operate elliptic curves.  
- *Application*: In efficient and fast cryptographic protocols (such as Spartan), Ristretto255 is widely used for fast encryption and signing operations.
  
6.**Pasta curve**

- *Basic information*: Pasta curves are a series of efficient and pairing-friendly curves proposed by Arkworks. Pallas and Vesta form a pair of friendly curves in the Pasta curve series, supporting efficient zero-knowledge proofs and cryptographic protocols.  
- *Application*: Pasta curve is suitable for application scenarios that require low latency and high throughput, especially in blockchain applications involving large-scale data verification.

### Curves supported in libraries
1.**libsnark**
- Supported curves:

  edwards: Based on Edwards curve, provides 80-bit security.

  bn128: Based on Barreto-Naehrig curve, provides 128-bit security and supports ate-pairing.

  alt_bn128: Alternative to BN128, avoiding dynamic code generation and suitable for different hardware architectures.

  MNT4 and MNT6: Pairing-friendly curves for high security requirements.
- Usage: When building, you can specify the curve to use through CMake. For example:
```
cmake -DCURVE=BN128 ..
```
This will choose to use the BN128 curve to build the libsnark library.  
2.**arkworks**
- Supported curves:

  BLS12-381: bilinear elliptic curve, suitable for efficient pairing operations.

  BLS12-377 and BLS12-377 derivatives: for higher security requirements.

  BN254: another common pairing-friendly curve.

  MNT4, MNT6: for large-scale cryptographic operations.

  Pasta (Pallas and Vesta): efficient curve pairs, supporting large-scale applications.
- Usage: When using arkworks, you can select the corresponding curve in the code, such as:
```
use ark_bls12_381::Bls12_381;
```
3.**gnark**
- Supported curves:

  Common curves such as bn254, bls12-381, bls12-377, bw6-761, etc.

  twisted Edwards subpackage: used to efficiently perform elliptic curve cryptographic operations in ZK-SNARK circuits.
- Usage: In gnark, you can select the corresponding curve type, for example:
```
curve := gnark.NewBls12_381()
```
4.**snarkjs**
- Supported curves:

  bn128 and bls12-381.
- Usage: When using snarkjs, you can specify the curve type during initialization, such as:
```
const curve = new snarkjs.bls12_381();
```
5.**libiop**
- Supported curves:

  libiop does not explicitly list the supported elliptic curves, but it focuses on providing input and output protocols for zero-knowledge proofs and relies on other libraries to implement elliptic curve operations. 

6.**Spartan**
- Supported curves:

  Curve25519 and Ristretto255 curves, which provide efficient elliptic curve operations.
- Usage: In Spartan, curve operations are usually implemented through curve25519-dalek.

7.**halo2**
- Supported curves:

  The current version mainly supports Pallas curves, and will expand to support Vesta curves in the future.
- Usage: In Halo2, when using elliptic curves, it is usually necessary to use the EccChip component to perform elliptic curve operations.

8.**Plonky2**
- Core design:

  The core design of Plonky2 does not rely on elliptic curves, but is based on polynomial commitments and FRI protocols. Although the core architecture of Plonky2 avoids the complexity of elliptic curves, it may use elliptic curves in some auxiliary operations.
