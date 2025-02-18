# Relation

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/social/smsg_contact_list.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_contact_list.wowm#L21).
```rust,ignore
struct Relation {
    Guid guid;
    RelationType relation_mask;
    CString note;
    if (relation_mask & FRIEND) {
        FriendStatus status;
        if (status == ONLINE) {
            Area area;
            Level32 level;
            (u32)Class class;
        }
    }
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| 0x08 | 4 / - | [RelationType](relationtype.md) | relation_mask |  |  |
| 0x0C | - / - | CString | note |  |  |

If relation_mask contains `FRIEND`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 1 / - | [FriendStatus](friendstatus.md) | status |  |  |

If status is equal to `ONLINE`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / - | [Area](area.md) | area |  |  |
| - | 4 / Little | Level32 | level |  |  |
| - | 4 / - | [Class](class.md) | class |  |  |

# Relation

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/social/smsg_contact_list.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_contact_list.wowm#L21).
```rust,ignore
struct Relation {
    Guid guid;
    RelationType relation_mask;
    CString note;
    if (relation_mask & FRIEND) {
        FriendStatus status;
        if (status == ONLINE) {
            Area area;
            Level32 level;
            (u32)Class class;
        }
    }
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| 0x08 | 4 / - | [RelationType](relationtype.md) | relation_mask |  |  |
| 0x0C | - / - | CString | note |  |  |

If relation_mask contains `FRIEND`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 1 / - | [FriendStatus](friendstatus.md) | status |  |  |

If status is equal to `ONLINE`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / - | [Area](area.md) | area |  |  |
| - | 4 / Little | Level32 | level |  |  |
| - | 4 / - | [Class](class.md) | class |  |  |

