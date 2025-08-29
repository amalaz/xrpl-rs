use serde::{Deserialize, Serialize};

pub type Address = String;
pub type SecretKey = String;
pub type CurrencyCode = String;
pub type Amount = String;
pub type TransactionHash = String;
pub type Sequence = u32;
pub type Fee = String;
pub type Timestamp = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDetails {
    pub destination: Address,
    pub amount: Amount,
    pub currency: CurrencyCode,
    pub issuer: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_type: String,
    pub account: Address,
    pub destination: Address,
    pub amount: Amount,
    pub currency: CurrencyCode,
    pub issuer: Option<Address>,
    pub fee: Fee,
    pub sequence: Sequence,
    pub flags: Option<u32>,
    pub last_ledger_sequence: Option<u32>,
    pub source_tag: Option<u32>,
    pub destination_tag: Option<u32>,
    pub invoice_id: Option<String>,
    pub paths: Option<Vec<Vec<serde_json::Value>>>,
    pub send_max: Option<Amount>,
    pub deliver_min: Option<Amount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub tx_blob: String,
    pub tx_json: Transaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub hash: TransactionHash,
    pub validated: bool,
    pub ledger_index: Option<u32>,
    pub engine_result: String,
    pub engine_result_message: String,
    pub engine_result_code: i32,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMetadata {
    pub transaction_type: String,
    pub account: Address,
    pub payment: Option<PaymentDetails>,
    pub fee: Fee,
    pub sequence: Sequence,
    pub hash: TransactionHash,
    pub ledger_index: u32,
    pub date: Timestamp,
    pub validated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub account_data: AccountData,
    pub ledger_current_index: u32,
    pub validated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountData {
    pub account: Address,
    pub balance: Amount,
    pub flags: u32,
    pub ledger_entry_type: String,
    pub owner_count: u32,
    pub previous_txn_id: Option<TransactionHash>,
    pub previous_txn_lgr_seq: Option<u32>,
    pub sequence: Sequence,
    pub transfer_rate: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XrplRequest {
    pub method: String,
    pub params: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XrplResponse<T> {
    pub result: T,
    pub status: String,
    pub type_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustLine {
    pub account: Address,
    pub balance: Amount,
    pub currency: CurrencyCode,
    pub limit: Amount,
    pub limit_peer: Amount,
    pub quality_in: u32,
    pub quality_out: u32,
    pub no_ripple: bool,
    pub no_ripple_peer: bool,
    pub authorized: bool,
    pub peer_authorized: bool,
    pub freeze: bool,
    pub freeze_peer: bool,
    pub obligation: Option<Amount>,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            transaction_type: "Payment".to_string(),
            account: String::new(),
            destination: String::new(),
            amount: String::new(),
            currency: String::new(),
            issuer: None,
            fee: "12".to_string(), // Default fee in drops
            sequence: 0,
            flags: None,
            last_ledger_sequence: None,
            source_tag: None,
            destination_tag: None,
            invoice_id: None,
            paths: None,
            send_max: None,
            deliver_min: None,
        }
    }
}
