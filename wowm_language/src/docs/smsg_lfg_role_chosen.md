# SMSG_LFG_ROLE_CHOSEN

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/lfg/smsg_lfg_role_chosen.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_role_chosen.wowm#L1).
```rust,ignore
smsg SMSG_LFG_ROLE_CHOSEN = 0x02BB {
    Guid guid;
    Bool ready;
    u32 roles;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| 0x0C | 1 / - | Bool | ready |  |  |
| 0x0D | 4 / Little | u32 | roles |  |  |

