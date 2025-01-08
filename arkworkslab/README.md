# arkworks
Arkworks is a Rust ecosystem for zk-SNARK programming.The core goal of arkworks is to provide a powerful cryptographic library for the Rust ecosystem, covering a wide range of mathematical and cryptographic algorithms, with a particular focus on supporting efficient ZKP architectures. It includes implementations of elliptic curves, finite fields, homomorphic encryption, SNARK, and other cryptographic primitives, allowing developers to easily implement complex cryptographic protocols in the Rust environment.

Their crypto-primitives library provides a lot of useful cryptographic primitive gadgets, such as commitment, signature, encryption, etc.

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


