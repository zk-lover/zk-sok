package main

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
	// compiles our circuit into a R1CS
	var circuit CubicCircuit
	// // building the circuit...
	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
	if err != nil {
		fmt.Println("circuit compilation error")
	}

	// create the necessary data for KZG.
	// This is a toy example, normally the trusted setup to build ZKG
	// has been run before.
	// The size of the data in KZG should be the closest power of 2 bounding //
	// above max(nbConstraints, nbVariables).
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

	// public data consists of the polynomials describing the constants involved
	// in the constraints, the polynomial describing the permutation ("grand
	// product argument"), and the FFT domains.
	pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)
	//_, err := plonk.Setup(r1cs, kate, &publicWitness)
	if err != nil {
		log.Fatal(err)
	}

	// witness definition
	assignment := CubicCircuit{X: 3, Y: 31}
	witness, _ := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	publicWitness, _ := witness.Public()

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
}
