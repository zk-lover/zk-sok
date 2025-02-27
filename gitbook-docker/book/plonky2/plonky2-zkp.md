# `plonky2` ZKP Documentation
In this section, we introduce the relevant APIs of `plonky2`'s proof system, including prove and verify.

# 1. data.prove(pw)
- **Functionality**: Generates a zero-knowledge proof using circuit data and a witness.
- **Input**:
  - `pw`: Type `PartialWitness`, containing concrete values for all target variables in the circuit.
  - `data`: Type `CircuitData`, implicitly used as circuit data, including constraints and operations information.
- **Output**: A zero-knowledge proof object of type `Proof`.

# 2. data.verify(proof)
- **Functionality**: Verifies whether a zero-knowledge proof is valid.
- **Input**:
  - `proof`: Type `Proof`, the proof object generated by `data.prove`.
- **Output**: A `Result<()>`, indicating whether the verification was successful.