package main

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

// CubicCircuit defines a simple circuit
// x**3 + x + 1 == y
type CubicCircuit struct {
	// struct tags on a variable is optional
	// default uses variable name and secret visibility.
	X frontend.Variable `gnark:"x"`
	Y frontend.Variable `gnark:",public"`
}

// Define declares the circuit constraints
// x**3 + x + 1 == y
func (circuit *CubicCircuit) Define(api frontend.API) error {
	x3 := api.Mul(circuit.X, circuit.X, circuit.X)
	api.AssertIsEqual(circuit.Y, api.Add(x3, circuit.X, 1))
	return nil
}

func main() {
	go func() {
		log.Println(http.ListenAndServe("0.0.0.0:6060", nil)) // 确保监听在所有接口上
	}()

	// compiles our circuit into a R1CS
	var circuit CubicCircuit
	ccs, _ := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)

	// groth16 zkSNARK: Setup
	pk, vk, _ := groth16.Setup(ccs)

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

	// witness definition
	assignment := CubicCircuit{X: 3, Y: 31}
	witness, _ := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	publicWitness, _ := witness.Public()

	// groth16: Prove & Verify
	proof, _ := groth16.Prove(ccs, pk, witness)

	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	_ = enc.Encode(proof)
	proofSize := buf.Len()
	fmt.Printf("Proof size: %d bytes\n", proofSize)

	groth16.Verify(proof, vk, publicWitness)
}
