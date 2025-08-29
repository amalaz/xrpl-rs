use crate::error::XrplError;
use crate::types::*;
use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use sha2::{Sha512, Digest};
use serde_json::json;

/// Transaction signer for offline signing
pub struct TransactionSigner;

impl TransactionSigner {
    /// Create a new transaction signer
    pub fn new() -> Self {
        Self
    }

    /// Create a new transaction signer with network specification
    pub fn with_network(_testnet: bool) -> Self {
        Self
    }

    /// Sign a transaction offline (produce a signed blob, but don't submit)
    /// 
    /// # Arguments
    /// * `secret` - The secret key to sign with
    /// * `transaction` - The transaction to sign
    pub fn sign_transaction(
        &self,
        secret: &str,
        transaction: &Transaction,
    ) -> Result<SignedTransaction> {
        self.validate_transaction_for_signing(transaction)?;

        let signing_key = self.secret_to_signing_key(secret)?;
        let canonical_tx = self.transaction_to_canonical_format(transaction)?;
        let signature = self.sign_canonical_transaction(&signing_key, &canonical_tx)?;
        let tx_blob = self.create_signed_blob(transaction, &signature)?;
        let signed_tx = SignedTransaction {
            tx_blob,
            tx_json: transaction.clone(),
        };

        Ok(signed_tx)
    }

    /// Verify a signed transaction
    /// 
    /// # Arguments
    /// * `public_key` - The public key to verify with
    /// * `signed_tx` - The signed transaction to verify
    pub fn verify_transaction(
        &self,
        public_key: &str,
        signed_tx: &SignedTransaction,
    ) -> Result<bool> {
        let signature = self.extract_signature_from_blob(&signed_tx.tx_blob)?;
        let verifying_key = self.public_key_to_verifying_key(public_key)?;
        let canonical_tx = self.transaction_to_canonical_format(&signed_tx.tx_json)?;
        let is_valid = self.verify_signature(&verifying_key, &canonical_tx, &signature)?;

        Ok(is_valid)
    }

    /// Create a multi-signature transaction
    /// 
    /// # Arguments
    /// * `transaction` - The base transaction
    /// * `signatures` - Vector of (public_key, signature) pairs
    pub fn create_multisig_transaction(
        &self,
        transaction: &Transaction,
        signatures: Vec<(String, String)>,
    ) -> Result<SignedTransaction> {
        self.validate_transaction_for_signing(transaction)?;

        let tx_blob = self.create_multisig_blob(transaction, &signatures)?;
        let signed_tx = SignedTransaction {
            tx_blob,
            tx_json: transaction.clone(),
        };

        Ok(signed_tx)
    }

    /// Validate transaction for signing
    fn validate_transaction_for_signing(&self, transaction: &Transaction) -> Result<()> {
        if transaction.account.is_empty() {
            return Err(XrplError::InvalidTransaction("Account is required".to_string()).into());
        }

        if transaction.sequence == 0 {
            return Err(XrplError::InvalidTransaction("Sequence number is required".to_string()).into());
        }

        if transaction.fee.is_empty() {
            return Err(XrplError::InvalidTransaction("Fee is required".to_string()).into());
        }

        Ok(())
    }

    /// Convert secret key to Ed25519 signing key
    fn secret_to_signing_key(&self, secret: &str) -> Result<SigningKey> {
        if secret.len() < 32 {
            return Err(XrplError::InvalidSecret("Secret too short".to_string()).into());
        }

        let mut hasher = Sha512::new();
        hasher.update(secret.as_bytes());

        let key_bytes = hasher.finalize();
        let key_slice: [u8; 32] = key_bytes[..32].try_into()
            .map_err(|_| XrplError::SigningFailed("Invalid key length".to_string()))?;
        let signing_key = SigningKey::from_bytes(&key_slice);

        Ok(signing_key)
    }

    fn public_key_to_verifying_key(&self, public_key: &str) -> Result<VerifyingKey> {
        let key_bytes = hex::decode(public_key)
            .map_err(|e| XrplError::InvalidAddress(e.to_string()))?;

        let key_array: [u8; 32] = key_bytes.try_into()
            .map_err(|_| XrplError::SigningFailed("Invalid public key length".to_string()))?;
        let verifying_key = VerifyingKey::from_bytes(&key_array)
            .map_err(|e| XrplError::SigningFailed(e.to_string()))?;

        Ok(verifying_key)
    }

    fn transaction_to_canonical_format(&self, transaction: &Transaction) -> Result<Vec<u8>> {
        let mut tx_json = json!({
            "TransactionType": transaction.transaction_type,
            "Account": transaction.account,
            "Destination": transaction.destination,
            "Amount": transaction.amount,
            "Currency": transaction.currency,
            "Fee": transaction.fee,
            "Sequence": transaction.sequence,
        });

        if let Some(issuer) = &transaction.issuer {
            tx_json["Issuer"] = json!(issuer);
        }

        if let Some(flags) = transaction.flags {
            tx_json["Flags"] = json!(flags);
        }

        if let Some(last_ledger_sequence) = transaction.last_ledger_sequence {
            tx_json["LastLedgerSequence"] = json!(last_ledger_sequence);
        }

        if let Some(source_tag) = transaction.source_tag {
            tx_json["SourceTag"] = json!(source_tag);
        }

        if let Some(destination_tag) = transaction.destination_tag {
            tx_json["DestinationTag"] = json!(destination_tag);
        }

        if let Some(invoice_id) = &transaction.invoice_id {
            tx_json["InvoiceID"] = json!(invoice_id);
        }

        let canonical_json = serde_json::to_string(&tx_json)
            .map_err(|e| XrplError::Serialization(e.to_string()))?;

        Ok(canonical_json.into_bytes())
    }

    fn sign_canonical_transaction(
        &self,
        signing_key: &SigningKey,
        canonical_tx: &[u8],
    ) -> Result<Vec<u8>> {
        let signature = signing_key.sign(canonical_tx);
        
        Ok(signature.to_bytes().to_vec())
    }

    fn verify_signature(
        &self,
        verifying_key: &VerifyingKey,
        canonical_tx: &[u8],
        signature: &[u8],
    ) -> Result<bool> {
        let sig_array: [u8; 64] = signature.try_into()
            .map_err(|_| XrplError::SigningFailed("Invalid signature length".to_string()))?;
        let signature = Signature::try_from(&sig_array)
            .map_err(|e| XrplError::SigningFailed(e.to_string()))?;

        let is_valid = verifying_key.verify(canonical_tx, &signature).is_ok();

        Ok(is_valid)
    }

    fn create_signed_blob(
        &self,
        transaction: &Transaction,
        signature: &[u8],
    ) -> Result<String> {
        let mut blob_data = Vec::new();
        
        blob_data.extend_from_slice(transaction.transaction_type.as_bytes());
        blob_data.push(0);
        
        blob_data.extend_from_slice(transaction.account.as_bytes());
        blob_data.push(0);
        
        blob_data.extend_from_slice(transaction.destination.as_bytes());
        blob_data.push(0);
        
        blob_data.extend_from_slice(transaction.amount.as_bytes());
        blob_data.push(0);
        
        blob_data.extend_from_slice(transaction.currency.as_bytes());
        blob_data.push(0);
        
        blob_data.extend_from_slice(transaction.fee.as_bytes());
        blob_data.push(0);
        
        blob_data.extend_from_slice(transaction.sequence.to_string().as_bytes());
        blob_data.push(0);
        
        blob_data.extend_from_slice(signature);
        
        Ok(hex::encode(blob_data))
    }

    fn extract_signature_from_blob(&self, blob: &str) -> Result<Vec<u8>> {
        let blob_bytes = hex::decode(blob)
            .map_err(|e| XrplError::Serialization(e.to_string()))?;
        
        if blob_bytes.len() < 64 {
            return Err(XrplError::InvalidTransaction("Invalid blob format".to_string()).into());
        }
        
        let signature_start = blob_bytes.len() - 64;
        Ok(blob_bytes[signature_start..].to_vec())
    }

    fn create_multisig_blob(
        &self,
        transaction: &Transaction,
        signatures: &[(String, String)],
    ) -> Result<String> {
        let mut blob_data = Vec::new();
        
        let canonical_tx = self.transaction_to_canonical_format(transaction)?;
        blob_data.extend_from_slice(&canonical_tx);
        
        blob_data.extend_from_slice(&(signatures.len() as u32).to_le_bytes());
        
        for (public_key, signature) in signatures {
            let pk_bytes = hex::decode(public_key)
                .map_err(|e| XrplError::InvalidAddress(e.to_string()))?;
            blob_data.extend_from_slice(&(pk_bytes.len() as u32).to_le_bytes());
            blob_data.extend_from_slice(&pk_bytes);
            
            let sig_bytes = hex::decode(signature)
                .map_err(|e| XrplError::SigningFailed(e.to_string()))?;
            blob_data.extend_from_slice(&(sig_bytes.len() as u32).to_le_bytes());
            blob_data.extend_from_slice(&sig_bytes);
        }
        
        Ok(hex::encode(blob_data))
    }
}

impl Default for TransactionSigner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signer_creation() {
        let signer = TransactionSigner::new();
        // Verify the signer was created successfully
        assert!(std::mem::size_of_val(&signer) == 0); // Unit struct has size 0
    }

    #[test]
    fn test_signer_with_network() {
        let signer = TransactionSigner::with_network(true);
        // Verify the signer was created successfully
        assert!(std::mem::size_of_val(&signer) == 0); // Unit struct has size 0
    }

    #[test]
    fn test_transaction_validation() {
        let signer = TransactionSigner::new();
        let mut transaction = Transaction::default();
        transaction.account = "rAccount123".to_string();
        transaction.sequence = 1;
        transaction.fee = "12".to_string();
        
        assert!(signer.validate_transaction_for_signing(&transaction).is_ok());
    }

    #[test]
    fn test_invalid_transaction_validation() {
        let signer = TransactionSigner::new();
        let transaction = Transaction::default();
        
        assert!(signer.validate_transaction_for_signing(&transaction).is_err());
    }

    #[test]
    fn test_canonical_format() {
        let signer = TransactionSigner::new();
        let mut transaction = Transaction::default();
        transaction.account = "rAccount123".to_string();
        transaction.destination = "rDestination456".to_string();
        transaction.amount = "100".to_string();
        transaction.currency = "USD".to_string();
        transaction.fee = "12".to_string();
        transaction.sequence = 1;
        
        let canonical = signer.transaction_to_canonical_format(&transaction).unwrap();
        assert!(!canonical.is_empty());
    }
}
