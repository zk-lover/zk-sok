### Sample programs using the gnark library

We have implemented the three example programs mentioned in our paper based on gnark, including:

* A Cubic Expression

* Range Proof

* Hash Function

The gnark library supports both Groth16 and Plonk schemes, so we implemented our example programs using both schemes.&#x20;

The source code for the programs is located in the `test.go` files within six files under the `gnarklab` folder.

For the "A Cubic Expression" example program we implemented, the source code based on the Groth16 scheme is located at:

> gnarklab\testCubitequation\test.go

The source code based on the Plonk scheme is located at:

> gnarklab\testCubquation_plonk\test.go

The same applies to the other example programs.

This tutorial will introduce how to run the example programs and provide an overview of our example program code to demonstrate the use of gnark.

### 1. Run the Example Programs

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

### 2.Annotations of the example programs

Next, we will annotate the code of our example programs.

#### A Cubic Expression Implemented Based on Groth16

The first code block is used to import the necessary packages.

* `bytes` and `encoding/gob` are used for serializing and deserializing data.

* `fmt` and `log` are used for outputting information and logging.

* gnark-related packages are used for the implementation of zkSNARK.

* `net/http` and `net/http/pprof` are used to start an HTTP server for performance profiling.

```markup
import (
	"bytes"
	"encoding/gob"
	"fmt"
	"log"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"

	"net/http"
	_ "net/http/pprof"
)
```

The second code block is used to define the circuit.

* `CubicCircuit` is a struct that defines a simple circuit representing the equation x^3 + x + 1 = y.

* `X` is a secret variable, and `Y` is a public variable.

```markup
type CubicCircuit struct {
	X frontend.Variable `gnark:"x"`
	Y frontend.Variable `gnark:",public"`
}
```

The third code block is used to define the circuit constraints. The `Define` method specifies the circuit's constraint condition, which is x^3 + x + 1 = y.

```markup
func (circuit *CubicCircuit) Define(api frontend.API) error {
	x3 := api.Mul(circuit.X, circuit.X, circuit.X)
	api.AssertIsEqual(circuit.Y, api.Add(x3, circuit.X, 1))
	return nil
}
```

Next is the main function.

It starts an HTTP server listening on `0.0.0.0:6060` for performance profiling.

```markup
go func() {
	log.Println(http.ListenAndServe("0.0.0.0:6060", nil)) // 确保监听在所有接口上
}()
```

Compile the circuit into R1CS (Rank-1 Constraint System).

```markup
var circuit CubicCircuit
ccs, _ := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
```

Use the Groth16 to set up zkSNARK, generating the proving key (pk) and the verifying key (vk).

```markup
pk, vk, _ := groth16.Setup(ccs)
```

Serialize the proving key and verifying key, and output their sizes.

```markup
var buf1 bytes.Buffer
enc1 := gob.NewEncoder(&buf1)
_ = enc1.Encode(pk)
pkSize := buf1.Len()
fmt.Printf("pk size: %d bytes\n", pkSize)

var buf2 bytes.Buffer
enc2 := gob.NewEncoder(&buf2)
_ = enc2.Encode(vk)
vkSize := buf2.Len()
fmt.Printf("vk size: %d bytes\n", vkSize)
```

Define a circuit instance with `X = 3` and `Y = 31`, and create a witness.

```markup
assignment := CubicCircuit{X: 3, Y: 31}
witness, _ := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
publicWitness, _ := witness.Public()
```

Generate the proof and verify it. Serialize the proof and output its size.

```markup
proof, _ := groth16.Prove(ccs, pk, witness)

var buf bytes.Buffer
enc := gob.NewEncoder(&buf)
_ = enc.Encode(proof)
proofSize := buf.Len()
fmt.Printf("Proof size: %d bytes\n", proofSize)

groth16.Verify(proof, vk, publicWitness)
```

#### A Cubic Expression Implemented Based on Plonk

Import the packages for the Plonk scheme.

```markup
import (
	"bytes"
	"encoding/gob"
	"fmt"
	"log"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	cs "github.com/consensys/gnark/constraint/bn254"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/consensys/gnark/test/unsafekzg"
)
```

Then define the circuit structure and circuit constraints, which are the same as in the Groth16 scheme.

```markup
type CubicCircuit struct {
	X frontend.Variable `gnark:"x"`
	Y frontend.Variable `gnark:",public"`
}

func (circuit *CubicCircuit) Define(api frontend.API) error {
	x3 := api.Mul(circuit.X, circuit.X, circuit.X)
	api.AssertIsEqual(circuit.Y, api.Add(x3, circuit.X, 1))
	return nil
}
```

Main():

Compile the circuit:

* Define the circuit instance: Create an instance of `CubicCircuit`.

* Compile the circuit: Use `frontend.Compile` to compile the circuit into R1CS (Rank-1 Constraint System), which is an intermediate representation of the circuit suitable for subsequent proof generation.

```markup
var circuit CubicCircuit
ccs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
if err != nil {
	fmt.Println("circuit compilation error")
}
```

Create KZG data

* Type conversion: Convert the compiled circuit to the `SparseR1CS` type.

* Generate SRS: Call `unsafekzg.NewSRS` to generate the Structured Reference String (SRS) and its Lagrange form, used for the KZG commitment scheme.

```go
scs := ccs.(*cs.SparseR1CS)
srs, srsLagrange, err := unsafekzg.NewSRS(scs)
if err != nil {
	panic(err)
}
```

Serialize SRS

Serialize SRS: Use the `gob` encoder to serialize the SRS and its Lagrange form into a byte buffer.

Output size: Print the size of the serialized SRS and SRS Lagrange form.

```markup
var buf1 bytes.Buffer
enc1 := gob.NewEncoder(&buf1)
_ = enc1.Encode(srs)
srsSize := buf1.Len()
fmt.Printf("srs size: %d bytes\n", srsSize)

var buf2 bytes.Buffer
enc2 := gob.NewEncoder(&buf1)
_ = enc2.Encode(srsLagrange)
srsLagrangeSize := buf2.Len()
fmt.Printf("srsLagrangeSize size: %d bytes\n", srsLagrangeSize)
```

Plonk setup: Use the `plonk.Setup` function to generate the proving key (pk) and the verifying key (vk).

```markup
pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)
if err != nil {
	log.Fatal(err)
}
```

Define the witness

* Create assignments: Provide specific values for the input variable `X` and the output variable `Y` of the circuit.

* Generate the witness: Use `frontend.NewWitness` to generate the witness, which includes the circuit's inputs and outputs.

* Extract the public witness: Extract the public part from the witness for verification.

```markup
assignment := CubicCircuit{X: 3, Y: 31}
witness, _ := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
publicWitness, _ := witness.Public()
```

Generate and verify the proof

* Generate the proof: Use the `plonk.Prove` function to generate the proof.

* Verify the proof: Use the `plonk.Verify` function to verify the generated proof.

```markup
proof, err := plonk.Prove(ccs, pk, witness)
if err != nil {
	log.Fatal(err)
}

err = plonk.Verify(proof, vk, publicWitness)
if err != nil {
	log.Fatal(err)
}
```

Serialize the proof and output its size.

```markup
var buf bytes.Buffer
enc := gob.NewEncoder(&buf)
_ = enc.Encode(proof)
proofSize := buf.Len()
fmt.Printf("Proof size: %d bytes\n", proofSize)
```

#### Range Proof Implemented Based on Groth16

Define the circuit

* The `Circuit` struct defines a circuit that includes a variable `Vals` and an integer `bits`.

* The `Define` method specifies the circuit's constraints, using `rangecheck` to verify if `Vals` is within the specified bit range.

```markup
type Circuit struct {
	Vals frontend.Variable
	bits int
}

func (c *Circuit) Define(api frontend.API) error {
	r := rangecheck.New(api)
	r.Check(c.Vals, c.bits)
	return nil
}
```

Main():

Create a `Circuit` instance and compile the circuit using the `frontend.Compile` function to compile the circuit into a constraint system (ccs). Here, `ecc.BN254.ScalarField()` is used as the scalar field, and `scs.NewBuilder` is used as the builder.

```markup
circuit := Circuit{
	bits: 32,
}
ccs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
if err != nil {
	log.Fatalf("failed to compile circuit: %v", err)
}
```

Use the Groth16 protocol to set up zkSNARK, generating the proving key `pk` and the verifying key `vk`.

```markup
pk, vk, err := groth16.Setup(ccs)
if err != nil {
	log.Fatalf("failed to setup groth16: %v", err)
}
```

Serialize and output the size of the proving key and verifying key.

```markup
var buf1 bytes.Buffer
enc1 := gob.NewEncoder(&buf1)
_ = enc1.Encode(pk)
pkSize := buf1.Len()
fmt.Printf("pk size: %d bytes\n", pkSize)

var buf2 bytes.Buffer
enc2 := gob.NewEncoder(&buf2)
_ = enc2.Encode(vk)
vkSize := buf2.Len()
fmt.Printf("vk size: %d bytes\n", vkSize)
```

Create a `Circuit` instance as the assignment and generate the witness.

```markup
assignment := Circuit{
	Vals: 665115184,
	bits: 32,
}
witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
if err != nil {
	log.Fatalf("failed to create witness: %v", err)
}
```

Extract the public witness.

```markup
publicWitness, err := witness.Public()
if err != nil {
	log.Fatalf("failed to extract public witness: %v", err)
}
```

Generate the proof, then encode the proof and output its size.

```markup
proof, err := groth16.Prove(ccs, pk, witness)
if err != nil {
	log.Fatalf("failed to generate proof: %v", err)
}

var buf bytes.Buffer
enc := gob.NewEncoder(&buf)
_ = enc.Encode(proof)
proofSize := buf.Len()
fmt.Printf("Proof size: %d bytes\n", proofSize)
```

Verify the proof and output the result.

```markup
err = groth16.Verify(proof, vk, publicWitness)
if err != nil {
	log.Fatalf("failed to verify proof: %v", err)
}
fmt.Println("Proof verified successfully")
```

#### Range Proof Implemented Based on Plonk

The circuit structure and circuit constraints are the same as in the Groth16.

Main():

Compile the circuit.

```markup
circuit := Circuit{
    bits: 32,
}
ccs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
if err != nil {
    fmt.Println("circuit compilation error")
}
```

Generate SRS

* Convert the constraint system: Convert the compiled ccs to the sparse R1CS format.

* Generate SRS: Use `unsafekzg.NewSRS` to generate the Structured Reference String (SRS) and the Lagrange form of the SRS.

```markup
scs := ccs.(*cs.SparseR1CS)
srs, srsLagrange, err := unsafekzg.NewSRS(scs)
if err != nil {
    panic(err)
}
```

Encode SRS: Use the `gob` encoder to encode the SRS and the SRS Lagrange form into a byte stream, then calculate and output the size of the encoded SRS and SRS Lagrange form.

```markup
var buf1 bytes.Buffer
enc1 := gob.NewEncoder(&buf1)
_ = enc1.Encode(srs)
srsSize := buf1.Len()
fmt.Printf("srs size: %d bytes\n", srsSize)

var buf2 bytes.Buffer
enc2 := gob.NewEncoder(&buf1)
_ = enc2.Encode(srsLagrange)
srsLagrangeSize := buf2.Len()
fmt.Printf("srsLagrangeSize size: %d bytes\n", srsLagrangeSize)
```

Use Plonk to set up zkSNARK, generating the proving key (pk) and the verifying key (vk).

```markup
pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)
if err != nil {
    log.Fatal(err)
}
```

Create a `Circuit` instance as the assignment and generate the witness.

```markup
assignment := Circuit{
    Vals: 665115184,
    bits: 32,
}
witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
if err != nil {
    log.Fatalf("failed to create witness: %v", err)
}
```

Extract the public witness.

```markup
publicWitness, err := witness.Public()
if err != nil {
    log.Fatalf("failed to extract public witness: %v", err)
}
```

Generate the proof, then encode the proof and output its size.

```markup
proof, err := plonk.Prove(ccs, pk, witness)
if err != nil {
    log.Fatal(err)
}
    
var buf bytes.Buffer
enc := gob.NewEncoder(&buf)
_ = enc.Encode(proof)
proofSize := buf.Len()
fmt.Printf("Proof size: %d bytes\n", proofSize)
```

Verify the proof.

```markup
err = plonk.Verify(proof, vk, publicWitness)
if err != nil {
    log.Fatal(err)
}
fmt.Println("Proof verified successfully")
```

#### Hash Function Implemented Based on Groth16

Define the circuit structure.

```markup
type Circuit struct {
	In       []uints.U8
	Expected [32]uints.U8 `gnark:",public"`
}
```

Define the circuit constraints: The `Define` method specifies the circuit's constraint logic. It uses the SHA-256 hash function to compute the hash of the input and compares it with the expected value.

* Create an instance `h` of the SHA-256 hash function.

* Write the input `In` to the hash function.

* Compute the hash value `res` and check if its length is 32 bytes.

* Use the `uapi.ByteAssertEq` method to compare the computed hash value with the expected value `Expected` byte by byte.

```markup
func (c *Circuit) Define(api frontend.API) error {
	h, err := sha2.New(api)
	if err != nil {
		return err
	}
	uapi, err := uints.New[uints.U32](api)
	if err != nil {
		return err
	}
	h.Write(c.In)
	res := h.Sum()
	if len(res) != 32 {
		return fmt.Errorf("not 32 bytes")
	}
	for i := range c.Expected {
		uapi.ByteAssertEq(c.Expected[i], res[i])
	}
	return nil
}
```

Main():

First, compile the circuit. The input string is converted into a byte array, and its SHA-256 hash value is computed.

* Convert the input string into a byte array `inbyte`.

* Compute its SHA-256 hash value `hash`.

* Create a `Circuit` instance `circuit` and assign the input byte array to `In`.

* Use `frontend.Compile` to compile the circuit, generating the constraint system `ccs`.

```markup
input := "xxxxxx"
inbyte := []byte(input)
hash := sha256.Sum256(inbyte)
circuit := Circuit{
	In: uints.NewU8Array(inbyte),
}
ccs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
if err != nil {
	log.Fatalf("failed to compile circuit: %v", err)
}
```

Use Groth16 to set up zkSNARK, generating the proving key `pk` and the verifying key `vk`, and output their sizes.

* Use `groth16.Setup` to generate the proving key `pk` and the verifying key `vk`.

* Use the `gob` encoder to calculate the sizes of `pk` and `vk`, and output them.

```markup
pk, vk, err := groth16.Setup(ccs)
if err != nil {
	log.Fatalf("failed to setup groth16: %v", err)
}

var buf1 bytes.Buffer
enc1 := gob.NewEncoder(&buf1)
_ = enc1.Encode(pk)
pkSize := buf1.Len()
fmt.Printf("pk size: %d bytes\n", pkSize)

var buf2 bytes.Buffer
enc2 := gob.NewEncoder(&buf2)
_ = enc2.Encode(vk)
vkSize := buf2.Len()
fmt.Printf("vk size: %d bytes\n", vkSize)
```

Create the witness for the circuit, assigning input values and the expected hash value.

* Create a `Circuit` instance `assignment` and assign the input byte array to `In`.

* Copy the computed hash value to the `Expected` field.

* Use `frontend.NewWitness` to generate the complete witness `witness`.

```markup
assignment := Circuit{
	In: uints.NewU8Array(inbyte),
}
copy(assignment.Expected[:], uints.NewU8Array(hash[:]))
witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
if err != nil {
	log.Fatalf("failed to create witness: %v", err)
}
```

Extract the public witness.

```markup
publicWitness, err := witness.Public()
if err != nil {
	log.Fatalf("failed to extract public witness: %v", err)
}
```

Generate and verify the proof.

* Use `groth16.Prove` to generate the proof `proof`.

* Use the `gob` encoder to calculate the size of the proof `proofSize`, and output it.

* Use `groth16.Verify` to verify the correctness of the proof.

```markup
proof, err := groth16.Prove(ccs, pk, witness)
if err != nil {
	log.Fatalf("failed to generate proof: %v", err)
}

var buf bytes.Buffer
enc := gob.NewEncoder(&buf)
_ = enc.Encode(proof)
proofSize := buf.Len()
fmt.Printf("Proof size: %d bytes\n", proofSize)

err = groth16.Verify(proof, vk, publicWitness)
if err != nil {
	log.Fatalf("failed to verify proof: %v", err)
}

fmt.Println("Proof verified successfully")
```

#### Hash Function Implemented Based on Plonk

The circuit structure and circuit constraints are the same as in the Groth16 scheme.

Main():

First, compile the circuit. Convert the input string into a byte array and compute its SHA-256 hash value.

Convert the input string "hudaqi" into a byte array `inbyte`.

* Compute its SHA-256 hash value `hash`.

* Create a `Circuit` instance `circuit` and assign the input byte array to `In`.

* Use `frontend.Compile` to compile the circuit, generating the constraint system `ccs`.

```markup
input := "hudaqi"
inbyte := []byte(input)
hash := sha256.Sum256(inbyte)
circuit := Circuit{
	In: uints.NewU8Array(inbyte),
}
ccs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
if err != nil {
	fmt.Println("circuit compilation error")
}
```

Generate the SRS (Structured Reference String) and calculate its size.

* Convert the constraint system `ccs` to the sparse R1CS format `scs`.

* Use `unsafekzg.NewSRS` to generate the Structured Reference String `SRS` and `SRS Lagrange`.

* Use the `gob` encoder to calculate the sizes of `SRS` and `SRS Lagrange`.

```markup
scs := ccs.(*cs.SparseR1CS)
srs, srsLagrange, err := unsafekzg.NewSRS(scs)
if err != nil {
	panic(err)
}
var buf1 bytes.Buffer
enc1 := gob.NewEncoder(&buf1)
_ = enc1.Encode(srs)
srsSize := buf1.Len()
fmt.Printf("srs size: %d bytes\n", srsSize)

var buf2 bytes.Buffer
enc2 := gob.NewEncoder(&buf1)
_ = enc2.Encode(srsLagrange)
srsLagrangeSize := buf2.Len()
fmt.Printf("srsLagrangeSize size: %d bytes\n", srsLagrangeSize)
```

Set up the parameters for zkSNARK, including generating the proving key and verifying key.

* Use `plonk.Setup` to generate the proving key `pk` and the verifying key `vk`.

```markup
pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)
if err != nil {
	log.Fatal(err)
}
```

Create the circuit assignment:

* Create a `Circuit` instance `assignment` and assign the input byte array to `In`.

* Copy the computed hash value to the `Expected` field.

```markup
assignment := Circuit{
	In: uints.NewU8Array(inbyte),
}
copy(assignment.Expected[:], uints.NewU8Array(hash[:]))
```

Generate the witness and extract the public witness.

* Use `frontend.NewWitness` to generate the complete witness `witness`, which is used to prove the correctness of the circuit.

* Extract the public witness `publicWitness` from the complete witness.

```markup
witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
if err != nil {
	log.Fatalf("failed to create witness: %v", err)
}
publicWitness, err := witness.Public()
if err != nil {
	log.Fatalf("failed to extract public witness: %v", err)
}
```

Generate and verify the proof:

* Use `plonk.Prove` to generate the proof `proof`.

* Use `plonk.Verify` to verify the correctness of the proof.

* Use the `gob` encoder to calculate the size of the proof `proofSize`.

* Output the proof size and confirm the verification success.

```markup
proof, err := plonk.Prove(ccs, pk, witness)
if err != nil {
	log.Fatal(err)
}

err = plonk.Verify(proof, vk, publicWitness)
if err != nil {
	log.Fatal(err)
}
var buf bytes.Buffer
enc := gob.NewEncoder(&buf)
_ = enc.Encode(proof)
proofSize := buf.Len()
fmt.Printf("Proof size: %d bytes\n", proofSize)

fmt.Println("Proof verified successfully")
```

### 3. Your Own Code

You can write your code after importing the necessary gnark libraries.
