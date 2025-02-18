# CMSG_SET_PLAYER_DECLINED_NAMES

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/social/cmsg_set_player_declined_names.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_set_player_declined_names.wowm#L1).
```rust,ignore
cmsg CMSG_SET_PLAYER_DECLINED_NAMES = 0x0418 {
    Guid player;
    CString name;
    CString[5] declined_names;
}
```
### Header

CMSG have a header of 6 bytes.

#### CMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | player |  |  |
| 0x0E | - / - | CString | name |  |  |
| - | ? / - | CString[5] | declined_names |  |  |

# CMSG_SET_PLAYER_DECLINED_NAMES

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/social/cmsg_set_player_declined_names.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_set_player_declined_names.wowm#L9).
```rust,ignore
cmsg CMSG_SET_PLAYER_DECLINED_NAMES = 0x0419 {
    Guid player;
    CString name;
    CString[5] declined_names;
}
```
### Header

CMSG have a header of 6 bytes.

#### CMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | player |  |  |
| 0x0E | - / - | CString | name |  |  |
| - | ? / - | CString[5] | declined_names |  |  |

