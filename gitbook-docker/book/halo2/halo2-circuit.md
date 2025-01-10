# `halo2` Circuit Documentation

halo2 is a highly customizable framework for zero-knowledge proof systems, focusing on scalability and flexibility. Its modular design is built around Chips, which encapsulate circuit logic, and Layouters, which manage the placement and wiring of these Chips. Developers define constraints and logic through Custom Gates and lookup tables, enabling efficient implementation of advanced cryptographic protocols. With its support for recursive proofs and minimal trusted setup requirements, halo2 is particularly well-suited for next-generation privacy-preserving and blockchain applications.

The following sections describe the key structures and functions used in the `halo2` framework for constructing circuits in zero-knowledge proofs. These components are essential for defining, organizing, and verifying constraints in a flexible and scalable manner, enabling the development of efficient and secure proof systems.

## 1.  Chip (Trait)

**Purpose:**
`Chip` is an abstract interface for circuit components. Each `Chip` implements a set of specific operations used during circuit synthesis.

**Components:**
- `Config`: Associated type, representing the `Chip` configuration, used to store static information during circuit synthesis.
- `Loaded`: Associated type, representing the state that needs to be loaded before circuit synthesis.

**Related Functions:**
1. `config` 
  ```
  fn config(&self) -> &Self::Config;
  ```
   - **Input:** None
   - **Output:** `&Self::Config`, returns the `Chip` configuration.
   - **Purpose:** Retrieves the current `Chip` configuration.
   - **Example Code:**
     ```rust
     let config = chip.config();
     ```

2. `loaded` 
  ```
  fn loaded(&self) -> &Self::Loaded;
  ```
   - **Input:** None
   - **Output:** `&Self::Loaded`, returns the loaded state.
   - **Purpose:** Accesses the global state loaded at the beginning of circuit synthesis.
   - **Example Code:**
     ```rust
     let loaded = chip.loaded();
     ```

## 2. Circuit (Trait)

**Purpose:**
The `Circuit` trait is an abstract interface for circuit implementations. It defines how to configure the circuit (column layout, constraints, etc.) and how to generate the circuit during the synthesis phase.

**Components:**
- `Config`: An associated type used to store column and constraint information.
- `FloorPlanner`: An associated type that defines the layout strategy of the circuit.
- `without_witnesses` method: Used to generate a copy of the circuit without witness values.
- `configure` method: Configures the columns and constraints of the circuit.
- `synthesize` method: Generates the circuit during the synthesis phase, including column allocation and value assignment.

**Related Functions:**
1. `without_witnesses` 
  ```
  fn without_witnesses(&self) -> Self;
  ```
   - **Input:** None
   - **Output:** Returns a copy of the circuit with all witness values set to `None`.
   - **Purpose:** Generates a circuit without specific values, used to verify the circuit structure.
   - **Example Code:**
     ```rust
     let circuit_without_witnesses = circuit.without_witnesses();
     ```

2. `configure` 
   ```
   fn configure(meta: &mut ConstraintSystem<F>) -> Self::Con
   ```
   - **Input:** `meta: &mut ConstraintSystem<F>`
   - **Output:** Returns `Self::Config`, containing the configuration of columns and constraints.
   - **Purpose:** Defines the column layout and constraint conditions of the circuit.
   - **Example Code:**
     ```rust
     impl<F: Field> Circuit<F> for MyCircuit {
         type Config = MyCircuitConfig;
         type FloorPlanner = SimpleFloorPlanner;

         fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
             let advice = meta.advice_column();
             let fixed = meta.fixed_column();
             meta.enable_equality(advice);

             MyCircuitConfig { advice, fixed }
         }
     }
     ```

3. `synthesize` 
  ```
  fn synthesize(&self, config: Self::Config, layouter: impl Layouter<F>) -> Result<(), Error>;
  ```
   - **Input:**
     - `config`: Configuration, type `Self::Config`.
     - `layouter`: Layouter, type `impl Layouter<F>`.
   - **Output:** `Result<(), Error>`, returns possible errors.
   - **Purpose:** Completes the synthesis of the circuit using the layouter based on the configuration.
   - **Example Code:**
     ```rust
     impl<F: Field> Circuit<F> for MyCircuit {
         fn synthesize(
             &self,
             config: Self::Config,
             mut layouter: impl Layouter<F>,
         ) -> Result<(), Error> {
             layouter.assign_region(
                 || "Assign values",
                 |mut region| {
                     region.assign_advice(
                         || "a",
                         config.advice,
                         0,
                         || Value::known(F::from(5)),
                     )?;
                     Ok(())
                 },
             )
         }
     }
     ```
## 3. AssignedCell

**Purpose:**
Represents an assigned circuit cell, including its value and location information.

**Components:**
- `value`: Stores the cell's value, type `Value<V>`.
- `cell`: The cell's location information, type `Cell`.
- `_marker`: `PhantomData`, used to carry the type information of the generic `F`.

**Related Functions:**
1. `value` 
   - **Input:** None
   - **Output:** Returns the cell's value, type `Value<&V>`.
   - **Purpose:** Retrieves the current cell's value.
   - **Example Code:**
     ```rust
     let value = assigned_cell.value();
     ```

2. `cell` 
   - **Input:** None
   - **Output:** Returns the cell's location information, type `Cell`.
   - **Purpose:** Retrieves the current cell's location information.
   - **Example Code:**
     ```rust
     let cell = assigned_cell.cell();
     ```

3. `copy_advice` 
  ```
  pub fn copy_advice<A, AR>(
        &self,
        annotation: A,
        region: &mut Region<'_, F>,
        column: Column<Advice>,
        offset: usize,
    ) -> Result<Self, Error>
    where
        A: Fn() -> AR,
        AR: Into<String>
  ```
   - **Input:**
     - `annotation`: Description information.
     - `region`: Target region.
     - `column`: Target column.
     - `offset`: Offset.
   - **Output:** Returns a new `AssignedCell`, representing the cell copied to the new location.
   - **Purpose:** Copies the current cell's value to a specified advice column cell in the target region and constrains them to be equal.
   - **Example Code:**
     ```rust
     let new_cell = assigned_cell.copy_advice(
         || "Copy cell",
         &mut region,
         column,
         offset
     )?;
     ```

## 4.  Region

**Purpose:**
Represents a region in the circuit, responsible for cell allocation and constraints.

**Components:**
- `region`: Dynamically borrowed `RegionLayouter`.

**Related Functions:**
1. `assign_advice` 
  ```
  pub fn assign_advice<'v, V, VR, A, AR>(
        &'v mut self,
        annotation: A,
        column: Column<Advice>,
        offset: usize,
        mut to: V,
    ) -> Result<AssignedCell<VR, F>, Error>
  ```
   - **Input:**
     - `annotation`: Description information.
     - `column`: Advice column.
     - `offset`: Offset.
     - `to`: Value assignment closure.
   - **Output:** Returns `AssignedCell`, representing the assigned cell.
   - **Purpose:** Assigns an advice value at the specified column and offset.
   - **Example Code:**
     ```rust
     let assigned_cell = region.assign_advice(
         || "Assign advice",
         column,
         offset,
         || Value::known(5)
     )?;
     ```

2. `constrain_equal` 
  ```
  pub fn constrain_equal(&mut self, left: Cell, right: Cell) -> Result<(), Error>
  ```
   - **Input:**
     - `left`: Left cell.
     - `right`: Right cell.
   - **Output:** `Result<(), Error>`.
   - **Purpose:** Constrains the values of two cells to be equal.
   - **Example Code:**
     ```rust
     region.constrain_equal(cell1, cell2)?;
     ```

## 5.  Cell

**Purpose:**
Represents a cell in the circuit, including its region, row offset, and column information.

**Components:**
- `region_index`: The index of the region the cell belongs to, type `RegionIndex`.
- `row_offset`: The row offset of the cell within the region, type `usize`.
- `column`: The column the cell belongs to, type `Column<Any>`.

**Related Functions:** None directly, mainly used as a location reference.

## 6.  Layouter

**Purpose:**
Provides an abstraction for circuit layout strategies, implementing cell allocation, region management, and other functionalities.

**Components:**
Defined as a trait, specific types need to implement its s.

**Related Functions:**
1. `assign_region` 
  ```
  fn assign_region<A, AR, N, NR>(&mut self, name: N, assignment: A) -> Result<AR, Error>
  ```
   - **Input:**
     - `name`: Region name.
     - `assignment`: Closure defining the allocation logic.
   - **Output:** `Result<AR, Error>`.
   - **Purpose:** Allocates the logic for a region.
   - **Example Code:**
     ```rust
     layouter.assign_region(
         || "Example Region",
         |mut region| {
             region.assign_advice(
                 || "Assign",
                 column,
                 0,
                 || Value::known(7)
             )
         }
     )?;
     ```

## 7.  Table

**Purpose:**
Represents a lookup table in the circuit, used for assigning fixed values.

**Components:**
- `table`: Dynamically borrowed `TableLayouter`.

**Related Functions:**
1. `assign_cell` 
  ```
  pub fn assign_cell<'v, V, VR, A, AR>(
        &'v mut self,
        annotation: A,
        column: TableColumn,
        offset: usize,
        mut to: V,
    ) -> Result<(), Error>
  ```
   - **Input:**
     - `annotation`: Description information.
     - `column`: Table column.
     - `offset`: Offset.
     - `to`: Value assignment closure.
   - **Output:** `Result<(), Error>`.
   - **Purpose:** Assigns a fixed cell in the lookup table.
   - **Example Code:**
     ```rust
     table.assign_cell(
         || "Assign to table",
         column,
         offset,
         || Value::known(3)
     )?;
     ```