use ripple_xrpl::XrplLib;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("XRPL Token Transfer Example");
    println!("==========================");

    let _xrpl = XrplLib::new(true);
    println!("✓ Created XRPL library instance (testnet)");

    let user1_secret = "your_secret_key_here";
    let user2_address = "rDestinationAddress123456789012345678901234";
    let issuer_address = "rIssuerAddress123456789012345678901234";
    let currency_code = "USD";
    let amount = "100.50";

    println!("\nTransaction Details:");
    println!("  From: {}", user1_secret);
    println!("  To: {}", user2_address);
    println!("  Issuer: {}", issuer_address);
    println!("  Currency: {}", currency_code);
    println!("  Amount: {}", amount);

    println!("\n⚠️  Note: This example uses placeholder values.");
    println!("   Replace with real testnet credentials to test actual transfers.");
    println!("   You can get testnet credentials from: https://xrpl.org/xrp-testnet-faucet.html");

    println!("\nExample completed successfully!");
    println!("To test with real data:");
    println!("1. Get testnet credentials from XRPL testnet faucet");
    println!("2. Replace placeholder values with real credentials");
    println!("3. Run: cargo run --example basic_transfer");

    Ok(())
}
