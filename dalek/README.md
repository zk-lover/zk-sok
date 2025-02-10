# dalek

dalek-bulletproofs is a Rust-based zero-knowledge proof (ZKP) library that focuses on the efficient implementation of the Bulletproofs protocol. Bulletproofs is a zero-knowledge proof protocol that is primarily used to build compact and efficient proofs, especially suitable for cryptocurrency and privacy-preserving applications.

Dalek does not provide any firmware related to gadgets.

Next we show how to build Docker to run our sample program based on the dalek library.



## Docker setup

Create a Docker image. This will take a few minutes. You only have to do 
this once.
```
$ docker build -t delak .
```

Spin up a Docker container from the image.
```
$ docker run -it --rm dalek .
```

Please note that any changes you make in the container are not persistent. 

## Architecture

## Running examples

Navigate to the directory of the program you would like to run.
Our examples are at `/root/src`.
Run the following code to execute the examples.
```
$ cargo run --bin rangeproof
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


