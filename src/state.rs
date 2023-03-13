use cosmwasm_std::{Binary, ContractInfo};
use secret_toolkit::storage::Item;
use serde::{Deserialize, Serialize};

/// storage for MigratedFromState singleton
pub static MIGRATED_FROM: Item<MigratedFromState> = Item::new(b"migratedFrom");
/// storage for MigratedToState singleton
pub static MIGRATED_TO: Item<MigratedToState> = Item::new(b"migratedTo");
/// storage for list of contracts to notify when this contract has been migrated
pub static NOTIFY_ON_MIGRATION_COMPLETE: Item<Vec<ContractInfo>> = Item::new(b"notifyOnMigrated");
/// storage for current ContractMode
pub static CONTRACT_MODE: Item<ContractMode> = Item::new(b"contractMode");

#[derive(
    serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, PartialEq, strum::EnumIter,
)]
#[repr(u8)]
pub enum ContractMode {
    MigrateDataIn = 1,
    Running = 2,
    // MigrateOutStarted is applicable when a migration can take more than one transaction to complete
    // For example when migrating a snip721 contract. A takes a least two transactions
    // The first execute migration message put's the contract in this state.
    // Which disables the contract's state from being altered but allows queries until migration
    // is complete (after all the tokens are migrated in a second transaction).
    // After all tokens are migrated. the contract switches to ContractMode::MigratedOut
    MigrateOutStarted = 3,
    MigratedOut = 4,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MigratedFromState {
    /// the info of the contract being migrated from
    pub contract: ContractInfo,
    /// the secret generated by the contract being migrated from
    pub migration_secret: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MigratedToState {
    /// the info of the contract being migrated to
    pub contract: ContractInfo,
    /// the secret needed by another contract to migrate data out
    pub migration_secret: Binary,
}
