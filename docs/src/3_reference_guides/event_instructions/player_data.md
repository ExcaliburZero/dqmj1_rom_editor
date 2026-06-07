# Player data

## Player data

### GetNumPartyMonsters (0x55)
`GetNumPartyMonsters` gets the number of monsters the player has in their party.

```
GetNumPartyMonsters
```

#### Outputs
* **num_monsters** (`Pool_1[0]`) - Number of monsters the player has in their party

### GetMonsterSpeciesId (0x56)
### GetItemCount (0x57)
### GivePlayerOneItem (0x58)
### TakePlayerOneItem (0x59)
### GetNumDarkoniumX5 (0x5A)
### IncreaseNumDarkoniumX5 (0x5B)

### GetPlayerGold (0x5D)
`GetPlayerGold` gets the amount of gold the player has.

```
GetPlayerGold
```

#### Outputs
* **gold** (`Pool_1[0]`) - Amount of gold the player has

### GivePlayerGold (0x5E)
`GivePlayerGold` gives the player gold and then returns the amount of gold they have.

The gold is added to the player's current gold. For example, if the player currently has 150 gold then giving them 500 gold will result in them having 650 gold.

```
# Give the player 500 gold
SetU32       Pool_1 0.0 Const 500.0
GivePlayerGold
```

#### Inputs
* **gold** (`Pool_1[0]`) - Amount of gold to give the player

#### Outputs
* **new_gold** (`Pool_1[0]`) - Amount of gold the player has after giving them the additional gold

### HealPlayerMonsters (0x65)
### PlayerHasRoomInHand (0xB5)
