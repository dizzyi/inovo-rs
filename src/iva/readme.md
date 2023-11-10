# IVA
`IVA` is a general communication protocal for inovo robot arm, which allow text-based communication.

`IVA` allow program treat the **robot as a device**, and control robot with relatively small amount of commands.

## Instructions
there are 7 type of instructions in this protocal.

### Overview
```rust
EXECUTE , MOTION, L/LR/J/JR, T/J, f64, f64, f64, f64, f64, f64
           PARAM, f64, f64, f64, f64, f64, f64
           SLEEP, f64
            SYNC,

ENQUEUE , MOTION, L/LR/J/JR, T/J, f64, f64, f64, f64, f64, f64
           PARAM, f64, f64, f64, f64, f64, f64
           SLEEP, f64
            SYNC,
DEQUEUE,

GRIPPER, ACTIVATE
              GET,
              SET, String

DIGITAL, BECHKOFF, u8,  INPUT,
            WRIST,     OUTPUT, HIGH/LOW

CURRENT, FRAME,
         JOINT,
         
CUSTOM , [String...]

```

### `EXECUTE`
```
EXECUTE, {RobotCommand...}
```
the excute instructions instruct the robot to excute the command immediately.

### `ENQUEUE`
```
ENQUEUE, {RobotCommand...}
```
the enqueue instructions instruct the robot to enqueue a certain command which allow continuous movement for the robot, since read/write to socket will break the movement blending.

#### `RobotCommand`
a robot command is something related to the robot's movement, there are 4 type
```
MOTION, {MotionType, Pose}
PARAM, {MotionParam}
SLEEP, f64,
SYNC,
```
- `MOTION` command, command the robot to move
- `PARAM` command, set the motion param
- `SLEEP` command, make the robot sleep for certain seconds
- `SYNC` command, synchronize the movement.

#### `MotionType`
The interpolation of movement
- `L` Linear motion
- `LR` Linear Relative motion
- `J` Joint motion
- `JR` Joint Relative motion

#### `Pose`
Either a transfrom or joint position.
Transform
```rust
f64, f64, f64 ,f64, f64, f64
| R3 position | euler angle |
```
Joint
```rust
f64, f64, f64 ,f64, f64, f64
|   joint angle position   |
```

#### `MotionParam`
```rust
f64, f64, f64, f64, f64, f64
|____|____|____|____|____|___> speed
     |____|____|____|____|___> accel
          |____|____|____|___> blend_linear
               |____|____|___> blend_angular
                    |____|___> tcp_speed_linear
                         |___> tcp_speed_angular
```

### `DEQUEUE`
```rust
DEQUEUE
```
the dequeue instructions instruct the robot to dequeue all enqueued commands.

### `GRIPPER`
```rust
GRIPPER
```
the gripper instructions instruct the robot to perform gripper related command.

- `ACTIVATE` command, activate the gripper
- `GET` command, get the curent gripper's position
- `SET` command, set the gripper's position to a certain label

### `DIGITAL`
```rust
DIGIRAL
```
the digital instructions instruct the robot to perform digital io related command.

#### source
- `BECKHOFF`, specify the source as PSU's IO
- `WRIST`, specify the source as wrist's IO

#### type
- `INPUT` command, get the current Input IO state
- `OUTPUT` command, set the Output IO state.

### `CURRENT`
```rust
CURRENT
```
the current instructions instruct the robot to get the current state of the robot, i.e. joint position or current frame

### `CUSTOM`
the custom instructions instruct the robot to perform custom command, allowing custom addition to the protocal

## Example 
a few example of `IVA`
```
   EXECUTE,    MOTION,         L, TRANSFORM
   EXECUTE,    MOTION,        JR,     JOINT,   3.14159,   0.00000,   0.00000,   0.00000,   0.00000,   0.00000
   ENQUEUE,     SLEEP,    12.000
   ENQUEUE,      SYNC
   ENQUEUE,     PARAM,   0.50000,   0.50000,   0.00001,   0.00017,   1.00000,  12.56637
   DEQUEUE
   DIGITAL,  BECKHOFF,         1,     INPUT
   DIGITAL,     WRIST,         1,    OUTPUT,      HIGH
   GRIPPER,  ACTIVATE
   GRIPPER,       SET,      OPEN
   CURRENT,     FRAME
```