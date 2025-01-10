# arkworks
Arkworks is a Rust ecosystem for zk-SNARK programming that started in 2022. Arkworks implements several latest academic zk-SNARK approaches including Groth16, Plonk, marlin, gm17 etc.. The schemes implemented in Arkworks span
various categories and exhibit diverse properties, such as transparency, small proof size, URS, elastic proofs, and postquantum security.  
Arkworks provides an explicit toolchain for compiling circuits and choosing proof systems. In the compiling phase, there are three predefined data types: finite field, elliptic curve, and polynomial. Arkworks provides gadgets API for these data types to generate a generic constraint system (i.e., R1CS). When choosing proof systems, arkworks provides several sublibraries separating different categories of zk-SNARK approaches. We follow the tutorials in arkworks and successfully implement our sample programs with Groth16. Arkworks also provides a repository binding to
circom’s R1CS, facilitating the generation of Groth16 Proof and Witness generation in Rust.

Their crypto-primitives library provides a lot of useful cryptographic primitive gadgets, such as commitment, signature, encryption, etc.

Next we show how to build Docker to run our sample program based on the arkworks library.

## Docker setup

Create a Docker image. This will take a few minutes. You only have to do 
this once.
```
$ docker build -t arkworks .
```

Spin up a Docker container from the image.
```
$ docker run -it --rm arkworks
```

Please note that any changes you make in the container are not persistent. 

## Architecture

## Running examples

Navigate to the directory of the program you would like to run.
Our examples are at `/root/src`.
Run the following code to execute the examples.
```
$ cargo run --bin cubic_expression
$ cargo run --bin rangeproof
$ cargo run --bin sha256
```

## Modifying examples
Modifying examples is straightforward. Write your own rust file xxx.rs and add the following code to the Cargo.toml file.
```
[[bin]]
name = "xxx"
path = "src/xxx.rs"
```
Then you can run your code using
```
$ cargo run --bin xxx
```


