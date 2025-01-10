gnark provides a concise and powerful way to build zero-knowledge proof circuits. It simplifies the circuit construction process by defining the Circuit interface and requiring developers to implement the Define method. Developers only need to describe the constraints of the circuit in the Define method, and gnark will automatically handle the rest. By implementing the Circuit interface, developers can easily add various logic and constraints to the circuit. gnark also provides developers with a rich API that covers common mathematical operations and constraints. Through these APIs, developers can define the operation logic of the circuit through simple function calls like normal programming.

In the following content, we will introduce gnark's Circuit interface and its Define method implementation in detail, as well as some commonly used APIs to help developers build and optimize zero-knowledge proof circuits more efficiently.

# 1. frontend.Circuit (from gnark/frontend/circuit.go)

**Purpose:**
Circuit is the circuit structure of gnark, responsible for the high-level definition of the circuit. It mainly contains the private input and public input variables of the circuit, and implements the Define method to constrain the circuit.
**Example:**
type MyCircuit struct {
X frontend.Variable `gnark:"-,public"`
Y frontend.Variable `gnark:"-,secret"`
}
Public input, use the `public` tag; private input, use the `secret` tag
func (c *MyCircuit) Define(api frontend.API) error {
api.AssertIsEqual(c.X, c.Y)
return nil
}
Define method, used to define the constraints of the circuit.
type Circuit interface {
Define(api API) error
}

# 2. frontend.Variable (from gnark/frontend/variable.go)

**Purpose:**
Variable represents a variable in the circuit, which is usually the object operated in frontend.API. Variable is an interface
type Variable interface{}

# 3. frontend.API (from gnark/frontend/api.go)

**Purpose:**
frontend.API is an interface for defining circuit operations in gnark. All circuit calculations (such as addition, multiplication, comparison, etc.) are performed through the methods provided by frontend.API. It contains a set of methods for performing variable operations in circuits, generally used for calculating the logic of circuit constraints in Define. Here are several important methods:

## 3.1 Add

```
Add(i1, i2 Variable, in ...Variable) Variable
```

**Purpose:**
Perform the addition operation of multiple frontend.Variable type variables and return the addition result.
**Input:**
Multiple variables of frontend.Variable type.
**Output:**
Return a new frontend.Variable type, representing the result of the addition.
**Example:**

```
result := api.Add(circuit.X, circuit.Y)
```

## 3.2 Mul

```
Mul(i1, i2 Variable, in ...Variable) Variable
```

**Purpose:**
Perform multiplication of multiple frontend.Variable type variables and return the multiplication result.
**Input:**
Multiple variables of frontend.Variable type.
**Output:**
Return a new frontend.Variable type representing the result of the multiplication.
**Example:**

```
result := api.Mul(circuit.X, circuit.Y)
```

## 3.3 AssertIsEqual

```
AssertIsEqual(i1, i2 Variable)
```

**Purpose:**
Add a constraint to assert that two variables of frontend.Variable type are equal. If they are not equal, circuit verification will fail.
**Input:**
Two variables of type frontend.Variable.
**Output:**
No return value, register the constraint directly in the circuit.
**Example:**

```
api.AssertIsEqual(circuit.Y, api.Add(x3, circuit.X, 1))
```

## 3.4 Cmp

```
Cmp(i1, i2 Variable) Variable
```

**Purpose:**
Compare two variables of type frontend.Variable and return a Boolean value indicating whether x is less than y.
**Input:**
Two variables of type frontend.Variable.
**Output:**

```
1 if i1>i2,
0 if i1=i2,
-1 if i1<i2.
```

**Example:**

```
result := api.Cmp(x, y)
```

frontend.API also implements multiple methods such as Sub, Neg, Div, and, or, Xor, IsZero, AssertIsBoolean, etc., which are not introduced here one by one.

# 4. frontend.Compile (from gnark/frontend/compile.go)

```
func Compile(field *big.Int, newBuilder NewBuilder, circuit Circuit, opts ...CompileOption) (constraint.ConstraintSystem, error)
```

**Purpose:**
compile compiles the front-end description of the circuit into a low-level representation suitable for generating proofs and verification. After the circuit is compiled, it can be used in the actual proof generation and verification process.
**Parameter Description:**
field: specifies the finite field used by the circuit. `gnark` supports different elliptic curves, you can choose according to your needs.
newBuilder: a constructor function used to create a circuit builder.
circuit: a structure that implements the `frontend.Circuit` interface. This structure defines the relationship and constraints between variables in the circuit.
opts: an optional parameter used to configure compilation options.
constraint.ConstraintSystem: returns a constraint system representing the constraints of the circuit.
**Example:**

```
ccs, _ := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
```

# 5. frontend.NewWitness

```
func NewWitness(assignment Circuit, field *big.Int, opts ...WitnessOption) (witness.Witness, error)
```

**Purpose:**
The NewWitness function is used to generate a new Witness object based on a given circuit (Circuit) and a finite field (field). The Witness object is a key structure in zero-knowledge proofs, containing all the private inputs in the circuit (i.e., the data known by the prover). This Witness is used to generate zero-knowledge proofs.
**Parameter Description:**
assignment: Circuit input assignment, which can be a structure instance corresponding to the circuit, containing the specific values ​​of all inputs.
field: specifies the finite field used by the circuit. `gnark` supports multiple elliptic curves. Users need to select the corresponding curve according to the constraints of the circuit.
opts: optional parameters used to configure the options of Witness.
witness.Witness: returns a Witness object representing the complete witness of the circuit.
**Example:**

```
witness, _ := frontend.NewWitness(&circuit, ecc.BN254.ScalarField())
```
