# MovementFlags

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/common_movement.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement.wowm#L12).

```rust,ignore
flag MovementFlags : u32 {
    NONE = 0x00000000;
    FORWARD = 0x00000001;
    BACKWARD = 0x00000002;
    STRAFE_LEFT = 0x00000004;
    STRAFE_RIGHT = 0x00000008;
    TURN_LEFT = 0x00000010;
    TURN_RIGHT = 0x00000020;
    PITCH_UP = 0x00000040;
    PITCH_DOWN = 0x00000080;
    WALK_MODE = 0x00000100;
    ON_TRANSPORT = 0x00000200;
    LEVITATING = 0x00000400;
    FIXED_Z = 0x00000800;
    ROOT = 0x00001000;
    JUMPING = 0x00002000;
    FALLINGFAR = 0x00004000;
    SWIMMING = 0x00200000;
    SPLINE_ENABLED = 0x00400000;
    CAN_FLY = 0x00800000;
    FLYING = 0x01000000;
    ONTRANSPORT = 0x02000000;
    SPLINE_ELEVATION = 0x04000000;
    WATERWALKING = 0x10000000;
    SAFE_FALL = 0x20000000;
    HOVER = 0x40000000;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `FORWARD` | 1 (0x01) |  |  |
| `BACKWARD` | 2 (0x02) |  |  |
| `STRAFE_LEFT` | 4 (0x04) |  |  |
| `STRAFE_RIGHT` | 8 (0x08) |  |  |
| `TURN_LEFT` | 16 (0x10) |  |  |
| `TURN_RIGHT` | 32 (0x20) |  |  |
| `PITCH_UP` | 64 (0x40) |  |  |
| `PITCH_DOWN` | 128 (0x80) |  |  |
| `WALK_MODE` | 256 (0x100) |  |  |
| `ON_TRANSPORT` | 512 (0x200) |  |  |
| `LEVITATING` | 1024 (0x400) |  |  |
| `FIXED_Z` | 2048 (0x800) |  |  |
| `ROOT` | 4096 (0x1000) |  |  |
| `JUMPING` | 8192 (0x2000) |  |  |
| `FALLINGFAR` | 16384 (0x4000) |  |  |
| `SWIMMING` | 2097152 (0x200000) |  |  |
| `SPLINE_ENABLED` | 4194304 (0x400000) |  |  |
| `CAN_FLY` | 8388608 (0x800000) |  |  |
| `FLYING` | 16777216 (0x1000000) |  |  |
| `ONTRANSPORT` | 33554432 (0x2000000) |  |  |
| `SPLINE_ELEVATION` | 67108864 (0x4000000) |  |  |
| `WATERWALKING` | 268435456 (0x10000000) |  |  |
| `SAFE_FALL` | 536870912 (0x20000000) |  |  |
| `HOVER` | 1073741824 (0x40000000) |  |  |

Used in:
* [MovementBlock](movementblock.md)
* [MovementInfo](movementinfo.md)
# MovementFlags

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/common_movement_2_4_3.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_2_4_3.wowm#L3).

```rust,ignore
flag MovementFlags : u32 {
    NONE = 0x00000000;
    FORWARD = 0x00000001;
    BACKWARD = 0x00000002;
    STRAFE_LEFT = 0x00000004;
    STRAFE_RIGHT = 0x00000008;
    TURN_LEFT = 0x00000010;
    TURN_RIGHT = 0x00000020;
    PITCH_UP = 0x00000040;
    PITCH_DOWN = 0x00000080;
    WALK_MODE = 0x00000100;
    ON_TRANSPORT = 0x00000200;
    LEVITATING = 0x00000400;
    FIXED_Z = 0x00000800;
    ROOT = 0x00001000;
    JUMPING = 0x00002000;
    FALLINGFAR = 0x00004000;
    SWIMMING = 0x00200000;
    ASCENDING = 0x00400000;
    CAN_FLY = 0x00800000;
    FLYING = 0x01000000;
    ONTRANSPORT = 0x02000000;
    SPLINE_ELEVATION = 0x04000000;
    SPLINE_ENABLED = 0x08000000;
    WATERWALKING = 0x10000000;
    SAFE_FALL = 0x20000000;
    HOVER = 0x40000000;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `FORWARD` | 1 (0x01) |  |  |
| `BACKWARD` | 2 (0x02) |  |  |
| `STRAFE_LEFT` | 4 (0x04) |  |  |
| `STRAFE_RIGHT` | 8 (0x08) |  |  |
| `TURN_LEFT` | 16 (0x10) |  |  |
| `TURN_RIGHT` | 32 (0x20) |  |  |
| `PITCH_UP` | 64 (0x40) |  |  |
| `PITCH_DOWN` | 128 (0x80) |  |  |
| `WALK_MODE` | 256 (0x100) |  |  |
| `ON_TRANSPORT` | 512 (0x200) |  |  |
| `LEVITATING` | 1024 (0x400) |  |  |
| `FIXED_Z` | 2048 (0x800) |  |  |
| `ROOT` | 4096 (0x1000) |  |  |
| `JUMPING` | 8192 (0x2000) |  |  |
| `FALLINGFAR` | 16384 (0x4000) |  |  |
| `SWIMMING` | 2097152 (0x200000) |  |  |
| `ASCENDING` | 4194304 (0x400000) |  |  |
| `CAN_FLY` | 8388608 (0x800000) |  |  |
| `FLYING` | 16777216 (0x1000000) |  |  |
| `ONTRANSPORT` | 33554432 (0x2000000) |  |  |
| `SPLINE_ELEVATION` | 67108864 (0x4000000) |  |  |
| `SPLINE_ENABLED` | 134217728 (0x8000000) |  |  |
| `WATERWALKING` | 268435456 (0x10000000) |  |  |
| `SAFE_FALL` | 536870912 (0x20000000) |  |  |
| `HOVER` | 1073741824 (0x40000000) |  |  |

Used in:
* [MovementBlock](movementblock.md)
* [MovementInfo](movementinfo.md)
# MovementFlags

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm#L11).

```rust,ignore
flag MovementFlags : u32 {
    NONE = 0x00000000;
    FORWARD = 0x00000001;
    BACKWARD = 0x00000002;
    STRAFE_LEFT = 0x00000004;
    STRAFE_RIGHT = 0x00000008;
    LEFT = 0x00000010;
    RIGHT = 0x00000020;
    PITCH_UP = 0x00000040;
    PITCH_DOWN = 0x00000080;
    WALKING = 0x00000100;
    ON_TRANSPORT = 0x00000200;
    DISABLE_GRAVITY = 0x00000400;
    ROOT = 0x00000800;
    FALLING = 0x00001000;
    FALLING_FAR = 0x00002000;
    PENDING_STOP = 0x00004000;
    PENDING_STRAFE_STOP = 0x00008000;
    PENDING_FORWARD = 0x00010000;
    PENDING_BACKWARD = 0x00020000;
    PENDING_STRAFE_LEFT = 0x00040000;
    PENDING_STRAFE_RIGHT = 0x00080000;
    PENDING_ROOT = 0x00100000;
    SWIMMING = 0x00200000;
    ASCENDING = 0x00400000;
    DESCENDING = 0x00800000;
    CAN_FLY = 0x01000000;
    FLYING = 0x02000000;
    SPLINE_ELEVATION = 0x04000000;
    SPLINE_ENABLED = 0x08000000;
    WATERWALKING = 0x10000000;
    FALLING_SLOW = 0x20000000;
    HOVER = 0x40000000;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `FORWARD` | 1 (0x01) |  |  |
| `BACKWARD` | 2 (0x02) |  |  |
| `STRAFE_LEFT` | 4 (0x04) |  |  |
| `STRAFE_RIGHT` | 8 (0x08) |  |  |
| `LEFT` | 16 (0x10) |  |  |
| `RIGHT` | 32 (0x20) |  |  |
| `PITCH_UP` | 64 (0x40) |  |  |
| `PITCH_DOWN` | 128 (0x80) |  |  |
| `WALKING` | 256 (0x100) |  | Walking |
| `ON_TRANSPORT` | 512 (0x200) |  | `AzerothCore`: Used for flying on some creatures |
| `DISABLE_GRAVITY` | 1024 (0x400) |  | `AzerothCore`: Former `MOVEMENTFLAG_LEVITATING`. This is used when walking is not possible. |
| `ROOT` | 2048 (0x800) |  | `AzerothCore`: Must not be set along with `MOVEMENTFLAG_MASK_MOVING` |
| `FALLING` | 4096 (0x1000) |  | `AzerothCore`: damage dealt on that type of falling |
| `FALLING_FAR` | 8192 (0x2000) |  |  |
| `PENDING_STOP` | 16384 (0x4000) |  |  |
| `PENDING_STRAFE_STOP` | 32768 (0x8000) |  |  |
| `PENDING_FORWARD` | 65536 (0x10000) |  |  |
| `PENDING_BACKWARD` | 131072 (0x20000) |  |  |
| `PENDING_STRAFE_LEFT` | 262144 (0x40000) |  |  |
| `PENDING_STRAFE_RIGHT` | 524288 (0x80000) |  |  |
| `PENDING_ROOT` | 1048576 (0x100000) |  |  |
| `SWIMMING` | 2097152 (0x200000) |  | `AzerothCore`: appears with fly flag also |
| `ASCENDING` | 4194304 (0x400000) |  | `AzerothCore`: press 'space' when flying |
| `DESCENDING` | 8388608 (0x800000) |  |  |
| `CAN_FLY` | 16777216 (0x1000000) |  | `AzerothCore`: Appears when unit can fly AND also walk |
| `FLYING` | 33554432 (0x2000000) |  | `AzerothCore`: unit is actually flying. pretty sure this is only used for players. creatures use `disable_gravity` |
| `SPLINE_ELEVATION` | 67108864 (0x4000000) |  | `AzerothCore`: used for flight paths |
| `SPLINE_ENABLED` | 134217728 (0x8000000) |  | `AzerothCore`: used for flight paths |
| `WATERWALKING` | 268435456 (0x10000000) |  | `AzerothCore`: prevent unit from falling through water |
| `FALLING_SLOW` | 536870912 (0x20000000) |  | `AzerothCore`: active rogue safe fall spell (passive) |
| `HOVER` | 1073741824 (0x40000000) |  | `AzerothCore`: hover, cannot jump |

Used in:
* [MovementBlock](movementblock.md)
* [MovementInfo](movementinfo.md)
