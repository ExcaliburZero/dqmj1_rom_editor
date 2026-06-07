# Math

## Arithmetic
### Add (0x16)
`Add` sums together two float values and stores the result in a value pool.

```
# Pool_1[0] = 4 + 5 = 9
Add Pool_1 0.0 Const 4.0 Const 5.0

# Pool_1[1] = Pool_1[0] + 3 = 12
Add Pool_1 1.0 Pool_1 0.0 Const 3.0
```

#### Arguments
* **destination** (`Pool` + `Value`) - Value pool and slot to store the result in
* **a** (`Pool` + `Value`) - First operand of the addition
* **b** (`Pool` + `Value`) - Second operand of the addition

#### Outputs
* **destination** (`destination argument`) - Result of the addition

### Subtract (0x17)
`Subtract` subtracts two float values and stores the result in a value pool.

```
# Pool_1[0] = 5 - 4 = 1
Subtract Pool_1 0.0 Const 5.0 Const 4.0

# Pool_1[1] = Pool_1[0] - 2 = -1
Subtract Pool_1 1.0 Pool_1 0.0 Const 2.0
```

#### Arguments
* **destination** (`Pool` + `Value`) - Value pool and slot to store the result in
* **a** (`Pool` + `Value`) - Value to subtract from
* **b** (`Pool` + `Value`) - Value to subtract

#### Outputs
* **destination** (`destination argument`) - Result of the subtraction

### Multiply (0x18)
`Multiply` multiplies two float values and stores the result in a value pool.

```
# Pool_1[0] = 5 * 4 = 20
Multiply Pool_1 0.0 Const 5.0 Const 4.0

# Pool_1[1] = Pool_1[0] * 2 = 40
Multiply Pool_1 1.0 Pool_1 0.0 Const 2.0
```

#### Arguments
* **destination** (`Pool` + `Value`) - Value pool and slot to store the result in
* **a** (`Pool` + `Value`) - First operand of the multiplication
* **b** (`Pool` + `Value`) - Second operand of the multiplication

#### Outputs
* **destination** (`destination argument`) - Result of the multiplication

### Divide (0x19)
`Divide` divides two float values and stores the result in a value pool.

```
# Pool_1[0] = 80 / 4 = 20
Divide Pool_1 0.0 Const 80.0 Const 4.0

# Pool_1[1] = Pool_1[0] / 2 = 10
Divide Pool_1 1.0 Pool_1 0.0 Const 2.0
```

#### Arguments
* **destination** (`Pool` + `Value`) - Value pool and slot to store the result in
* **a** (`Pool` + `Value`) - Value to divide
* **b** (`Pool` + `Value`) - Value to use as the divisor

#### Outputs
* **destination** (`destination argument`) - Result of the division

### Modulo (0x1A)
`Modulo` divides two float values and stores the remainder of that division in a value pool.

```
# Pool_1[0] = 6 % 4 = 2
Modulo Pool_1 0.0 Const 6.0 Const 4.0

# Pool_1[1] = Pool_1[0] % 2 = 0
Modulo Pool_1 1.0 Pool_1 0.0 Const 2.0
```

#### Arguments
* **destination** (`Pool` + `Value`) - Value pool and slot to store the remainder in
* **a** (`Pool` + `Value`) - Value to divide
* **b** (`Pool` + `Value`) - Value to use as the divisor

#### Outputs
* **destination** (`destination argument`) - Result of the modulo operation

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