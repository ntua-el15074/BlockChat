use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender_address: i32,
    pub receiver_address: i32,
    pub type_of_transaction: bool,
    pub amount: f32,
    pub message: String,
    pub nonce: i32,
    pub transaction_id: i32,
    pub signature: String,
}

impl Transaction {
    
    // Constructor function for struct Transaction
    pub fn new(sender_address: i32, receiver_address: i32,
               type_of_transaction: bool, amount: f32, message: String,
               nonce: i32, transaction_id: i32, signature: String) -> Self {
        Transaction {
            sender_address,
            receiver_address,
            type_of_transaction,
            amount,
            message,
            nonce,
            transaction_id,
            signature,
        }

    }

    // Jsonify Transaction Object
    pub fn jsonify(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }








}
