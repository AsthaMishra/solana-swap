use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use super::transfor_tokens;
use crate::Offer;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    pub maker: SystemAccount<'info>,
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = taker,
        associated_token :: mint = token_mint_a,
        associated_token :: authority = taker,
        associated_token :: token_program = token_program,
    )]
    pub taker_token_account_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token :: mint = token_mint_b,
        associated_token :: authority = taker,
        associated_token :: token_program = token_program,
    )]
    pub taker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token :: mint = token_mint_b,
        associated_token :: authority = maker,
        associated_token :: token_program = token_program,
    )]
    pub maker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
       close = maker,
       has_one = maker,
       has_one = token_mint_a,
       has_one = token_mint_b,
       seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
       bump = offer.bump
    )]
    offer: Account<'info, Offer>,

    #[account(mut,
    associated_token :: mint = token_mint_a,
    associated_token :: authority = offer,
    associated_token :: token_program = token_program)]
    vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn send_wanted_tokens_to_maker(ctx: &Context<TakeOffer>) -> Result<()> {
    transfor_tokens(
        &ctx.accounts.taker_token_account_a,
        &ctx.accounts.maker_token_account_b,
        &ctx.accounts.taker,
        &ctx.accounts.offer.wanted_amount_of_token_b,
        &ctx.accounts.token_mint_b,
        &ctx.accounts.token_program,
    )?;

    Ok(())
}

pub fn withdraw_and_close_vault(ctx: Context<TakeOffer>) -> Result<()> {
    let seeds = &[
        b"offer",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &ctx.accounts.offer.id.to_le_bytes()[..],
        &[ctx.accounts.offer.bump],
    ];
    let signer = &[&seeds[..]];
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.taker_token_account_a.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    transfer_checked(
        cpi_ctx,
        ctx.accounts.vault.amount,
        ctx.accounts.token_mint_a.decimals,
    )?;

    let accounts = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        destination: ctx.accounts.taker.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        accounts,
        signer,
    );

    close_account(cpi_ctx)
}
