# Other

## Battle-related

### StartBattle (0x44)
Starts a battle based on enemy encounter information. Has multiple variations on the inputs it takes.

Seems like it can only be sucessfully used in areas where battles normally occur in the game.

```
# Start a battle against a cannibox
SetU32       Pool_1 0.0 Const 0.0
SetU32       Pool_1 1.0 Const 854.0
SetU32       Pool_1 2.0 Const 0.0
SetU32       Pool_1 3.0 Const 0.0
SetU32       Pool_1 4.0 Const 1.0
SetU32       Pool_1 5.0 Const 0.0
StartBattle    
```

#### Inputs
* **type** (`Pool_1[0]`) - Type of StartBattle instruction. Determines which other inputs are read and how those values are interpreted.

##### Type 0
Starts a battle against 1-3 enemies with the specified encounter ids. Using an encounter id of `0` for an enemy causes no enemy to appear in that position.

```
SetU32       Pool_1 0.0 Const 0.0    # type = 0
SetU32       Pool_1 1.0 Const 854.0  # cannibox encounter id
SetU32       Pool_1 2.0 Const 0.0    # no enemy
SetU32       Pool_1 3.0 Const 0.0    # no enemy
SetU32       Pool_1 4.0 Const 1.0
SetU32       Pool_1 5.0 Const 0.0
StartBattle    
```

* **enemy_1** (`Pool_1[1]`) - Encounter id of first enemy
* **enemy_2** (`Pool_1[2]`) - Encounter id of second enemy
* **enemy_3** (`Pool_1[3]`) - Encounter id of third enemy
* **???** (`Pool_1[4]`) - 
* **???** (`Pool_1[5]`) - 

##### Type 1
```
SetU32       Pool_1 0.0 Const 1.0  # type = 1
SetU32       Pool_1 1.0 Const 8.0
SetU32       Pool_1 2.0 Const 0.0
SetU32       Pool_1 3.0 Const 0.0
SetU32       Pool_1 4.0 Const 0.0
SetU32       Pool_1 5.0 Const 0.0
StartBattle    
```

* **enemy_1** (`Pool_1[1]`) - Encounter id of first enemy
* **enemy_2** (`Pool_1[2]`) - Encounter id of second enemy
* **enemy_3** (`Pool_1[3]`) - Encounter id of third enemy
* **???** (`Pool_1[4]`) - 
* **???** (`Pool_1[5]`) - 

##### Type 2
Starts a battle against a scout, using that scout's monster pool.

```
SetU32       Pool_1 0.0 Const 2.0   # type = 2
SetU32       Pool_1 1.0 Pool_3 0.0
SetU32       Pool_1 2.0 Const 0.0
StartBattle    
```

* **???** (`Pool_1[1]`) - Related to which pool to pull enemies from, but not directly the id...
* **???** (`Pool_1[2]`) - 

##### Type 3
```
SetU32       Pool_1 0.0 Const 3.0    # type = 3
SetU32       Pool_1 1.0 Const 200.0
SetU32       Pool_1 2.0 Const 0.0
StartBattle    
```

* **???** (`Pool_1[1]`) - 
* **???** (`Pool_1[2]`) - 

##### Type 4

```
SetU32       Pool_1 0.0 Const 4.0    # type = 4
SetU32       Pool_1 1.0 Const 489.0
SetU32       Pool_1 2.0 Const 490.0
SetU32       Pool_1 3.0 Const 491.0
SetU32       Pool_1 4.0 Const 0.0
SetU32       Pool_1 5.0 Const 0.0
SetU32       Pool_1 6.0 Const 1.0
StartBattle    
```

* **enemy_1** (`Pool_1[1]`) - Encounter id of first enemy
* **enemy_2** (`Pool_1[2]`) - Encounter id of second enemy
* **enemy_3** (`Pool_1[3]`) - Encounter id of third enemy
* **???** (`Pool_1[4]`) - 
* **???** (`Pool_1[5]`) - 
* **???** (`Pool_1[6]`) - Seems unused in the game's code?

##### Type 5
Maybe related to wifi battles?

```
SetU32       Pool_1 0.0 Const 5.0    # type = 5
SetU32       Pool_1 1.0 Pool_0 40.0
StartBattle    
```

* **???** (`Pool_1[1]`) - 

##### Type 6
* **???** (`Pool_1[1]`) - 
* **???** (`Pool_1[2]`) - 

##### Type 7
* **enemy_1** (`Pool_1[1]`) - Encounter id of first enemy
* **enemy_2** (`Pool_1[2]`) - Encounter id of second enemy
* **enemy_3** (`Pool_1[3]`) - Encounter id of third enemy
* **???** (`Pool_1[4]`) - 
* **???** (`Pool_1[5]`) - 

##### Type 8
* **???** (`Pool_1[1]`) - 
* **???** (`Pool_1[2]`) - 

#### Outputs