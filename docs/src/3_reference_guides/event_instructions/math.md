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

## Bitwise operations
> [!NOTE]
> The bitwise operation instructions might be bugged.
>
> They seem to load two values, convert them to 32bit integers, and then store the result without converting it back into a 32bit float. Though this has not yet been confirmed.

### BitAnd (0x1B)
### BitOr (0x1C)
### BitXor (0x1D)

## Other

### RandomNum (0x28)
`RandomNum` generates a random integer in the range from 0 up to but not including a given upper bound.

Ex. with an upper bound of 5, `RandomNum` will pick a number from: 0, 1, 2, 3, 4

```
# Pick a random integer in the range [0, 9]
SetU32      Pool_1 0.0 Const 10.0
RandomNum
```

#### Inputs
* **upper bound** (`Pool_1[0]`) - Upper bound of the range of numbers to randomly generate

#### Outputs
* **number** (`Pool_1[0]`) - Generated random number in range