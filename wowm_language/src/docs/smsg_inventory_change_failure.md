# SMSG_INVENTORY_CHANGE_FAILURE

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/item/smsg_inventory_change_failure.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_inventory_change_failure.wowm#L1).
```rust,ignore
smsg SMSG_INVENTORY_CHANGE_FAILURE = 0x0112 {
    InventoryResult result;
    if (result == CANT_EQUIP_LEVEL_I) {
        Level32 required_level;
    }
    if (result != OK) {
        Guid item1;
        Guid item2;
        u8 bag_type_subclass;
    }
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
| 0x04 | 1 / - | [InventoryResult](inventoryresult.md) | result |  |  |

If result is equal to `CANT_EQUIP_LEVEL_I`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x05 | 4 / Little | Level32 | required_level |  |  |

If result is not equal to `OK`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x09 | 8 / Little | [Guid](../spec/packed-guid.md) | item1 |  |  |
| 0x11 | 8 / Little | [Guid](../spec/packed-guid.md) | item2 |  |  |
| 0x19 | 1 / - | u8 | bag_type_subclass |  | cmangos: bag type subclass, used with EQUIP_ERR_EVENT_AUTOEQUIP_BIND_CONFIRM and EQUIP_ERR_ITEM_DOESNT_GO_INTO_BAG2<br/>vmangos sets to 0 |

# SMSG_INVENTORY_CHANGE_FAILURE

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/item/smsg_inventory_change_failure.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_inventory_change_failure.wowm#L18).
```rust,ignore
smsg SMSG_INVENTORY_CHANGE_FAILURE = 0x0112 {
    InventoryResult result;
    if (result != OK) {
        Guid item1;
        Guid item2;
        u8 bag_type_subclass;
    }
    if (result == CANT_EQUIP_LEVEL_I) {
        Level32 required_level;
    }
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
| 0x04 | 1 / - | [InventoryResult](inventoryresult.md) | result |  |  |

If result is not equal to `OK`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x05 | 8 / Little | [Guid](../spec/packed-guid.md) | item1 |  |  |
| 0x0D | 8 / Little | [Guid](../spec/packed-guid.md) | item2 |  |  |
| 0x15 | 1 / - | u8 | bag_type_subclass |  | cmangos: bag type subclass, used with EQUIP_ERR_EVENT_AUTOEQUIP_BIND_CONFIRM and EQUIP_ERR_ITEM_DOESNT_GO_INTO_BAG2<br/>vmangos sets to 0 |

If result is equal to `CANT_EQUIP_LEVEL_I`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x16 | 4 / Little | Level32 | required_level |  |  |

# SMSG_INVENTORY_CHANGE_FAILURE

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/item/smsg_inventory_change_failure.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_inventory_change_failure.wowm#L18).
```rust,ignore
smsg SMSG_INVENTORY_CHANGE_FAILURE = 0x0112 {
    InventoryResult result;
    if (result != OK) {
        Guid item1;
        Guid item2;
        u8 bag_type_subclass;
    }
    if (result == CANT_EQUIP_LEVEL_I) {
        Level32 required_level;
    }
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
| 0x04 | 1 / - | [InventoryResult](inventoryresult.md) | result |  |  |

If result is not equal to `OK`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x05 | 8 / Little | [Guid](../spec/packed-guid.md) | item1 |  |  |
| 0x0D | 8 / Little | [Guid](../spec/packed-guid.md) | item2 |  |  |
| 0x15 | 1 / - | u8 | bag_type_subclass |  | cmangos: bag type subclass, used with EQUIP_ERR_EVENT_AUTOEQUIP_BIND_CONFIRM and EQUIP_ERR_ITEM_DOESNT_GO_INTO_BAG2<br/>vmangos sets to 0 |

If result is equal to `CANT_EQUIP_LEVEL_I`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x16 | 4 / Little | Level32 | required_level |  |  |

