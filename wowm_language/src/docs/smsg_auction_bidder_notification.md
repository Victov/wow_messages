# SMSG_AUCTION_BIDDER_NOTIFICATION

## Client Version 1, Client Version 2

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm#L1).
```rust,ignore
smsg SMSG_AUCTION_BIDDER_NOTIFICATION = 0x025E {
    u32 auction_house_id;
    u32 auction_id;
    Guid bidder;
    u32 won;
    u32 out_bid;
    u32 item_template;
    u32 item_random_property_id;
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
| 0x04 | 4 / Little | u32 | auction_house_id |  |  |
| 0x08 | 4 / Little | u32 | auction_id |  |  |
| 0x0C | 8 / Little | [Guid](../spec/packed-guid.md) | bidder |  |  |
| 0x14 | 4 / Little | u32 | won |  | vmangos/cmangos: if 0, client shows ERR_AUCTION_WON_S, else ERR_AUCTION_OUTBID_S |
| 0x18 | 4 / Little | u32 | out_bid |  |  |
| 0x1C | 4 / Little | u32 | item_template |  |  |
| 0x20 | 4 / Little | u32 | item_random_property_id |  |  |

# SMSG_AUCTION_BIDDER_NOTIFICATION

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm#L15).
```rust,ignore
smsg SMSG_AUCTION_BIDDER_NOTIFICATION = 0x025E {
    u32 auction_house_id;
    u32 auction_id;
    Guid bidder;
    u32 bid_sum;
    u32 new_highest_bid;
    u32 out_bid_amount;
    u32 item_template;
    u32 item_random_property_id;
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
| 0x04 | 4 / Little | u32 | auction_house_id |  |  |
| 0x08 | 4 / Little | u32 | auction_id |  |  |
| 0x0C | 8 / Little | [Guid](../spec/packed-guid.md) | bidder |  |  |
| 0x14 | 4 / Little | u32 | bid_sum |  |  |
| 0x18 | 4 / Little | u32 | new_highest_bid |  |  |
| 0x1C | 4 / Little | u32 | out_bid_amount |  |  |
| 0x20 | 4 / Little | u32 | item_template |  |  |
| 0x24 | 4 / Little | u32 | item_random_property_id |  |  |

