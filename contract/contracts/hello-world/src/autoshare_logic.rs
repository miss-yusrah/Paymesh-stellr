use crate::base::errors::Error;
use crate::base::events::emit_autoshare_created;
use crate::base::types::AutoShareDetails;
use soroban_sdk::{contracttype, Address, BytesN, Env, String};

#[contracttype]
pub enum DataKey {
    AutoShare(BytesN<32>),
}

pub fn create_autoshare(
    env: Env,
    id: BytesN<32>,
    name: String,
    creator: Address,
) -> Result<(), Error> {
    let key = DataKey::AutoShare(id.clone());

    // Check if it already exists to prevent overwriting
    if env.storage().persistent().has(&key) {
        return Err(Error::AlreadyExists);
    }

    let details = AutoShareDetails {
        id: id.clone(),
        name,
        creator: creator.clone(),
    };

    // Store the details in persistent storage
    env.storage().persistent().set(&key, &details);

    // Emit the success event
    emit_autoshare_created(&env, id, creator);
    Ok(())
}

pub fn get_autoshare(env: Env, id: BytesN<32>) -> Result<AutoShareDetails, Error> {
    let key = DataKey::AutoShare(id);
    env.storage().persistent().get(&key).ok_or(Error::NotFound)
}
