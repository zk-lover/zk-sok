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
	"github.com/consensys/gnark/std/rangecheck"
)

type Circuit struct {
	Vals frontend.Variable
	bits int
}

func (c *Circuit) Define(api frontend.API) error {
	r := rangecheck.New(api)
	r.Check(c.Vals, c.bits)
	return nil
}

func main() {

	// Step 1: Compile the circuit

	circuit := Circuit{
		bits: 32,
	}
	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		log.Fatalf("failed to compile circuit: %v", err)
	}

	// Step 2: Groth16 zkSNARK: Setup
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

	assignment := Circuit{
		Vals: 665115184,
		bits: 32,
	}
	// Step 3: Create a witness
	//此处hashfunc需要和ccs的曲线相同

	// Step 4: Assign values to the circuit

	// Step 5: Generate the full witness
	witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	if err != nil {
		log.Fatalf("failed to create witness: %v", err)
	}

	// Step 6: Extract the public witness
	publicWitness, err := witness.Public()
	if err != nil {
		log.Fatalf("failed to extract public witness: %v", err)
	}

	// Step 7: Prove the circuit
	proof, err := groth16.Prove(ccs, pk, witness)
	if err != nil {
		log.Fatalf("failed to generate proof: %v", err)
	}

	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	_ = enc.Encode(proof)
	proofSize := buf.Len()
	fmt.Printf("Proof size: %d bytes\n", proofSize)

	// Step 8: Verify the proof
	err = groth16.Verify(proof, vk, publicWitness)
	if err != nil {
		log.Fatalf("failed to verify proof: %v", err)
	}

	fmt.Println("Proof verified successfully")
}
