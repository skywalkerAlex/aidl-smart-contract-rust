use anchor_lang::prelude::*;

declare_id!("AxEL6oqhuMeeGnAZJ9rzoM9FztdBUt3t7LmKgUwKF3Dk");

#[program]
pub mod aidl_smart_contract_rust {
    use super::*;

    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.
        base_account.total_datasets = 0;
        Ok(())
    }

    // The function now accepts a gif_link param from the user. We also reference the user from the Context
    pub fn upload_dataset_details(ctx: Context<UploadDatasetDetails>, uploaded_dataset: DatasetObj) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user; 

        let dataset_item = DatasetObj {
            user_address: *user.to_account_info().key,
            name: uploaded_dataset.name,
            size: uploaded_dataset.size,
            accuracy_score: uploaded_dataset.accuracy_score,
            data_type: uploaded_dataset.data_type,
            file_type: uploaded_dataset.file_type,
            models_used: uploaded_dataset.models_used,
            libraries_used: uploaded_dataset.libraries_used,
        };
        // Add it to the account vector.
        base_account.datasets_list.push(dataset_item);
        base_account.total_datasets += 1;
        Ok(())
    }

}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct UploadDatasetDetails<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Attach certain variables to the Initialize context.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)] // 'space' in bytes
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_datasets: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub datasets_list: Vec<DatasetObj>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DatasetObj {
    pub user_address: Pubkey,
    pub name: String,
    pub size: String,
    pub accuracy_score: String,
    pub data_type: String,
    pub file_type: String,
    pub models_used: Vec<String>,
    pub libraries_used: Vec<String>,
}