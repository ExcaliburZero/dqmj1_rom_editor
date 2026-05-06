# Value pools
**Value pools** are the way that event scripts store and keep track of data. [Instructions](instructions.md) read data from value pools as input and store data into value pools as output.

Each pool holds a numbered list of values that can be written to, or read from.

We will refer to entries in a pool by the pool name followed by the entry number in square brackets (ex. entry `0` of `Pool_1` = `Pool_1[0]`).

```
SetU32       Pool_1 0.0 Const 2.0   # Store the value 2.0 into Pool_1[0]
SetU32       Pool_1 1.0 Const 1.0   # Store the value 1.0 into Pool_1[1]

GetItemCount                        # Checks how many strong medicines (2.0)
                                    # the player has in their bag (1.0)
                                    # and store that number in Pool_1[0]

GivePlayerGold                      # Give the player 1 gold for each strong
                                    # medicine in their bag
```

## Example
Let's explore the example shown above, which gives the player 1 gold for each strong medicine they have in their bag.

### As input
Notice that the `GetItemCount` instruction does not take any [arguments](instructions.md#parts-of-an-instruction). But it needs some way to know what type of item we want to count, as well as whether we want to look in the player's hand, bag, or both.

The `GetItemCount` instruction takes two inputs, an `item_id` and an `item_location`. It reads the `item_id` from `Pool_1[0]`, and it reads `item_location` from `Pool_1[1]`.

So in order to have `GetItemCount` count the number of strong medicines the player has in their bag, we will need to:

* Store `2.0` in `Pool_1[0]`, as `2` is the [item id for strong medicine](https://datacrystal.tcrf.net/wiki/Dragon_Quest_Monsters:_Joker/Notes#Item_IDs)
* Store `1.0` in `Pool_1[1]`, as `1` is a special value for `GetItemCount` indicating to look only in the player's bag

To store a value within a pool, we can use the `SetU32` instruction. It takes 4 arguments:

| Argument | Purpose |
|----------|---------|
| 1st | Pool to store the value in |
| 2nd | Entry within that pool to store the value in |
| 3rd | `Const` |
| 4th | Value to store |

```admonish note
`SetU32` can also be used to move a value from one pool entry to another. In that case the 3rd argument is the pool to read the value from, and the 4th argument is the entry in that pool to read from.
```

The two `SetU32` instructions below will store `2.0` into `Pool_1[0]` and `1.0` into `Pool_1[1]`.

```
SetU32       Pool_1 0.0 Const 2.0   # Store the value 2.0 into Pool_1[0]
SetU32       Pool_1 1.0 Const 1.0   # Store the value 1.0 into Pool_1[1]
```

The `GetItemCount` instruction will then count the number of strong medicines that the player has in their bag. Once it has executed, the count is stored into `Pool_1[0]`, which the next instruction can then read.

### As output
We want the `GivePlayerGold` instruction to give the player 1 gold for each strong medicine in their bag, but we don't know how many strong medicines the player will have in their bag until the script is actually running. So we can't just use `SetU32` to specify a fixed amount.

`GetItemCount` stores its result into `Pool_1[0]` after it executes, and `GivePlayerGold` reads the amount of gold to give from `Pool_1[0]`. So by placing `GivePlayerGold` directly after `GetItemCount`, it will give the player gold equal to the count that was calculated.

## Supported pools
| Name | Main purpose |
|------|--------------|
| `Pool_0` | Storing data for later use. |
| `Pool_1` | Passing data as input to instructions. Instructions also store their output in this pool. |
| `Pool_3` | Storing data for later use. (unclear if this has a special purpose) |

```admonish note
There is no `Pool_2`, as the internal representation for event scripts uses `2` for the pool to represent `Const` rather than an actual pool.
```