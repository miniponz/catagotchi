use bitcoincore_rpc::{json::GetBlockchainInfoResult, Auth, Client, RpcApi};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pet {
    name: String,
    hunger_level: u32,
    egg_count: u32,
}

pub struct AppState {
    pet: Mutex<Pet>,
    btc_client: Mutex<Client>,
    last_block_checked: Mutex<u64>,
}

#[tauri::command]
pub fn greet(name: &str, state: State<AppState>) -> String {
    let pet = state.pet.lock().unwrap();
    format!("Hello, {}! You've been greeted by {}!", name, pet.name)
}

#[tauri::command]
pub fn get_pet_state(state: State<'_, AppState>) -> Pet {
    state.pet.lock().unwrap().clone()
}

#[tauri::command]
pub fn feed_pet(state: State<AppState>) -> Pet {
    let mut pet = state.pet.lock().unwrap();
    if pet.egg_count > 0 {
        pet.hunger_level = pet.hunger_level.saturating_sub(10);
        pet.egg_count -= 1;
    }
    pet.clone()
}

#[tauri::command]
pub fn check_bitcoin_blocks(state: State<AppState>) -> Result<Pet, String> {
    let btc_client = state.btc_client.lock().unwrap();
    let blockchain_info: GetBlockchainInfoResult = btc_client
        .get_blockchain_info()
        .map_err(|e| e.to_string())?;
    let current_height: u64 = blockchain_info.blocks;

    let mut last_block_checked = state.last_block_checked.lock().unwrap();
    let new_blocks: u64 = current_height - *last_block_checked;

    let mut new_eggs: u32 = 0;

    if new_blocks > 0 {
        for block_height in *last_block_checked + 1..=current_height {
            // TODO: include transaction fees from block specific data in block reward/egg calculation
            // let block_hash = btc_client
            //     .get_block_hash(block_height)
            //     .map_err(|e| e.to_string())?;
            // let block = btc_client
            //     .get_block(&block_hash)
            //     .map_err(|e| e.to_string())?;

            let block_reward = calculate_block_reward(block_height);
            
            new_eggs += (block_reward * 10.0).round() as u32;
        }

        *last_block_checked = current_height;
    }

    let mut pet = state.pet.lock().unwrap();
    pet.egg_count += new_eggs;
    // 5 is arbitrary, revisit later
    pet.hunger_level = std::cmp::min(pet.hunger_level + (new_blocks as u32 * 5), 100);

    Ok(pet.clone())
}

fn calculate_block_reward(height: u64) -> f64 {
    // by halvings excluding transaction fees
    let halvings: u64 = height / 210_000;
    if halvings >= 64 {
        // lol imagine all bitcoins mined
        return 0.0;
    }
    50.0 / 2_f64.powi(halvings as i32)
}

pub fn create_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let btc_client = Client::new(
        // TODO: use env to set this to testnet/mainnet
        "http://127.0.0.1:18332",
        Auth::UserPass("your_rpc_username".to_string(), "your_rpc_password".to_string()),
    )?;

    let blockchain_info: GetBlockchainInfoResult = btc_client.get_blockchain_info()?;
    let current_height: u64 = blockchain_info.blocks;

    Ok(AppState {
        pet: Mutex::new(Pet {
            hunger_level: 0,
            egg_count: 0,
            name: "Catagotchi".to_string(),
        }),
        btc_client: Mutex::new(btc_client),
        last_block_checked: Mutex::new(current_height),
    })
}
