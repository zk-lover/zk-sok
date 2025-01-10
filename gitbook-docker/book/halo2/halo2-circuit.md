# HALO2 Circuit Documentation

## 1. AssignedCell

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

## 2.  Region

**Purpose:**
Represents a region in the circuit, responsible for cell allocation and constraints.

**Components:**
- `region`: Dynamically borrowed `RegionLayouter`.

**Related Functions:**
1. `assign_advice` 
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
   - **Input:**
     - `left`: Left cell.
     - `right`: Right cell.
   - **Output:** `Result<(), Error>`.
   - **Purpose:** Constrains the values of two cells to be equal.
   - **Example Code:**
     ```rust
     region.constrain_equal(cell1, cell2)?;
     ```

## 3.  Cell

**Purpose:**
Represents a cell in the circuit, including its region, row offset, and column information.

**Components:**
- `region_index`: The index of the region the cell belongs to, type `RegionIndex`.
- `row_offset`: The row offset of the cell within the region, type `usize`.
- `column`: The column the cell belongs to, type `Column<Any>`.

**Related Functions:** None directly, mainly used as a location reference.

## 4.  Layouter

**Purpose:**
Provides an abstraction for circuit layout strategies, implementing cell allocation, region management, and other functionalities.

**Components:**
Defined as a trait, specific types need to implement its s.

**Related Functions:**
1. `assign_region` 
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

## 5.  Table

**Purpose:**
Represents a lookup table in the circuit, used for assigning fixed values.

**Components:**
- `table`: Dynamically borrowed `TableLayouter`.

**Related Functions:**
1. `assign_cell` 
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

## 6.  Chip (Trait)

**Purpose:**
`Chip` is an abstract interface for circuit components. Each `Chip` implements a set of specific operations used during circuit synthesis.

**Components:**
- `Config`: Associated type, representing the `Chip` configuration, used to store static information during circuit synthesis.
- `Loaded`: Associated type, representing the state that needs to be loaded before circuit synthesis.

**Related Functions:**
1. `config` 
   - **Input:** None
   - **Output:** `&Self::Config`, returns the `Chip` configuration.
   - **Purpose:** Retrieves the current `Chip` configuration.
   - **Example Code:**
     ```rust
     let config = chip.config();
     ```

2. `loaded` 
   - **Input:** None
   - **Output:** `&Self::Loaded`, returns the loaded state.
   - **Purpose:** Accesses the global state loaded at the beginning of circuit synthesis.
   - **Example Code:**
     ```rust
     let loaded = chip.loaded();
     ```