# Instructions
**Instructions** are the basic building blocks of event scripts. During a cutscene or overworld segment, the game will run an event script by executing the instructions in that script one-by-one in order.

```
Sleep        Const 60.0
SetU32       Pool_1 0.0 Const 42.0
LoadMap     
LoadPos      "demo001.pos"
```

## Parts of an instruction
Every instruction consists of the following parts:

| Part | Example | Description |
|------|---------|-------------|
| Name | `Sleep` | Determines the type of instruction, and thus what it does (wait, show dialog, move the camera, etc.) |
| Arguments | `Const 60.0` | Additional information the instruction needs to perform its action (how long to wait, where to move the camera to, etc.)<br/><br/>Instructions can have 0 or more arguments. |

As an example, the following instruction has the name `Sleep` and the arguments `Const` and `60.0`.

```
Sleep        Const 60.0
```

This particular instruction forces the game to wait 60 frames (1 second) before executing the next instruction.

For a full list of the different types of instructions with descriptions and argument breakdowns, see the [list of event instructions](../../3_reference_guides/list_of_event_instructions.md).