# SMSG_CHANNEL_LIST

## Client Version 1, Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/chat/smsg_channel_list.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_list.wowm#L29).
```rust,ignore
smsg SMSG_CHANNEL_LIST = 0x009B {
    CString channel_name;
    ChannelFlags channel_flags;
    u32 amount_of_members;
    ChannelMember[amount_of_members] members;
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
| 0x04 | - / - | CString | channel_name |  |  |
| - | 1 / - | [ChannelFlags](channelflags.md) | channel_flags |  |  |
| - | 4 / Little | u32 | amount_of_members |  |  |
| - | ? / - | [ChannelMember](channelmember.md)[amount_of_members] | members |  |  |

