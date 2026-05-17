# Jumps and conditionals
For more information on jumps and conditionals, see the [jumps and conditionals](../../2_topic_guides/event_scripts/jumps_and_conditionals.md) topic guide.

## Jumps
### Jump (0x0C)
`Jump` moves the event to the specified instruction location instead of moving to the next instruction.

```
loop_start:
    SetDialog    "[0xEA]I am going to tell you this infinitely many times"
    ShowDialog  
    Jump loop_start
```
#### Arguments
* **destination** (`InstructionLocation`) - Instruction location to move the event to

### JumpIfTrue (0x0D)
`JumpIfTrue` moves the event to the specified instruction location if the previous conditional was true, else moves the event to the next instruction.

```
loop_start:
    SetDialog    "[0xEA]Do you want me to say this again?"
    PromptYesNo
    FloatsEq Pool_1 0.0 Const 1.0
    JumpIfTrue loop_start
    EndDialog 
```
#### Arguments
* **destination** (`InstructionLocation`) - Instruction location to move the event to if the conditional is true

## Conditionals