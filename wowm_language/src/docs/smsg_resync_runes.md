# SMSG_RESYNC_RUNES

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/spell/smsg_resync_runes.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_resync_runes.wowm#L8).
```rust,ignore
smsg SMSG_RESYNC_RUNES = 0x0487 {
    u32 amount_of_runes;
    ResyncRune[amount_of_runes] runes;
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
| 0x04 | 4 / Little | u32 | amount_of_runes |  |  |
| 0x08 | ? / - | [ResyncRune](resyncrune.md)[amount_of_runes] | runes |  |  |

