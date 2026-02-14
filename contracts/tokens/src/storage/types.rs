use soroban_sdk::{contracttype};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataKey {
    // Add custom storage keys here if needed
    // standard token keys are managed by stellar_tokens
    Admin, 
}
