#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short, log, Address};

#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub id: u64,
    pub name: String,
    pub metadata_uri: String,
    pub owner: Address,
}

#[contracttype]
pub enum NFTKey {
    Token(u64),
    Count,
}

#[contract]
pub struct BasicNFTMinter;

#[contractimpl]
impl BasicNFTMinter {
    pub fn mint(env: Env, name: String, metadata_uri: String, owner: Address) -> u64 {
        let mut count: u64 = env.storage().instance().get(&NFTKey::Count).unwrap_or(0);
        count += 1;

        let token = NFT {
            id: count,
            name,
            metadata_uri,
            owner: owner.clone(),
        };

        env.storage().instance().set(&NFTKey::Token(count), &token);
        env.storage().instance().set(&NFTKey::Count, &count);

        log!(&env, "NFT Minted: ID={}, Owner={}", count, owner);

        count
    }

    pub fn get_nft(env: Env, id: u64) -> NFT {
        env.storage().instance().get(&NFTKey::Token(id)).unwrap()
    }

    pub fn total_supply(env: Env) -> u64 {
        env.storage().instance().get(&NFTKey::Count).unwrap_or(0)
    }
}
