
type User_PrivateKey = Hash256;
type User_TransactionAddress = Hash160;

struct Wallet {
    personal_key: Vec<User_PrivateKey>,
    wallet_address: Vec<User_TransactionAddress>,
}

impl Wallet {
    fn new_wallet(&self) ->Wallet {

    }
}

fn apply_transaction() { 

}
fn transfer() {
    
}