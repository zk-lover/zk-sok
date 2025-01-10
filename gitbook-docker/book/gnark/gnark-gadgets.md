# `gnark` Gadget Documentation
`gnark` provides many commonly used gadgets in the gnark/std directory, which can be used directly by developers to build circuits for their own applications, including signatures, hashes, polynomial extensions, interpolation, sumcheck protocol, range checks, etc. For example:

- The hashes directory provides frontend.APIs related to mimc, sha2, sha3, etc.;
- The signature directory provides frontend.APIs related to ecdsa, eddsa, etc.;
- The polynomial directory provides frontend.APIs related to polynomial extensions, polynomial interpolation, etc.;
- The sumcheck directory provides frontend.APIs related to sumcheck protocol;
- The rangecheck directory provides frontend.APIs related to range checks;
  
In addition, gnark also provides some commonly used cryptographic primitives in the gnark/crypto directory, focusing on providing encryption and mathematical operation support for gnark. Especially in terms of elliptic curve operations, gnark provides rich support, including: BN, BLS12, BLS24 and BW6 curves, etc.

The following introduces some specific gadgets and provides examples:

## 1. Range check

```
func (c *Circuit) Define(api frontend.API) error {
    r := rangecheck.New(api)
    r.Check(c.Vals, c.bits)
    ...
}
```

## 2. sha256

```
func (c *Circuit) Define(api frontend.API) error {
h, err := sha2.New(api)
    ...
    h.Write(c.In)
    res := h.Sum()
    ...
}
```

## 3. eddsa

```
type eddsaCircuit struct {
    PublicKey eddsa.PublicKey `gnark:",public"`
    Signature eddsa.Signature `gnark:",public"`
    Message frontend.Variable `gnark:",public"` } ... 
    privateKey, err := eddsa.New(twistededwards.BN254, crand.Reader) 
    publicKey := privateKey.Public() ... 
```
