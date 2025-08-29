use ripple_xrpl::{XrplLib, TransactionBuilder, TransactionSigner, TransactionValidator};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("XRPL Offline Transaction Signing Example");
    println!("========================================");

    let _xrpl = XrplLib::new(true);
    println!("✓ Created XRPL library instance (testnet)");

    let builder = TransactionBuilder::new(true);
    println!("✓ Created transaction builder");

    let account = "rSourceAccount123456789012345678901234";
    let destination = "rDestinationAccount123456789012345678901234";
    let amount = "50.00";
    let currency = "USD";
    let issuer = Some("rIssuerAccount123456789012345678901234");
    let fee = "12";
    let sequence = 1;
    let last_ledger_sequence = Some(1000);

    println!("\nTransaction Parameters:");
    println!("  Account: {}", account);
    println!("  Destination: {}", destination);
    println!("  Amount: {}", amount);
    println!("  Currency: {}", currency);
    println!("  Issuer: {:?}", issuer);
    println!("  Fee: {}", fee);
    println!("  Sequence: {}", sequence);
    println!("  Last Ledger Sequence: {:?}", last_ledger_sequence);

    let transaction = builder.build_payment_transaction(
        account,
        destination,
        amount,
        currency,
        issuer,
        Some(fee),
        sequence,
        last_ledger_sequence,
    )?;
    println!("✓ Built payment transaction");

    builder.validate_transaction(&transaction)?;
    println!("✓ Transaction validation passed");

    TransactionValidator::validate_address(account)?;
    TransactionValidator::validate_address(destination)?;
    TransactionValidator::validate_currency_code(currency)?;
    TransactionValidator::validate_amount(amount)?;
    println!("✓ All transaction components validated");

    let tx_json = builder.transaction_to_json(&transaction)?;
    println!("✓ Transaction converted to JSON format");
    println!("  JSON: {}", serde_json::to_string_pretty(&tx_json)?);

    let _signer = TransactionSigner::new();
    println!("✓ Created transaction signer");

    let _secret_key = "your_secret_key_here";
    println!("\n⚠️  Note: Using placeholder secret key");
    println!("   Replace with real key to test actual signing");

    println!("\nExample completed successfully!");
    println!("To test with real data:");
    println!("1. Replace placeholder values with real credentials");
    println!("2. Run: cargo run --example offline_signing");

    Ok(())
}
