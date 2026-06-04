# Math

## Arithmetic
### Add (0x16)
`Add` sums together two values and stores the result in a value pool.

```
# Pool_1[0] = 4 + 5
Add Pool_1 0.0 Const 4.0 Const 5.0

# Pool_1[1] = Pool_1[0] + 3
Add Pool_1 1.0 Pool_1 0.0 Const 3.0
```

#### Arguments
* **destination** (`Pool` + `Value`) - Value pool and slot to store the result in
* **a** (`Pool` + `Value`) - First operand of the addition
* **b** (`Pool` + `Value`) - Second operand of the addition

### Subtract (0x17)
`Subtract` subtracts two values and stores the result in a value pool.

```
# Pool_1[0] = 5 - 4
Subtract Pool_1 0.0 Const 5.0 Const 4.0

# Pool_1[1] = Pool_1[0] - 2
Subtract Pool_1 1.0 Pool_1 0.0 Const 2.0
```

#### Arguments
* **destination** (`Pool` + `Value`) - Value pool and slot to store the result in
* **a** (`Pool` + `Value`) - First operand of the subtraction
* **b** (`Pool` + `Value`) - Second operand of the subtraction

### Multiply (0x18)
`Multiply` multiplies two values and stores the result in a value pool.

```
# Pool_1[0] = 5 * 4
Multiply Pool_1 0.0 Const 5.0 Const 4.0

# Pool_1[1] = Pool_1[0] * 2
Multiply Pool_1 1.0 Pool_1 0.0 Const 2.0
```

#### Arguments
* **destination** (`Pool` + `Value`) - Value pool and slot to store the result in
* **a** (`Pool` + `Value`) - First operand of the multiplication
* **b** (`Pool` + `Value`) - Second operand of the multiplication

### Divide (0x19)
`Divide` divides two values and stores the result in a value pool.

```
# Pool_1[0] = 80 / 4
Divide Pool_1 0.0 Const 80.0 Const 4.0

# Pool_1[1] = Pool_1[0] / 2
Divide Pool_1 1.0 Pool_1 0.0 Const 2.0
```

#### Arguments
* **destination** (`Pool` + `Value`) - Value pool and slot to store the result in
* **a** (`Pool` + `Value`) - First operand of the division
* **b** (`Pool` + `Value`) - Second operand of the division 

### Modulo (0x1A)
`Modulo` divides two values and stores the remainder of that division in a value pool.

```
# Pool_1[0] = 6 % 4 = 2
Modulo Pool_1 0.0 Const 6.0 Const 4.0

# Pool_1[1] = Pool_1[0] % 2 = 0
Modulo Pool_1 1.0 Pool_1 0.0 Const 2.0
```

#### Arguments
* **destination** (`Pool` + `Value`) - Value pool and slot to store the remainder in
* **a** (`Pool` + `Value`) - First operand of the division
* **b** (`Pool` + `Value`) - Second operand of the division 

## Bit-wise operations

### BitAnd (0x1B)
### BitOr (0x1C)
### BitXor (0x1D)

## Other

### RandomNum (0x28)