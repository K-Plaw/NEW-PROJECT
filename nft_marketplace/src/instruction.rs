use borsh::{BorshDeserialize, BorshSerialize};

/// Enum describing all nft marketplace instructions.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum NFTMarketplaceInstructions {
    /// InitializeMarketplace { name, fee }
    InitializeMarketplace {
        name: String,
        fee: u16,
    },
    /// ListNFT { price, listing_bump }
    ListNFT {
        price: u64,
        listing_bump: u8,
    },
    /// DelistNFT { listing_bump }
    DelistNFT {
        listing_bump: u8,
    },
    /// PurchaseNFT { }
    PurchaseNFT {},
    /// UpdateFee { updated_fee }
    UpdateFee {
        updated_fee: u16,
    },
}