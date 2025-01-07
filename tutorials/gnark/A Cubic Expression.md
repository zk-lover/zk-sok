### A Cubic Expression

### A Cubic Expression Implemented Based on Groth16

This code defines a simple circuit `CubicCircuit` with the constraint x^3 + x + 1 = y. It uses the `gnark` library to compile the circuit into an R1CS (Rank-1 Constraint System) and performs setup, proof, and verification using the `groth16` zkSNARK. The code also starts an HTTP server for performance profiling. The steps include compiling the circuit, setting up proving and verifying keys, defining a witness, generating a proof, and verifying it. It outputs the sizes of the proving key, verifying key, and proof.

Below, we will divide the code into code blocks and annotate them.

###### The first code block is used to import the necessary packages.

```Go
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

* `bytes` and `encoding/gob` are used for serializing and deserializing data.

* `fmt` and `log` are used for outputting information and logging.

* gnark-related packages are used for the implementation of zkSNARK.

* `net/http` and `net/http/pprof` are used to start an HTTP server for performance profiling.

###### The second code block is used to define the circuit.

```Go
type CubicCircuit struct {
	X frontend.Variable `gnark:"x"`
	Y frontend.Variable `gnark:",public"`
}
```

* `CubicCircuit` is a struct that defines a simple circuit representing the equation x^3 + x + 1 = y.

* `X` is a secret variable, and `Y` is a public variable.

###### The third code block is used to define the circuit constraints.

```Go
func (circuit *CubicCircuit) Define(api frontend.API) error {
	x3 := api.Mul(circuit.X, circuit.X, circuit.X)
	api.AssertIsEqual(circuit.Y, api.Add(x3, circuit.X, 1))
	return nil
}
```

The `Define` method specifies the circuit's constraint condition, which is x^3 + x + 1 = y.

###### Next is the main function.

```Go
go func() {
	log.Println(http.ListenAndServe("0.0.0.0:6060", nil))
}()
```

It starts an HTTP server listening on `0.0.0.0:6060` for performance profiling.

```Go
var circuit CubicCircuit
ccs, _ := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
```

Compile the circuit into R1CS (Rank-1 Constraint System).

```Go
pk, vk, _ := groth16.Setup(ccs)
```

Use the Groth16 to set up zkSNARK, generating the proving key (pk) and the verifying key (vk).

```Go
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

Serialize the proving key and verifying key, and output their sizes.

```Go
assignment := CubicCircuit{X: 3, Y: 31}
witness, _ := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
publicWitness, _ := witness.Public()
```

Define a circuit instance with `X = 3` and `Y = 31`, and create a witness.

```Go
proof, _ := groth16.Prove(ccs, pk, witness)

var buf bytes.Buffer
enc := gob.NewEncoder(&buf)
_ = enc.Encode(proof)
proofSize := buf.Len()
fmt.Printf("Proof size: %d bytes\n", proofSize)

groth16.Verify(proof, vk, publicWitness)
```

Generate the proof and verify it. Serialize the proof and output its size.

#### A Cubic Expression Implemented Based on Plonk

This code defines a simple circuit `CubicCircuit` to verify the polynomial equation x^3 + x + 1 = y. It uses the `gnark` library to compile the circuit into a constraint system (R1CS). The code then creates the necessary data for a KZG commitment scheme and uses the `plonk` protocol for setup, proof generation, and verification. Finally, it calculates and outputs the sizes of the generated proof and the structured reference string (SRS).

Below, we will divide the code into code blocks and annotate them.

```Go
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

Import the packages for the Plonk scheme.

```Go
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

Then define the circuit structure and circuit constraints, which are the same as in the Groth16 scheme.

##### Main

###### Compile the circuit

```Go
var circuit CubicCircuit
ccs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
if err != nil {
	fmt.Println("circuit compilation error")
}
```

* Define the circuit instance: Create an instance of `CubicCircuit`.

* Compile the circuit: Use `frontend.Compile` to compile the circuit into R1CS (Rank-1 Constraint System), which is an intermediate representation of the circuit suitable for subsequent proof generation.

###### Create KZG data

```Go
scs := ccs.(*cs.SparseR1CS)
srs, srsLagrange, err := unsafekzg.NewSRS(scs)
if err != nil {
	panic(err)
}
```

* Type conversion: Convert the compiled circuit to the `SparseR1CS` type.

* Generate SRS: Call `unsafekzg.NewSRS` to generate the Structured Reference String (SRS) and its Lagrange form, used for the KZG commitment scheme.

###### Serialize SRS

```Go
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

* Serialize SRS: Use the `gob` encoder to serialize the SRS and its Lagrange form into a byte buffer.

* Output size: Print the size of the serialized SRS and SRS Lagrange form.

###### Plonk setup

```Go
pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)
if err != nil {
	log.Fatal(err)
}
```

Use the `plonk.Setup` function to generate the proving key (pk) and the verifying key (vk).

###### Define the witness

```Go
assignment := CubicCircuit{X: 3, Y: 31}
witness, _ := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
publicWitness, _ := witness.Public()
```

* Create assignments: Provide specific values for the input variable `X` and the output variable `Y` of the circuit.

* Generate the witness: Use `frontend.NewWitness` to generate the witness, which includes the circuit's inputs and outputs.

* Extract the public witness: Extract the public part from the witness for verification.

###### Generate and verify the proof

```Go
proof, err := plonk.Prove(ccs, pk, witness)
if err != nil {
	log.Fatal(err)
}

err = plonk.Verify(proof, vk, publicWitness)
if err != nil {
	log.Fatal(err)
}
```

* Generate the proof: Use the `plonk.Prove` function to generate the proof.

* Verify the proof: Use the `plonk.Verify` function to verify the generated proof.

###### Serialize the proof and output its size

```Go
var buf bytes.Buffer
enc := gob.NewEncoder(&buf)
_ = enc.Encode(proof)
proofSize := buf.Len()
fmt.Printf("Proof size: %d bytes\n", proofSize)
```

