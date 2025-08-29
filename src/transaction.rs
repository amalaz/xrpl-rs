use crate::error::XrplError;
use crate::types::*;
use anyhow::Result;
use serde_json::{json, Value};

/// Transaction builder for creating XRPL transactions
pub struct TransactionBuilder {
    testnet: bool,
}

impl TransactionBuilder {
    pub fn new(testnet: bool) -> Self {
        Self { testnet }
    }

    pub fn build_payment_transaction(
        &self,
        account: &str,
        destination: &str,
        amount: &str,
        currency: &str,
        issuer: Option<&str>,
        fee: Option<&str>,
        sequence: u32,
        last_ledger_sequence: Option<u32>,
    ) -> Result<Transaction> {
        let mut transaction = Transaction::default();
        
        transaction.account = account.to_string();
        transaction.destination = destination.to_string();
        transaction.amount = amount.to_string();
        transaction.currency = currency.to_string();
        transaction.issuer = issuer.map(|i| i.to_string());
        transaction.fee = fee.unwrap_or("12").to_string();
        transaction.sequence = sequence;
        transaction.last_ledger_sequence = last_ledger_sequence;
        
        transaction.flags = Some(0x00020000);
        
        Ok(transaction)
    }

    pub fn build_trust_set_transaction(
        &self,
        account: &str,
        currency: &str,
        issuer: &str,
        limit: &str,
        fee: Option<&str>,
        sequence: u32,
        last_ledger_sequence: Option<u32>,
    ) -> Result<Transaction> {
        let mut transaction = Transaction::default();
        
        transaction.transaction_type = "TrustSet".to_string();
        transaction.account = account.to_string();
        transaction.fee = fee.unwrap_or("12").to_string();
        transaction.sequence = sequence;
        transaction.last_ledger_sequence = last_ledger_sequence;
        
        transaction.amount = limit.to_string();
        transaction.currency = currency.to_string();
        transaction.issuer = Some(issuer.to_string());
        
        Ok(transaction)
    }

    pub fn validate_transaction(&self, transaction: &Transaction) -> Result<()> {
        if transaction.account.is_empty() {
            return Err(XrplError::InvalidTransaction("Account is required".to_string()).into());
        }

        if transaction.destination.is_empty() {
            return Err(XrplError::InvalidTransaction("Destination is required".to_string()).into());
        }

        if transaction.amount.is_empty() {
            return Err(XrplError::InvalidTransaction("Amount is required".to_string()).into());
        }

        if transaction.currency.is_empty() {
            return Err(XrplError::InvalidTransaction("Currency is required".to_string()).into());
        }

        if transaction.fee.is_empty() {
            return Err(XrplError::InvalidTransaction("Fee is required".to_string()).into());
        }

        if let Err(_) = transaction.amount.parse::<f64>() {
            return Err(XrplError::InvalidAmount("Invalid amount format".to_string()).into());
        }

        if let Err(_) = transaction.fee.parse::<u32>() {
            return Err(XrplError::InvalidTransaction("Invalid fee format".to_string()).into());
        }

        Ok(())
    }

    pub fn transaction_to_json(&self, transaction: &Transaction) -> Result<Value> {
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

        Ok(tx_json)
    }

    pub fn get_network_id(&self) -> u32 {
        if self.testnet {
            1024 // Testnet network ID
        } else {
            1049344 // Mainnet network ID
        }
    }
}

pub struct TransactionValidator;

impl TransactionValidator {
    pub fn validate_transaction_hash(hash: &str) -> Result<()> {
        if hash.len() != 64 {
            return Err(XrplError::InvalidTransaction("Invalid transaction hash length".to_string()).into());
        }

        if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(XrplError::InvalidTransaction("Invalid transaction hash format".to_string()).into());
        }

        Ok(())
    }

    pub fn validate_address(address: &str) -> Result<()> {
        if !address.starts_with('r') {
            return Err(XrplError::InvalidAddress("Address must start with 'r'".to_string()).into());
        }

        if address.len() < 25 || address.len() > 45 {
            return Err(XrplError::InvalidAddress("Invalid address length".to_string()).into());
        }

        if !address.chars().all(|c| c.is_alphanumeric()) {
            return Err(XrplError::InvalidAddress("Address contains invalid characters".to_string()).into());
        }

        Ok(())
    }

    pub fn validate_currency_code(currency: &str) -> Result<()> {
        if currency.is_empty() {
            return Err(XrplError::InvalidCurrency("Currency code cannot be empty".to_string()).into());
        }

        if currency.len() > 20 {
            return Err(XrplError::InvalidCurrency("Currency code too long".to_string()).into());
        }

        if currency.len() == 40 {
            if !currency.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(XrplError::InvalidCurrency("Invalid hex currency format".to_string()).into());
            }
        } else {
            if !currency.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
                return Err(XrplError::InvalidCurrency("Invalid currency code format".to_string()).into());
            }
        }

        Ok(())
    }

    pub fn validate_amount(amount: &str) -> Result<()> {
        if amount.is_empty() {
            return Err(XrplError::InvalidAmount("Amount cannot be empty".to_string()).into());
        }

        if let Err(_) = amount.parse::<f64>() {
            return Err(XrplError::InvalidAmount("Invalid amount format".to_string()).into());
        }

        if amount.starts_with('-') {
            return Err(XrplError::InvalidAmount("Amount cannot be negative".to_string()).into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_builder_creation() {
        let builder = TransactionBuilder::new(true);
        assert!(builder.testnet);
    }

    #[test]
    fn test_payment_transaction_building() {
        let builder = TransactionBuilder::new(true);
        let tx = builder.build_payment_transaction(
            "rAccount123",
            "rDestination456",
            "100",
            "USD",
            Some("rIssuer789"),
            Some("12"),
            1,
            Some(1000),
        ).unwrap();

        assert_eq!(tx.account, "rAccount123");
        assert_eq!(tx.destination, "rDestination456");
        assert_eq!(tx.amount, "100");
        assert_eq!(tx.currency, "USD");
        assert_eq!(tx.issuer, Some("rIssuer789".to_string()));
        assert_eq!(tx.fee, "12");
        assert_eq!(tx.sequence, 1);
        assert_eq!(tx.last_ledger_sequence, Some(1000));
    }

    #[test]
    fn test_transaction_validation() {
        let builder = TransactionBuilder::new(true);
        let tx = builder.build_payment_transaction(
            "rAccount123",
            "rDestination456",
            "100",
            "USD",
            Some("rIssuer789"),
            Some("12"),
            1,
            Some(1000),
        ).unwrap();

        assert!(builder.validate_transaction(&tx).is_ok());
    }

    #[test]
    fn test_address_validation() {
        assert!(TransactionValidator::validate_address("rAccount123456789012345678901234").is_ok());
        assert!(TransactionValidator::validate_address("invalid").is_err());
        assert!(TransactionValidator::validate_address("xAccount123").is_err());
    }

    #[test]
    fn test_currency_validation() {
        assert!(TransactionValidator::validate_currency_code("USD").is_ok());
        assert!(TransactionValidator::validate_currency_code("12345678901234567890").is_ok());
        assert!(TransactionValidator::validate_currency_code("").is_err());
    }

    #[test]
    fn test_amount_validation() {
        assert!(TransactionValidator::validate_amount("100").is_ok());
        assert!(TransactionValidator::validate_amount("100.50").is_ok());
        assert!(TransactionValidator::validate_amount("").is_err());
        assert!(TransactionValidator::validate_amount("-100").is_err());
    }
}
