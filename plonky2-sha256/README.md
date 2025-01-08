## Running examples

This code is from https://github.com/polymerdao/plonky2-sha256. We only made a few minor modifications to make it more suitable as an example program.

## Docker setup

Create a Docker image. This will take a few minutes. You only have to do 
this once.
```
$ docker build -t plonky2-sha256 .
```

Spin up a Docker container from the image.
```
$ docker run -it --rm plonky2-sha256
```

Please note that any changes you make in the container are not persistent. 

Navigate to the directory of the program you would like to run.
Our examples are at `/root/src`.
Run the following code to execute the examples.
```
$ cargo run 
```