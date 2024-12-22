用每个库分别测了三个程序cubic_expression，range_proof，sha256

1. **delak/bulletproofs**
   - 这个就测了range_proof，代码在tests目录下的range_proof.rs

2. **gnark**
   - 这个三个都测了，用了groth16和plonk，代码在gnarklab下六个目录的test.go

3. **halo2**
   - 这个三个都测了，代码在halo2/halo2_proofs/examples的cubic_expression，range_proof，sha256同名rust文件

4. **libiop**
   - 这个三个都测了，代码在libiop/libiop/tests/snark的test_aurora_snark.cpp，test_fractal_snark.cpp，test_ligero_snark.cpp

5. **libsnark**
   - 这个三个都测了，代码在libsnark-playground/src/的cubic_expression，range_proof，sha256同名cpp文件

6. **plonky2**
   - 这个三个都测了，代码在plonky2/plonky2/examples/cubic_expresstion.rs和range_proof.rs
     和plonky2-sha256/src/main.rs

7. **snarkjs**
   - 这个三个都测了，代码在snarkjs/test/circuit
     目录下的9个子目录（3个不同zkp。3*3）的cubic_expression，range_proof，sha256同名circom文件。

8. **arkworks（snarklab）**
   - 这个三个都测了，代码在snarklab/groth16/src/cubic_expression.rs和rangeproof.rs，sha256的找不到了，等我重做

9. **Spartan**
   - 这个三个都测了，代码在Spartan/examples的cubic_expression，range_proof，sha256同名rust文件

智文可以先去注释所有非rust的代码，libiop库可以不管。其余的等我重新整理之后再注释。

dockerfile我会持续往里面加，现在gnark搞好了。
