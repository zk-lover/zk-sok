# plonky2

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


