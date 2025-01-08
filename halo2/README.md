# halo2

Halo2 is a library for building zero-knowledge proof (ZKP) circuits, which aims to provide developers with efficient and flexible tools to design and verify zero-knowledge proofs, especially supporting the Plonk protocol and the Groth16 protocol. It is the second-generation library of the Halo series developed by the Zcash team, which aims to help developers achieve privacy protection and decentralized computing in a wider range of scenarios by providing a more efficient, scalable and upgradeable zero-knowledge proof system.

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


