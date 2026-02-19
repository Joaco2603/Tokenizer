use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataKey {
    /// Tracks the current app version for monotonic post-upgrade migrations.
    AppVersion,
}
