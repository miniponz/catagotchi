# Catagotchi Node Fren Setup Guide

## Prerequisites

1. Rust and Cargo (latest stable version)
2. Node.js and npm (for the frontend)
3. Tauri CLI
4. Bitcoin Core

## Step 1: Install Bitcoin Core

1. Download Bitcoin Core: https://bitcoin.org/en/download
2. Install Bitcoin Core on your system. On Mac, drag Bitcoin Core into your Applications folder. You may have to right click -> open 
3. Run Bitcoin Core in testnet mode to avoid using real bitcoins:

- On macOS:

    1. Open Terminal
    2. Enter the command: 
    ```
    /Applications/Bitcoin-Qt.app/Contents/MacOS/Bitcoin-Qt -testnet
    ```
## Step 2: Configure Bitcoin Core

1. Create or edit the `bitcoin.conf` file:
   - On Linux: `~/.bitcoin/bitcoin.conf`
   - On macOS: `~/Library/Application Support/Bitcoin/bitcoin.conf`
   - On Windows: `%APPDATA%\Bitcoin\bitcoin.conf`

2. Add the following lines to enable RPC:
   ```
   rpcallowip=127.0.0.1
   server=1

   [test]
   testnet=1
   rpcuser=your_rpc_username
   rpcpassword=your_rpc_password
   ```

3. Restart Bitcoin Core for the changes to take effect

## Step 3: Update Tauri App Configuration

1. In `pet.rs`, update the Bitcoin client initialization:
   ```rust
   let bitcoin_client = Client::new(
       "http://127.0.0.1:18332",  // Use 18332 for testnet
       Auth::UserPass("your_rpc_username".to_string(), "your_rpc_password".to_string()),
   )?;
   ```

2. Make sure that `tauri.conf.json` has permission for network access:
   ```json
   {
     "tauri": {
       "allowlist": {
         "http": {
           "all": true,
           "request": true
         }
       }
     }
   }
   ```

## Step 4: Run the Tauri App

1. Open a terminal and navigate to your project directory, then at the root

2. Install frontend dependencies:
   ```
   yarn install
   ```

3. Start the Tauri development server:
   ```
   yarn tauri dev
   ```

This command will compile your Rust code and start the app.

## Troubleshooting

- For RPC connection errors, make sure Bitcoin Core is running and the RPC credentials, port, etc. in your `pet.rs` file match those in your local `bitcoin.conf`
- If the app can't connect to the Bitcoin node, check that the node is fully synced with testnet
- For permission issues, make sure your `tauri.conf.json` is correctly configured for network access
