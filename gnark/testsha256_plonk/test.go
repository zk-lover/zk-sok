package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/gob"
	"fmt"
	"log"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	cs "github.com/consensys/gnark/constraint/bn254"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/consensys/gnark/std/hash/sha2"
	"github.com/consensys/gnark/std/math/uints"
	"github.com/consensys/gnark/test/unsafekzg"
)

type Circuit struct {
	In       []uints.U8
	Expected [32]uints.U8 `gnark:",public"`
}

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

func main() {

	// Step 1: Compile the circuit
	input := "I love zk-sok"
	inbyte := []byte(input)
	hash := sha256.Sum256(inbyte)
	circuit := Circuit{
		In: uints.NewU8Array(inbyte),
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

	// Step 2: Groth16 zkSNARK: Setup
	pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)
	//_, err := plonk.Setup(r1cs, kate, &publicWitness)
	if err != nil {
		log.Fatal(err)
	}

	assignment := Circuit{
		In: uints.NewU8Array(inbyte),
	}
	copy(assignment.Expected[:], uints.NewU8Array(hash[:]))
	// Step 3: Create a witness
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
}
