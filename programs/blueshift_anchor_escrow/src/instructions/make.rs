#[instruction(see:u64)]
pub struct make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init,payer=maker,
        space = Escrow::INIT_SPACE+ Escrow::DISCRIMINATIOR.len(),
        seeds =[b"escrow", maker.key().as_ref(),seed.to_le_bytes().as_ref],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    ///Token Accounts
    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,

    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        associated_token::mint=mint_a,
        associated_token::authority=escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    ///Programs
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    /// Create the escrow
    fn populate_escrow(&mut self, seed: u64, amount: u64, bump: u8) -> result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive: amount,
            bump,
        });

        Ok(())
    }

    /// # Deposit Tokens
    fn deposit_tokens(&self, amount: u64) -> Result<()> {
        transfer_checked(
            Cpi::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.maker_ata_a.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.maker.to_account_info(),
                },
            ),
            amount,
            self.mint_a.decimals,
        )?;

        Ok(())
    }
    pub fn handler(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        //validate the amount
        require_gte!(receive, 0, EscrowError::InvalidAmount);
        require_gte!(amount, 0, EscrowError::InvalisAmount);

        //save the escrow DATA
        ctx.accounts
            .populate_escrow(seed, receive, ctx.bumps.escrow)?;

        //deposit tokens
        ctx.accounts.deposit_tokens(amount)?;

        Ok(())
    }
}
