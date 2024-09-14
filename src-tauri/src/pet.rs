use bitcoincore_rpc::{Auth, Client, RpcApi};
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
    let blockchain_info = btc_client
        .get_blockchain_info()
        .map_err(|e| e.to_string())?;
    let current_height = blockchain_info.blocks;

    // TODO: Implement logic to track last checked block and calculate new eggs
    // For now, we'll just add 1 egg per check as a placeholder
    let new_eggs:u32 = 1;

    let mut pet = state.pet.lock().unwrap();
    pet.egg_count += new_eggs;
    pet.hunger_level = std::cmp::min(pet.hunger_level + 5, 100);

    Ok(pet.clone())
}

pub fn create_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let btc_client = Client::new(
        "http://localhost:8332",
        Auth::UserPass("rpcuser".to_string(), "rpcpassword".to_string()),
    )?;

    Ok(AppState {
        pet: Mutex::new(Pet {
            hunger_level: 0,
            egg_count: 0,
            name: "Catagotchi".to_string(),
        }),
        btc_client: Mutex::new(btc_client),
    })
}
