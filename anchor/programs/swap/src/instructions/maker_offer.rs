use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::Offer;

use super::transfor_tokens;

pub fn send_offered_tokens_to_vault(
    ctx: &Context<MakeOffer>,
    token_a_offer_amount: u64,
) -> Result<()> {
    transfor_tokens(
        &ctx.accounts.maker_token_account,
        &ctx.accounts.vault,
        &ctx.accounts.maker,
        &token_a_offer_amount,
        &ctx.accounts.token_mint_a,
        &ctx.accounts.token_program,
    )?;
    Ok(())
}

pub fn save_offer(ctx: Context<MakeOffer>, id: u64, wanted_amount_of_token_b: u64) -> Result<()> {
    ctx.accounts.offer.set_inner(Offer {
        id,
        maker: ctx.accounts.maker.key(),
        token_mint_a: ctx.accounts.token_mint_a.key(),
        token_mint_b: ctx.accounts.token_mint_b.key(),
        wanted_amount_of_token_b,
        bump: ctx.bumps.offer,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(id : u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,

    #[account(mint :: token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    #[account(mint :: token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token :: mint = token_mint_a,
        associated_token :: authority = maker,
        associated_token :: token_program = token_program
    )]
    pub maker_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = 8 + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(init,
            payer = maker,
            associated_token::mint = token_mint_a,
            associated_token::authority = offer,
            associated_token::token_program = token_program)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
