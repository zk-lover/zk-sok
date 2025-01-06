### Hash Function

#### Hash Function Implemented Based on Groth16

This code implements a zero-knowledge proof system using Groth16 zkSNARKs. Specifically, it defines a circuit `Circuit` to compute the SHA-256 hash of an input string and verify if the hash matches an expected value. The main steps include compiling the circuit, setting up Groth16 parameters, creating and assigning witness values to the circuit, generating a proof, and verifying the proof. The entire process demonstrates how to construct and verify zero-knowledge proofs using the gnark library in Go.

Below, we will divide the code into code blocks and annotate them.

###### Define the circuit structure

```markup
type Circuit struct {
	In       []uints.U8
	Expected [32]uints.U8 `gnark:",public"`
}
```

###### Define the circuit constraints

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

The `Define` method specifies the circuit's constraint logic. It uses the SHA-256 hash function to compute the hash of the input and compares it with the expected value.

* Create an instance `h` of the SHA-256 hash function.

* Write the input `In` to the hash function.

* Compute the hash value `res` and check if its length is 32 bytes.

* Use the `uapi.ByteAssertEq` method to compare the computed hash value with the expected value `Expected` byte by byte.

###### Main

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

First, compile the circuit. The input string is converted into a byte array, and its SHA-256 hash value is computed.

* Convert the input string into a byte array `inbyte`.

* Compute its SHA-256 hash value `hash`.

* Create a `Circuit` instance `circuit` and assign the input byte array to `In`.

* Use `frontend.Compile` to compile the circuit, generating the constraint system `ccs`.

###### Use Groth16 to set up zkSNARK

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

* Use `groth16.Setup` to generate the proving key `pk` and the verifying key `vk`.

* Use the `gob` encoder to calculate the sizes of `pk` and `vk`, and output them.

###### Create the witness for the circuit, assigning input values and the expected hash value.

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

* Create a `Circuit` instance `assignment` and assign the input byte array to `In`.

* Copy the computed hash value to the `Expected` field.

* Use `frontend.NewWitness` to generate the complete witness `witness`.

###### Extract the public witness

```markup
publicWitness, err := witness.Public()
if err != nil {
	log.Fatalf("failed to extract public witness: %v", err)
}
```

###### Generate and verify the proof

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

* Use `groth16.Prove` to generate the proof `proof`.

* Use the `gob` encoder to calculate the size of the proof `proofSize`, and output it.

* Use `groth16.Verify` to verify the correctness of the proof.

#### Hash Function Implemented Based on Plonk

This code implements a circuit verification process using the PLONK zero-knowledge proof system. It defines a circuit `Circuit` to compute the SHA-256 hash of an input byte array and compares it with an expected hash value. The code compiles the circuit to generate a constraint system and uses the `unsafekzg` library to generate a structured reference string (SRS). It then sets up the PLONK parameters, creates an instance of the circuit with assigned values, and generates a full witness. The code extracts the public witness, generates a proof, and verifies its validity. Finally, it outputs the size of the proof and confirms successful verification, demonstrating the complete process of circuit compilation, proof generation, and verification using the gnark library.

Below, we will divide the code into code blocks and annotate them.

The circuit structure and circuit constraints are the same as in the Groth16 scheme.

##### Main

```markup
input := "xxx"
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

First, compile the circuit. Convert the input string into a byte array and compute its SHA-256 hash value.

Convert the input string "hudaqi" into a byte array `inbyte`.

* Compute its SHA-256 hash value `hash`.

* Create a `Circuit` instance `circuit` and assign the input byte array to `In`.

* Use `frontend.Compile` to compile the circuit, generating the constraint system `ccs`.

###### Generate the SRS (Structured Reference String) and calculate its size.

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

* Convert the constraint system `ccs` to the sparse R1CS format `scs`.

* Use `unsafekzg.NewSRS` to generate the Structured Reference String `SRS` and `SRS Lagrange`.

* Use the `gob` encoder to calculate the sizes of `SRS` and `SRS Lagrange`.

###### Set up the parameters for zkSNARK

```markup
pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)
if err != nil {
	log.Fatal(err)
}
```

* Use `plonk.Setup` to generate the proving key `pk` and the verifying key `vk`.

###### Create the circuit assignment

```markup
assignment := Circuit{
	In: uints.NewU8Array(inbyte),
}
copy(assignment.Expected[:], uints.NewU8Array(hash[:]))
```

* Create a `Circuit` instance `assignment` and assign the input byte array to `In`.

* Copy the computed hash value to the `Expected` field.

###### Generate the witness and extract the public witness

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

* Use `frontend.NewWitness` to generate the complete witness `witness`, which is used to prove the correctness of the circuit.

* Extract the public witness `publicWitness` from the complete witness.

###### Generate and verify the proof

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

* Use `plonk.Prove` to generate the proof `proof`.

* Use `plonk.Verify` to verify the correctness of the proof.

* Use the `gob` encoder to calculate the size of the proof `proofSize`.

* Output the proof size and confirm the verification success.

