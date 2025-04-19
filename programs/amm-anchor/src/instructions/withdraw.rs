use anchor_lang::prelude::*;
use crate::{error::AmmError,state::Config};

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer}
};

use constant_product_curve::ConstantProduct;


#[derive(Accounts)]
pub struct Withdraw <'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
        has_one = mint_x,
        has_one = mint_y
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        mut,
        associated_token:: mint = mint_x,
        associated_token:: authority = config
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = mint_y,
        associated_token:: authority = config
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = mint_x,
        associated_token:: authority = user
    )]
    pub user_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = mint_y,
        associated_token:: authority = user
    )]
    pub user_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = mint_lp,
        associated_token:: authority = user
    )]
    pub user_lp: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl <'info>

