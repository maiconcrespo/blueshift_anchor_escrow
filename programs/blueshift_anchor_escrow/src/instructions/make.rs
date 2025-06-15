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
