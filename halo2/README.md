# halo2

Halo2 is a Rust library developed by Electric Coin Company(ECC). Halo2 introduces new features and improvements based on Plonk, including recursive proofs of proof systems and parallel computation. Due to the long review cycle in submission, halo2 has not been published yet, but a detailed online book is provided to demonstrate its design `Halo2 Book`.  
Halo2 provides many available functions for building a circuit. It implements redundant gate-level constraints,
including addition, multiplication, array, sum, etc. There are also high-level constraints like the inner product and range
check (with lookup tables). For popular zero-knowledge applications like hash and signature, halo2 provides an integrated
API as a part of its tools.

Next we show how to build Docker to run our sample program based on the halo2 library.

## Docker setup

Create a Docker image. This will take a few minutes. You only have to do 
this once.
```
$ docker build -t halo2 .
```

Spin up a Docker container from the image.
```
$ docker run -it --rm halo2
```

Please note that any changes you make in the container are not persistent. 

## Architecture

## Running examples

Navigate to the directory of the program you would like to run.
Our examples are at `/root/src`.
Run the following code to execute the examples.
```
$ cargo run --bin cubic_expression
$ cargo run --bin range_proof
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


