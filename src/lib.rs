pub mod error;
pub mod types;
pub mod client;
pub mod transaction;
pub mod signing;

pub use error::XrplError;
pub use types::*;
pub use client::XrplClient;
pub use transaction::*;
pub use signing::*;

use anyhow::Result;

pub struct XrplLib {
    client: XrplClient,
}

impl XrplLib {
    pub fn new(testnet: bool) -> Self {
        let client = XrplClient::new(testnet);
        Self { client }
    }

    /// Send a token (issued asset) from user1 to user2
    /// 
    /// # Arguments
    /// * `user1_secret` - The secret key of the sender
    /// * `user2_address` - The address of the recipient
    /// * `issuer_address` - The address of the token issuer
    /// * `currency_code` - The currency code of the token
    /// * `amount` - The amount to transfer
    pub async fn send_token(
        &self,
        user1_secret: &str,
        user2_address: &str,
        issuer_address: &str,
        currency_code: &str,
        amount: &str,
    ) -> Result<TransactionResult> {
        let transaction = self.client.create_payment_transaction(
            user1_secret,
            user2_address,
            issuer_address,
            currency_code,
            amount,
        )?;

        let signed_tx = self.sign_transaction_offline(user1_secret, &transaction)?;
        self.submit_signed_transaction(&signed_tx).await
    }

    /// Verify that user1 sent a token to user2
    /// 
    /// # Arguments
    /// * `user1_address` - The address of the sender
    /// * `user2_address` - The address of the recipient
    /// * `issuer_address` - The address of the token issuer
    /// * `currency_code` - The currency code of the token
    /// * `amount` - The expected amount
    /// * `tx_hash` - The transaction hash to verify
    pub async fn verify_token_transfer(
        &self,
        user1_address: &str,
        user2_address: &str,
        issuer_address: &str,
        currency_code: &str,
        amount: &str,
        tx_hash: &str,
    ) -> Result<bool> {
        let tx_data = self.client.get_transaction(tx_hash).await?;
        
        if tx_data.transaction_type != "Payment" {
            return Ok(false);
        }

        if let Some(payment) = tx_data.payment {
            let amount_matches = payment.amount == amount;
            let currency_matches = payment.currency == currency_code;
            let issuer_matches = payment.issuer.as_deref() == Some(issuer_address);
            let sender_matches = tx_data.account == user1_address;
            let destination_matches = payment.destination == user2_address;

            Ok(amount_matches && currency_matches && issuer_matches && 
               sender_matches && destination_matches)
        } else {
            Ok(false)
        }
    }

    /// Sign a transfer transaction offline (produce a signed blob, but don't submit)
    /// 
    /// # Arguments
    /// * `secret` - The secret key to sign with
    /// * `transaction` - The transaction to sign
    pub fn sign_transaction_offline(
        &self,
        secret: &str,
        transaction: &Transaction,
    ) -> Result<SignedTransaction> {
        let signer = TransactionSigner::new();
        signer.sign_transaction(secret, transaction)
    }

    /// Submit a signed transaction using a different wallet/connection
    /// 
    /// # Arguments
    /// * `signed_tx` - The signed transaction to submit
    pub async fn submit_signed_transaction(
        &self,
        signed_tx: &SignedTransaction,
    ) -> Result<TransactionResult> {
        self.client.submit_transaction(signed_tx).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_library_creation() {
        let lib = XrplLib::new(true);
        assert!(lib.client.is_testnet());
    }
}
