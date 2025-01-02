# 1. PedersenGens::default()
- **Input**: None
- **Output**: A `PedersenGens` type, representing the default generators used for creating Pedersen commitments.
- **Functionality**: Provides the basic parameters required for generating Pedersen commitments.

# 2. BulletproofGens::new(x, y)
- **Input**:  
  - `x`: The maximum proof range supported, in bits.  
  - `y`: The maximum proof aggregation size supported.  
- **Output**: A `BulletproofGens` type, representing the Bulletproof generators used to create range proofs.
- **Functionality**: Generates the public parameters for Bulletproofs, suitable for single-party or multi-party range proofs.