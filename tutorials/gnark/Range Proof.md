## Range Proof

### Range Proof Implemented Based on Groth16

This code implements a simple zero-knowledge proof (zkSNARK) process using the Groth16 protocol. It defines a circuit (`Circuit`) and compiles it into a constraint system (R1CS) using `frontend.Compile`. Then, it sets up the proving and verifying keys (`pk` and `vk`) with `groth16.Setup`. The code creates an assignment instance (`assignment`), generates a full witness, and extracts the public witness. It proceeds to generate a proof and finally verifies the proof to ensure its correctness. The entire process involves defining the circuit, compiling it, generating the proof, and verifying it.

Below, we will divide the code into code blocks and annotate them.

### 1. Circuit Construction

##### Define the circuit

```Go
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

* The `Circuit` struct defines a circuit that includes a variable `Vals` and an integer `bits`.

* The `Define` method specifies the circuit's constraints, using `rangecheck` to verify if `Vals` is within the specified bit range.

##### Main():

```Go
circuit := Circuit{
	bits: 32,
}
ccs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
if err != nil {
	log.Fatalf("failed to compile circuit: %v", err)
}
```

Create a `Circuit` instance and compile the circuit using the `frontend.Compile` function to compile the circuit into a constraint system (ccs). Here, `ecc.BN254.ScalarField()` is used as the scalar field, and `scs.NewBuilder` is used as the builder.

### 2. ZK Proof Generation and Verification

##### Use the Groth16 to set up zkSNARK

```Go
pk, vk, err := groth16.Setup(ccs)
if err != nil {
	log.Fatalf("failed to setup groth16: %v", err)
}
```

Generating the proving key `pk` and the verifying key `vk`.

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

Serialize and output the size of the proving key and verifying key.

```Go
assignment := Circuit{
	Vals: 665115184,
	bits: 32,
}
witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
if err != nil {
	log.Fatalf("failed to create witness: %v", err)
}
```

Create a `Circuit` instance as the assignment and generate the witness.

```Go
publicWitness, err := witness.Public()
if err != nil {
	log.Fatalf("failed to extract public witness: %v", err)
}
```

Extract the public witness.

```Go
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

Generate the proof, then encode the proof and output its size.

```Go
err = groth16.Verify(proof, vk, publicWitness)
if err != nil {
	log.Fatalf("failed to verify proof: %v", err)
}
fmt.Println("Proof verified successfully")
```

Verify the proof and output the result.

### Range Proof Implemented Based on Plonk

This code implements a process using the gnark library for circuit compilation, proof generation, and verification. It defines a circuit structure `Circuit` with a variable `Vals` and a bit length `bits`. In the `main` function, the circuit is compiled into a sparse R1CS format, and then SRS and SRS Lagrange forms are generated. The PLONK protocol is used to set up the proving and verification keys. An instance of the circuit assignment is created, and a corresponding proof is generated. Finally, the proof is verified for correctness, and the verification result is printed.

Below, we will divide the code into code blocks and annotate them.

### 1. Circuit Construction

The circuit structure and circuit constraints are the same as in the Groth16.

##### Main

```Go
circuit := Circuit{
    bits: 32,
}
ccs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
if err != nil {
    fmt.Println("circuit compilation error")
}
```

Compile the circuit.

```Go
scs := ccs.(*cs.SparseR1CS)
srs, srsLagrange, err := unsafekzg.NewSRS(scs)
if err != nil {
    panic(err)
}
```

### 2. ZK Proof Generation and Verification

Generate SRS

* Convert the constraint system: Convert the compiled ccs to the sparse R1CS format.

* Generate SRS: Use `unsafekzg.NewSRS` to generate the Structured Reference String (SRS) and the Lagrange form of the SRS.

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

Encode SRS: Use the `gob` encoder to encode the SRS and the SRS Lagrange form into a byte stream, then calculate and output the size of the encoded SRS and SRS Lagrange form.

```Go
pk, vk, err := plonk.Setup(ccs, srs, srsLagrange)
if err != nil {
    log.Fatal(err)
}
```

Use Plonk to set up zkSNARK, generating the proving key (pk) and the verifying key (vk).

```Go
assignment := Circuit{
    Vals: 665115184,
    bits: 32,
}
witness, err := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
if err != nil {
    log.Fatalf("failed to create witness: %v", err)
}
```

Create a `Circuit` instance as the assignment and generate the witness.

```Go
publicWitness, err := witness.Public()
if err != nil {
    log.Fatalf("failed to extract public witness: %v", err)
}
```

Extract the public witness.

```Go
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

Generate the proof, then encode the proof and output its size.

```Go
err = plonk.Verify(proof, vk, publicWitness)
if err != nil {
    log.Fatal(err)
}
fmt.Println("Proof verified successfully")
```

Verify the proof.
