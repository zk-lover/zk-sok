# spartan

Spartan is an efficient zero-knowledge proof (ZKP) system that aims to provide concise and low-cost proof generation and verification capabilities. It was developed by researchers at the Massachusetts Institute of Technology (MIT) and focuses on efficient, flexible zero-knowledge proof protocols with low computational and storage overhead. Spartan is a specially optimized SNARK (Succinct Non-Interactive Zero-Knowledge Proof) system, mainly used in decentralized applications, blockchains, and cryptographic protocols that require privacy protection and data verification.

Spartan mainly provides some basic gadgets to assist in building zero-knowledge proof circuits, but does not provide ready-made solutions for specific applications (such as hash functions, signature algorithms, encryption primitives, etc.). Therefore, if you need to implement these personalized applications in Spartan, you may need to design and build the corresponding circuit components yourself according to actual needs. This usually involves converting the logic of the application into a circuit structure and verifying and proving it through the basic functions provided by Spartan. 

Next we show how to build Docker to run our sample program based on the Spartan library.    

## Docker setup

Create a Docker image. This will take a few minutes. You only have to do 
this once.
```
$ docker build -t spartan .
```

Spin up a Docker container from the image.
```
$ docker run -it --rm spartan
```

Please note that any changes you make in the container are not persistent. 

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


