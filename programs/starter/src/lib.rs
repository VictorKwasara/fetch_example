use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod starter {
    use super::*;
    pub fn initialize(ctx: Context<Example>) -> Result<()> {
    //  Here we set the message and authority for account one
     ctx.accounts.one.message = "Hey there!".to_string();
     ctx.accounts.one.authority = *ctx.accounts.payer.key ;

     //  Here we set the message account two
     ctx.accounts.two.message = "Solana is awesome!".to_string();
     Ok(())
    }
}

#[derive(Accounts)]
pub struct Example <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init,payer=payer, space= 8 + One::INIT_SPACE, seeds=[b"hello"], bump)]
    pub one:Account<'info,One>,

    #[account(init,payer=payer, space= 8 + Two::INIT_SPACE, seeds=[b"helloWorld",payer.key().as_ref()], bump)]
    pub two:Account<'info,Two>,

    pub system_program: Program<'info, System>,
}

//Many accounts of this type 
//But Each is unique per user 

#[account]
#[derive(Default, InitSpace)]
pub struct One {
   pub authority: Pubkey,
   #[max_len(35)]
   pub message : String ,
}


//Only one type of this account 
#[account]
#[derive(Default, InitSpace)]
pub struct Two {
   #[max_len(35)]
   pub message : String ,
}
