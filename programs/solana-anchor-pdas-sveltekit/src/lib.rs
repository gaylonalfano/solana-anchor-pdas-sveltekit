use anchor_lang::prelude::*;

declare_id!("BtNQxoo46aU5qTKxehPSkfgxKKtJZVL7Evuz9pWhwPvN");

#[program]
pub mod solana_anchor_pdas_sveltekit {
    use super::*;

    pub fn create_ledger(ctx: Context<CreateLedger>, color: String) -> Result<()> {

        let ledger_account = &mut ctx.accounts.ledger_account;
        ledger_account.color = color;
        ledger_account.balance = 0;
        Ok(())
    }

    pub fn modify_ledger(ctx: Context<ModifyLedger>, new_balance: u32) -> Result<()> {

        let ledger_account = &mut ctx.accounts.ledger_account;
        ledger_account.balance = new_balance;

        Ok(())
    }

    // Q: How do I implement/pass program instructions data to utilize the
    // custom evaluate function?
    pub fn modify_ledger_with_instruction_data(ctx: Context<ModifyLedger>, data: LedgerInstructions) -> Result<()> {
        // 1. Deserialize so we can work with the account
        let ledger_account = &mut ctx.accounts.ledger_account;
        // 2. Work with account data by using program's instruction data (LedgerInstructions)
        // TODO Probably on the Client side......
        // Q: Is data.evaluate(ledger_account.balance) enough? Obviously need to pass
        // in the LedgerInstructions operation, operation_value args from client...
        // Q: Do I pass the operation, operation_value from client using something like
        // LedgerInstructions { operation: 1, operation_value: 5 } or, do I use the
        // custom util function (e.g., createCalculatorInstructionsBuffer(o, ov)) and BufferLayout?
        // let ledger_instructions = LedgerInstructions::try_from_slice(&data)?;
        ledger_account.balance = data.evaluate(ledger_account.balance);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(color: String)] // This is telling Anchor to expect color arg from instruction fn call
pub struct CreateLedger<'info> {
    #[account(init, payer = wallet, space = 25, seeds = [wallet.key().as_ref(), b"_", color.as_ref()], bump )]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyLedger<'info> {
    // NOTE Anchor will check that this ledger_account has ALREADY been created (CheckedAccount??)
    // Q: Is this why in some other projects we use UncheckedAccount as the type?
    // For example, in MintNft struct, we have:
    // ==
    // #[account(mut)]
    // pub token_account = UncheckedAccount<'info>
    // ==
    // since Anchor will eventually create and initialize the token_account.
    #[account(mut)]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
}

#[account]
pub struct Ledger {
    // NOTE The Cookbook says it's common to store the bump seed in PDA account data itself!
    // pub bump_seed: u8;
    pub color: String,
    pub balance: u32,
}

// Q: How do you create the struct for the program's instruction data using Anchor?
// A: Looks like it's the same as any Account struct by adding #[account]
// REF: https://book.anchor-lang.com/anchor_in_depth/the_program_module.html
#[account]
pub struct LedgerInstructions {
    operation: u32,
    operation_value: u32,
}

impl LedgerInstructions {
    pub fn evaluate(self, value: u32) -> u32 {
        // Modify the incoming Ledger balance by this value
        match &self.operation {
            1 => value + &self.operation_value,
            2 => value - &self.operation_value,
            3 => value * &self.operation_value,
            _ => value * 0, // Reset balance to 0
        }
    }
    
}
