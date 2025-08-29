use crate::error::XrplError;
use crate::types::*;
use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};

pub struct XrplClient {
    client: Client,
    base_url: String,
    testnet: bool,
}

impl XrplClient {
    pub fn new(testnet: bool) -> Self {
        let base_url = if testnet {
            "https://s.altnet.rippletest.net:51234".to_string()
        } else {
            "https://xrplcluster.com".to_string()
        };

        Self {
            client: Client::new(),
            base_url,
            testnet,
        }
    }

    pub fn is_testnet(&self) -> bool {
        self.testnet
    }

    pub async fn get_ledger_index(&self) -> Result<u32> {
        let request = json!({
            "method": "ledger",
            "params": [{
                "ledger_index": "validated"
            }]
        });

        let response: Value = self.make_request(&request).await?;
        
        response["result"]["ledger_index"]
            .as_u64()
            .map(|v| v as u32)
            .ok_or_else(|| XrplError::ApiError("Invalid ledger response".to_string()).into())
    }

    pub async fn get_account_info(&self, address: &str) -> Result<AccountInfo> {
        let request = json!({
            "method": "account_info",
            "params": [{
                "account": address,
                "ledger_index": "validated"
            }]
        });

        let response: Value = self.make_request(&request).await?;
        
        if let Some(error) = response["result"]["error"].as_str() {
            return Err(XrplError::ApiError(error.to_string()).into());
        }

        serde_json::from_value(response["result"].clone())
            .map_err(|e| XrplError::Deserialization(e.to_string()).into())
    }

    pub async fn get_account_sequence(&self, address: &str) -> Result<u32> {
        let account_info = self.get_account_info(address).await?;
        Ok(account_info.account_data.sequence)
    }

    pub async fn get_transaction(&self, tx_hash: &str) -> Result<TransactionMetadata> {
        let request = json!({
            "method": "tx",
            "params": [{
                "transaction": tx_hash,
                "binary": false
            }]
        });

        let response: Value = self.make_request(&request).await?;
        
        if let Some(error) = response["result"]["error"].as_str() {
            return Err(XrplError::ApiError(error.to_string()).into());
        }

        serde_json::from_value(response["result"].clone())
            .map_err(|e| XrplError::Deserialization(e.to_string()).into())
    }

    pub fn create_payment_transaction(
        &self,
        user1_secret: &str,
        user2_address: &str,
        issuer_address: &str,
        currency_code: &str,
        amount: &str,
    ) -> Result<Transaction> {
        let public_key = self.secret_to_public_key(user1_secret)?;
        let user1_address = self.public_key_to_address(&public_key)?;

        let mut transaction = Transaction::default();
        transaction.account = user1_address;
        transaction.destination = user2_address.to_string();
        transaction.amount = amount.to_string();
        transaction.currency = currency_code.to_string();
        transaction.issuer = Some(issuer_address.to_string());

        Ok(transaction)
    }

    pub async fn submit_transaction(&self, signed_tx: &SignedTransaction) -> Result<TransactionResult> {
        let request = json!({
            "method": "submit",
            "params": [{
                "tx_blob": signed_tx.tx_blob
            }]
        });

        let response: Value = self.make_request(&request).await?;
        
        if let Some(error) = response["result"]["error"].as_str() {
            return Err(XrplError::ApiError(error.to_string()).into());
        }

        let result = &response["result"];
        
        Ok(TransactionResult {
            hash: result["tx_json"]["hash"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            validated: result["validated"].as_bool().unwrap_or(false),
            ledger_index: result["ledger_index"].as_u64().map(|v| v as u32),
            engine_result: result["engine_result"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            engine_result_message: result["engine_result_message"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            engine_result_code: result["engine_result_code"]
                .as_i64()
                .unwrap_or(0) as i32,
            meta: Some(result["meta"].clone()),
        })
    }

    async fn make_request(&self, request: &Value) -> Result<Value> {
        let response = self
            .client
            .post(&self.base_url)
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(XrplError::Network(format!(
                "HTTP error: {}",
                response.status()
            ))
            .into());
        }

        let response_data: Value = response.json().await?;
        Ok(response_data)
    }

    fn secret_to_public_key(&self, secret: &str) -> Result<String> {
        if secret.len() < 32 {
            return Err(XrplError::InvalidSecret("Secret too short".to_string()).into());
        }
        
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        let result = hasher.finalize();
        
        Ok(hex::encode(result))
    }

    fn public_key_to_address(&self, public_key: &str) -> Result<String> {
        if public_key.len() < 64 {
            return Err(XrplError::InvalidAddress("Invalid public key".to_string()).into());
        }
        
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(public_key.as_bytes());
        let result = hasher.finalize();
        
        Ok(format!("r{}", hex::encode(&result[..20])))
    }

    pub async fn get_account_balance(&self, address: &str) -> Result<String> {
        let account_info = self.get_account_info(address).await?;
        Ok(account_info.account_data.balance)
    }

    pub async fn get_trust_lines(&self, address: &str) -> Result<Vec<TrustLine>> {
        let request = json!({
            "method": "account_lines",
            "params": [{
                "account": address,
                "ledger_index": "validated"
            }]
        });

        let response: Value = self.make_request(&request).await?;
        
        if let Some(error) = response["result"]["error"].as_str() {
            return Err(XrplError::ApiError(error.to_string()).into());
        }

        let lines = &response["result"]["lines"];
        if lines.is_array() {
            serde_json::from_value(lines.clone())
                .map_err(|e| XrplError::Deserialization(e.to_string()).into())
        } else {
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = XrplClient::new(true);
        assert!(client.is_testnet());
        assert!(client.base_url.contains("altnet.rippletest.net"));
    }

    #[test]
    fn test_mainnet_client() {
        let client = XrplClient::new(false);
        assert!(!client.is_testnet());
        assert!(client.base_url.contains("xrplcluster.com"));
    }
}
