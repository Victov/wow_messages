# BattlegroundEndStatus

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm#L3).

```rust,ignore
enum BattlegroundEndStatus : u8 {
    NOT_ENDED = 0;
    ENDED = 1;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NOT_ENDED` | 0 (0x00) |  |  |
| `ENDED` | 1 (0x01) |  |  |

Used in:
* [MSG_PVP_LOG_DATA_Server](msg_pvp_log_data_server.md)

