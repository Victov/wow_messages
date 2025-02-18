# RollVote

## Client Version 1

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/loot/loot_common.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/loot_common.wowm#L1).

```rust,ignore
enum RollVote : u8 {
    PASS = 0;
    NEED = 1;
    GREED = 2;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `PASS` | 0 (0x00) |  |  |
| `NEED` | 1 (0x01) |  |  |
| `GREED` | 2 (0x02) |  |  |

Used in:
* [CMSG_LOOT_ROLL](cmsg_loot_roll.md)
* [SMSG_LOOT_ROLL](smsg_loot_roll.md)
* [SMSG_LOOT_ROLL_WON](smsg_loot_roll_won.md)

# RollVote

## Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/loot/loot_common.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/loot_common.wowm#L9).

```rust,ignore
enum RollVote : u8 {
    PASS = 0;
    NEED = 1;
    GREED = 2;
    DISENCHANT = 3;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `PASS` | 0 (0x00) |  |  |
| `NEED` | 1 (0x01) |  |  |
| `GREED` | 2 (0x02) |  |  |
| `DISENCHANT` | 3 (0x03) |  |  |

Used in:
* [CMSG_LOOT_ROLL](cmsg_loot_roll.md)
* [SMSG_LOOT_ROLL](smsg_loot_roll.md)
* [SMSG_LOOT_ROLL_WON](smsg_loot_roll_won.md)

