use std::convert::{TryFrom, TryInto};
use crate::world::tbc::AuctionListItem;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_list_result.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_list_result.wowm#L9):
/// ```text
/// smsg SMSG_AUCTION_BIDDER_LIST_RESULT = 0x0265 {
///     u32 count;
///     AuctionListItem[count] auctions;
///     u32 total_amount_of_auctions;
///     u32 auction_search_delay;
/// }
/// ```
pub struct SMSG_AUCTION_BIDDER_LIST_RESULT {
    pub auctions: Vec<AuctionListItem>,
    pub total_amount_of_auctions: u32,
    pub auction_search_delay: u32,
}

impl crate::Message for SMSG_AUCTION_BIDDER_LIST_RESULT {
    const OPCODE: u32 = 0x0265;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // count: u32
        w.write_all(&(self.auctions.len() as u32).to_le_bytes())?;

        // auctions: AuctionListItem[count]
        for i in self.auctions.iter() {
            i.write_into_vec(w)?;
        }

        // total_amount_of_auctions: u32
        w.write_all(&self.total_amount_of_auctions.to_le_bytes())?;

        // auction_search_delay: u32
        w.write_all(&self.auction_search_delay.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // count: u32
        let count = crate::util::read_u32_le(r)?;

        // auctions: AuctionListItem[count]
        let mut auctions = Vec::with_capacity(count as usize);
        for i in 0..count {
            auctions.push(AuctionListItem::read(r)?);
        }

        // total_amount_of_auctions: u32
        let total_amount_of_auctions = crate::util::read_u32_le(r)?;

        // auction_search_delay: u32
        let auction_search_delay = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctions,
            total_amount_of_auctions,
            auction_search_delay,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_AUCTION_BIDDER_LIST_RESULT {}

impl SMSG_AUCTION_BIDDER_LIST_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // count: u32
        + self.auctions.len() * 136 // auctions: AuctionListItem[count]
        + 4 // total_amount_of_auctions: u32
        + 4 // auction_search_delay: u32
    }
}

