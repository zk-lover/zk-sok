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
	"github.com/consensys/gnark/std/rangecheck"
	"github.com/consensys/gnark/test/unsafekzg"
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

	circuit := Circuit{
		bits: 32,
	}
	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
	if err != nil {
		fmt.Println("circuit compilation error")
	}

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

	pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)
	//_, err := plonk.Setup(r1cs, kate, &publicWitness)
	if err != nil {
		log.Fatal(err)
	}

	assignment := Circuit{
		Vals: 665115184,
		bits: 32,
	}
	witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	if err != nil {
		log.Fatalf("failed to create witness: %v", err)
	}

	publicWitness, err := witness.Public()
	if err != nil {
		log.Fatalf("failed to extract public witness: %v", err)
	}

	proof, err := plonk.Prove(ccs, pk, witness)
	if err != nil {
		log.Fatal(err)
	}

	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	_ = enc.Encode(proof)
	proofSize := buf.Len()
	fmt.Printf("Proof size: %d bytes\n", proofSize)

	err = plonk.Verify(proof, vk, publicWitness)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Proof verified successfully")
}
