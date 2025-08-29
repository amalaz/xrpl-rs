# Ripple XRPL Rust Library

A Rust library for XRPL (Ripple) operations including token transfers, transaction signing, and verification.

## Features

- **Token Transfers**: Send issued assets on XRPL Testnet
- **Transaction Verification**: Verify token transfers between users
- **Offline Signing**: Sign transactions offline without submitting
- **Transaction Submission**: Submit signed transactions using different connections
- **Multi-signature Support**: Create and manage multi-signature transactions
- **Comprehensive Validation**: Validate addresses, amounts, and transaction data

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ripple-xrpl = "0.1.0"
```

## Usage

### Basic Setup

```rust
use ripple_xrpl::XrplLib;

// Create a library instance for testnet
let xrpl = XrplLib::new(true);
```

### Part 1: Token Transfer and Verification

#### 1. Send a Token

```rust
// Send a token from user1 to user2
let result = xrpl.send_token(
    "user1_secret_key",
    "rUser2Address123456789012345678901234",
    "rIssuerAddress123456789012345678901234",
    "USD",
    "100.50"
).await?;

println!("Transaction hash: {}", result.hash);
```

#### 2. Verify Token Transfer

```rust
// Verify that user1 sent the token to user2
let is_valid = xrpl.verify_token_transfer(
    "rUser1Address123456789012345678901234",
    "rUser2Address123456789012345678901234",
    "rIssuerAddress123456789012345678901234",
    "USD",
    "100.50",
    "transaction_hash_here"
).await?;

if is_valid {
    println!("Token transfer verified successfully!");
} else {
    println!("Token transfer verification failed!");
}
```

### Part 2: Offline Signing and Submission

#### 3. Sign Transaction Offline

```rust
// Sign a transaction offline (produces signed blob, doesn't submit)
let signed_tx = xrpl.sign_transaction_offline(
    "user_secret_key",
    &transaction
)?;

println!("Signed transaction blob: {}", signed_tx.tx_blob);
```

#### 4. Submit Signed Transaction

```rust
// Submit the signed transaction using a different wallet/connection
let result = xrpl.submit_signed_transaction(&signed_tx).await?;

println!("Transaction submitted: {}", result.hash);
```

### Advanced Usage

#### Transaction Building

```rust
use ripple_xrpl::TransactionBuilder;

let builder = TransactionBuilder::new(true);
let transaction = builder.build_payment_transaction(
    "rAccount123",
    "rDestination456",
    "100",
    "USD",
    Some("rIssuer789"),
    Some("12"),
    1,
    Some(1000),
)?;
```

#### Transaction Validation

```rust
use ripple_xrpl::TransactionValidator;

// Validate transaction hash
TransactionValidator::validate_transaction_hash("hash_here")?;

// Validate address
TransactionValidator::validate_address("rAddress123")?;

// Validate currency code
TransactionValidator::validate_currency_code("USD")?;

// Validate amount
TransactionValidator::validate_amount("100.50")?;
```

#### Multi-signature Transactions

```rust
use ripple_xrpl::TransactionSigner;

let signer = TransactionSigner::new();
let signatures = vec![
    ("public_key_1".to_string(), "signature_1".to_string()),
    ("public_key_2".to_string(), "signature_2".to_string()),
];

let multisig_tx = signer.create_multisig_transaction(&transaction, signatures)?;
```

## API Reference

### Main Library (`XrplLib`)

- `new(testnet: bool)` - Create new instance
- `send_token(...)` - Send token transfer
- `verify_token_transfer(...)` - Verify token transfer
- `sign_transaction_offline(...)` - Sign transaction offline
- `submit_signed_transaction(...)` - Submit signed transaction

### Transaction Builder (`TransactionBuilder`)

- `build_payment_transaction(...)` - Build payment transaction
- `build_trust_set_transaction(...)` - Build trust set transaction
- `validate_transaction(...)` - Validate transaction
- `transaction_to_json(...)` - Convert to JSON format

### Transaction Validator (`TransactionValidator`)

- `validate_transaction_hash(...)` - Validate transaction hash
- `validate_address(...)` - Validate XRPL address
- `validate_currency_code(...)` - Validate currency code
- `validate_amount(...)` - Validate amount format

### Transaction Signer (`TransactionSigner`)

- `sign_transaction(...)` - Sign transaction offline
- `verify_transaction(...)` - Verify signed transaction
- `create_multisig_transaction(...)` - Create multi-signature transaction

## Error Handling

The library uses custom error types for different failure scenarios:

```rust
use ripple_xrpl::XrplError;

match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(XrplError::Network(msg)) => println!("Network error: {}", msg),
    Err(XrplError::InvalidSecret(msg)) => println!("Invalid secret: {}", msg),
    Err(XrplError::TransactionFailed(msg)) => println!("Transaction failed: {}", msg),
    Err(e) => println!("Other error: {:?}", e),
}
```

## Testing

Run the test suite:

```bash
cargo test
```

## Examples

See the `examples/` directory for complete working examples:

- `basic_transfer.rs` - Basic token transfer
- `offline_signing.rs` - Offline transaction signing
- `multisig.rs` - Multi-signature transactions

## Network Configuration

- **Testnet**: Uses `https://s.altnet.rippletest.net:51234`
- **Mainnet**: Uses `https://xrplcluster.com`

## Security Notes

- This is a simplified implementation for educational purposes
- In production, use proper XRPL libraries with full cryptographic implementations
- Always validate inputs and handle errors appropriately
- Keep secret keys secure and never expose them in code

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Disclaimer

This library is provided as-is for educational and development purposes. Use at your own risk in production environments.
