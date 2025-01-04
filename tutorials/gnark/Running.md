### Run the Example Programs

We recommend running our example programs in an Ubuntu virtual machine. First, you need to install the Go language. You can follow these steps to install it:

Download and extract the Go language archive.

```markup
sudo apt update
wget https://go.dev/dl/go1.22.4.linux-amd64.tar.gz -P /usr/local
cd /usr/local
sudo tar -C /usr/local -xzf go1.22.4.linux-amd64.tar.gz
```

Configure the environment variables.

* Edit the `~/.bashrc` file:

```markup
nano ~/.bashrc
```

* Add the following contents:

```markup
export GOROOT=/usr/local/go
export GOPATH=$HOME/go
export PATH=$PATH:/usr/local/go/bin
```

* Apply the changes:

```markup
source ~/.bashrc
```

Verify if Go is installed successfully. If the version information is displayed, the installation was successful:

```markup
go version
```

After successfully installing Go, you can navigate to the folder containing the program source code and run the executable file using the following command:

```markup
./test
```

Note: If access is denied, you need to add execute permissions using the following command:

```markup
ls -l test
chmod +x test
```

Then execute it again;

Alternatively, you can recompile and run it:

```markup
go build -o test
./test
```

Or you can run directly:

```markup
go run test.go
```

### Your own code

You can write your code after importing the necessary gnark libraries.
