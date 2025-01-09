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

1.**cubic_expression**:

- This program demonstrates a cubic arithmetic polynomial, proving \(x^3 + x + 5 = 35\). It is a classic example used by many libraries, papers, and blogs. By constructing a zero-knowledge proof for cubic_expression, users can see the basic usage of ZKP.

2. **range_proof**:
   
   - This program proves that a number \(x\) is within a specified range. In our example, it proves that \(x\) is between 0 and 2^32. This scenario is practical for zero-knowledge proofs, such as the range proof of transaction amounts on the blockchain. The usual practice involves decomposing \(x\) into binary bits, determining whether each bit is 0 or 1, and ensuring the reconstructed value equals the original value.
3. **sha256**:
   
   - This program implements the SHA-256 hash, a common cryptographic primitive widely used in various scenarios. Due to its complex construction and calculation process, using zero-knowledge proof requires tens of thousands of circuit constraints. This example allows users to assess the zkSNARK library's ability to implement complex circuits.

## Curve

libsnark: “edwards”：基于 Edwards 曲线的实例，提供 80 位安全性;“bn128”：基于 Barreto-Naehrig 曲线的实例，提供 128 位安全性。底层曲线实现是 [ate-pairing];“alt_bn128”：“bn128”的替代品，速度稍慢，但避免了动态代码生成。bn128 需要 x86-64 CPU，而其他曲线选择应该与架构无关;MNT4、MNT6;
cmake -DCURVE=choice（其中choice是以下之一：ALT_BN128、BN128、EDWARDS、MNT4、MNT6）
将默认曲线设置为上述之一（参见椭圆曲线选择）。

arkworks: 

BLS12-381 and embedded curves:
ark-bls12-381: Implements the BLS12-381 pairing-friendly curve
ark-ed-on-bls12-381: Implements a Twisted Edwards curve atop the scalar field of BLS12-381
ark-ed-on-bls12-381-bandersnatch: Implements Bandersnatch, another Twisted Edwards curve atop the scalar field of BLS12-381

BLS12-377 and related curves:
ark-bls12-377: Implements the BLS12-377 pairing-friendly curve
ark-ed-on-bls12-377: Implements a Twisted Edwards curve atop the scalar field of BLS12-377
ark-bw6-761: Implements the BW6-761 pairing-friendly curve, which is a curve whose scalar field equals the base field of BLS12-377
ark-ed-on-bw6-761: Implements a Twisted Edwards curve atop the scalar field of BW6-761
ark-cp6-782: Implements the CP6-782 pairing-friendly curve, which is a curve whose scalar field equals the base field of BLS12-377
ark-ed-on-cp6-782: Implements a Twisted Edwards curve atop the scalar field of CP6-782. This is the same curve as in ark-ed-on-bw6-761

BN254 and related curves
ark-bn254: Implements the BN254 pairing-friendly curve
ark-ed-on-bn254: Implements a Twisted Edwards curve atop the scalar field of BN254
ark-grumpkin: Implements the Grumpkin curve. A curve that forms a cycle with bn254.

MNT-298 cycle of curves and related curves
ark-mnt4-298: Implements the MNT4-298 pairing-friendly curve. This curve forms a pairing-friendly cycle with MNT6-298
ark-mnt6-298: Implements the MNT6-298 pairing-friendly curve. This curve forms a pairing-friendly cycle with MNT4-298
ark-ed-on-mnt4-298: Implements a Twisted Edwards curve atop the scalar field of MNT4-298

MNT-753 cycle of curves and related curves
ark-mnt4-753: Implements the MNT4-753 pairing-friendly curve. This curve forms a pairing-friendly cycle with MNT6-753
ark-mnt6-753: Implements the MNT6-753 pairing-friendly curve. This curve forms a pairing-friendly cycle with MNT4-753
ark-ed-on-mnt4-753: Implements a Twisted Edwards curve atop the scalar field of MNT4-753

Pasta cycle of curves
ark-pallas: Implements Pallas, a prime-order curve that forms an amicable pair with Vesta
ark-vesta: Implements Vesta, a prime-order curve that forms an amicable pair with Pallas

gnark:Elliptic curve cryptography & Pairing on:
bn254 (audit report)
bls12-381 (audit report)
bls24-317
bls12-377 / bw6-761
bls24-315 / bw6-633
Each of these curves has a twistededwards sub-package with its companion curve which allow efficient elliptic curve cryptography inside zkSNARK circuits.

snarkjs:
The first parameter after new refers to the type of curve you wish to use. At the moment, we support both bn128 and bls12-381.

libiop:
无

Spartan:
Standardized security: Spartan's security relies on the hardness of computing discrete logarithms (a standard cryptographic assumption) in the random oracle model. libspartan uses ristretto255, a prime-order group abstraction atop curve25519 (a high-speed elliptic curve). We use curve25519-dalek for arithmetic over ristretto255.

halo2：EccChip
halo2_gadgets provides a chip that implements EccInstructions using 10 advice columns. The chip is currently restricted to the Pallas curve, but will be extended to support the Vesta curve in the near future.

Plonky2：To avoid the difficulties associated with elliptic curve cycles, the core design of Plonky2 is based on polynomial commitments and the FRI protocol
