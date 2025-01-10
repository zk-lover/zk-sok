# plonky2

Plonky2 is an optimized version of Plonk, a common general-purpose zero-knowledge proof protocol, and aims to achieve higher performance than traditional ZKP systems (such as zk-SNARKs and zk-STARKs) through deep optimization of mathematics and implementation. It is able to generate and verify proofs very quickly, especially suitable for scenarios that require low latency and high throughput. Plonky2 uses FRI as a core building block and is able to generate SNARK proofs with very small proof size and verification overhead.

plonky2 provides a set of powerful and modular gadgets for building zero-knowledge proof circuits. These gadgets simplify common cryptographic and arithmetic operations, making it easier for developers to build efficient circuits for real-world applications. Key gadgets include support for arithmetic operations, polynomial operations, lookup tables, range checking, and hash functions, among others.

Next we show how to build Docker to run our sample program based on the plonky2 library.
    
## Docker setup

Create a Docker image. This will take a few minutes. You only have to do 
this once.
```
$ docker build -t plonky2 .
```

Spin up a Docker container from the image.
```
$ docker run -it --rm plonky2
```

Please note that any changes you make in the container are not persistent. 
## Running examples

Since plonky2 does not provide gadgets related to sha256 circuit construction, we sought an open source implementation of the forked plonky2 library and placed it in our plonky2-sha256 directory.

Navigate to the directory of the program you would like to run.
Our examples are at `/root/src`.
Run the following code to execute the examples.
```
$ cargo run --bin cubic_expression
$ cargo run --bin range_proof
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


