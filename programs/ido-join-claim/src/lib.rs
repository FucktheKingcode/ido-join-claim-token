use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use spl_associated_token_account::create_associated_token_account;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("9S4HT9tekg7BZXGeyG5VdCrKgiZ4aRL88yMurfi8vXqQ");

#[program]
mod ido_join_claim {
    use super::*;

    pub fn create_ata(ctx: Context<CreateAta>) -> Result<()> {
        let payer = ctx.accounts.payer.to_account_info();
        let token_mint = ctx.accounts.token_mint.to_account_info();
        let associated_token_account = ctx.accounts.associated_token_account.to_account_info();

        let create_ata_ix = create_associated_token_account(
            payer.key,
            associated_token_account.key,
            token_mint.key,
        );

        let mut instructions = vec![create_ata_ix];

        **ctx
            .accounts
            .system_program
            .to_account_info()
            .clone()
            .push(&mut instructions);
        **msg!("Sending transaction...");
        let tx_hash = ctx
            .program
            .rpc()
            .send_and_confirm_transaction(&instructions)?;
        msg!("Transaction hash: {}", tx_hash);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAta<'info> {
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    #[account(init, payer = payer, associated_token: mint)]
    pub associated_token_account: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    pub token_program: Program<'info, Token>,
    #[account]
    pub token_mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
