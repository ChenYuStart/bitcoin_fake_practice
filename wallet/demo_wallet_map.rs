
type WalletAddress = String;
pub const WALLET_FILE: &str = "wallet.dat";

struct WalletMap {
    wallet_map: HashMap<WalletAddress, Wallet>
}

impl WalletMap {
    pub fn new() -> Result<Self, BlockChainError> {
        let wallet_map = WalletMap::load_wallet_from_file();
        wallet_map
    }

    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallet_map.insert(address.clone(), wallet);
        self.save_wallet_to_file().unwrap();
        address
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallet_map.get(address)
    }

    fn get_addresses(&self) -> Vec<&String> {
        self.wallet_map.keys().collect()
    }

    fn save_wallet_to_file(&self) -> Result<(), BlockChainError> {
        let path = current_dir().unwrap().join(WALLET_FILE);
        let wallet_map_serialize = serialize(&self)?;
        fs::write(path, &wallet_map_serialize).unwrap();
        Ok(())
    }

    pub fn load_wallet_from_file() -> Result<Self, BlockChainError> {
        let path = current_dir().unwrap().join(WALLET_FILE);
        info!("Wallet path: {:?}", path);
        if !path.exists() {
            let wallet_map = WalletMap {
                wallet_map: HashMap::new(),
            };
            return Ok(wallet_map);
        }

        let wallets_serialize = fs::read(&path).unwrap();
        let wallet_map = deserialize(&wallets_serialize);
        wallet_map
    }
}