use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The escrow has expired")]
    EscrowExpired,

    #[msg("The escrow has not expired yet")]
    EscrowNotExpired,

    #[msg("Amount must be greater than zero")]
    InvalidAmount,

    #[msg("Expiration must be in the future")]
    InvalidExpiration,

    #[msg("Maker cannot take their own escrow")]
    MakerCannotTakeOwnEscrow,

    #[msg("Both token mints must be different")]
    SameMint,
}
