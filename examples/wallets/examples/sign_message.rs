/// Atipicial Message Signing Example
///
/// This example demonstrates the concept of message signing in Atipicial
/// using educational content instead of actual cryptographic operations.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🔐 Atipicial Message Signing Example");
	println!("==============================");

	println!("\n📚 Understanding Message Signing in Atipicial:");
	println!("   • Digital signatures prove message authenticity");
	println!("   • Uses ECDSA with secp256r1 curve");
	println!("   • Sign with private key, verify with public key");
	println!("   • Essential for secure off-chain communications");

	// 1. Account-based signing concepts
	println!("\n1. Account-Based Message Signing:");
	println!("   🔑 Private Key Usage:");
	println!("     • Never expose private keys");
	println!("     • Use secure key management");
	println!("     • Consider hardware wallets for sensitive operations");

	println!("\n   📍 Address Verification:");
	println!("     • Public key derives to Atipicial address");
	println!("     • Verify signer identity through address");
	println!("     • Check address format and validity");

	// 2. Message preparation
	let message = "Hello, Atipicial blockchain! This is a signed message.";
	println!("\n2. Message Preparation:");
	println!("   📝 Message: \"{message}\"");
	println!("   🔐 Process:");
	println!("     1. Convert message to bytes");
	println!("     2. Hash the message bytes (SHA256)");
	println!("     3. Sign the hash (not the raw message)");
	println!("     4. Encode signature for transmission");

	// 3. Signing process concepts
	println!("\n3. Digital Signature Process:");
	println!("   ⚡ ECDSA Signature Generation:");
	println!("     • Input: Private key + Message hash");
	println!("     • Output: 64-byte signature (r, s values)");
	println!("     • Deterministic: Same input = Same signature");
	println!("     • Non-forgeable without private key");

	println!("\n   📏 Signature Format:");
	println!("     • Raw bytes: 64 bytes (32 + 32)");
	println!("     • Hex encoding: 128 characters");
	println!("     • Base64 encoding: ~88 characters");
	println!("     • DER encoding: Variable length");

	// 4. Verification concepts
	println!("\n4. Signature Verification:");
	println!("   ✅ Verification Process:");
	println!("     1. Extract public key from account");
	println!("     2. Hash the original message");
	println!("     3. Verify signature against hash");
	println!("     4. Confirm public key matches expected address");

	println!("\n   🔒 Security Properties:");
	println!("     • Authenticity: Proves message origin");
	println!("     • Integrity: Detects message tampering");
	println!("     • Non-repudiation: Signer cannot deny");
	println!("     • Immutability: Signature tied to exact message");

	// 5. Message package structure
	println!("\n5. Verifiable Message Package Structure:");
	println!("   📦 Complete Package Contains:");
	println!("   {{");
	println!("     \"message\": \"Original message text\",");
	println!("     \"signature\": \"hex_encoded_signature\",");
	println!("     \"public_key\": \"compressed_public_key\",");
	println!("     \"address\": \"NXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx\",");
	println!("     \"timestamp\": 1234567890");
	println!("   }}");

	// 6. Different message types
	println!("\n6. Common Message Types in Atipicial:");

	println!("\n   📋 JSON Messages:");
	println!("     • Transaction proposals");
	println!("     • Smart contract interactions");
	println!("     • API requests with authentication");
	println!("     • Structured data exchanges");

	println!("\n   💾 Binary Data:");
	println!("     • File integrity verification");
	println!("     • Contract deployment verification");
	println!("     • Image or document signing");
	println!("     • Protocol-level messages");

	println!("\n   ⏰ Timestamped Messages:");
	println!("     • Time-sensitive operations");
	println!("     • Audit trail creation");
	println!("     • Session management");
	println!("     • Replay attack prevention");

	// 7. Use cases
	println!("\n7. 🎯 Real-World Use Cases:");

	println!("\n   🏦 Financial Applications:");
	println!("     • Transaction authorization");
	println!("     • Payment confirmations");
	println!("     • Account ownership proof");
	println!("     • Regulatory compliance");

	println!("\n   🔐 Authentication Systems:");
	println!("     • Login without passwords");
	println!("     • API access tokens");
	println!("     • Multi-factor authentication");
	println!("     • Session establishment");

	println!("\n   📄 Document Verification:");
	println!("     • Contract signing");
	println!("     • Certificate issuance");
	println!("     • Academic credentials");
	println!("     • Legal document authentication");

	println!("\n   🎮 Gaming Platforms:");
	println!("     • Score verification");
	println!("     • Achievement validation");
	println!("     • Item ownership proof");
	println!("     • Tournament integrity");

	// 8. Security best practices
	println!("\n8. 🛡️ Security Best Practices:");

	println!("\n   🔑 Key Management:");
	println!("     • Use secure random number generation");
	println!("     • Never reuse private keys");
	println!("     • Implement proper key rotation");
	println!("     • Use hardware security modules");

	println!("\n   📝 Message Handling:");
	println!("     • Always hash before signing");
	println!("     • Include nonces for replay protection");
	println!("     • Validate message format");
	println!("     • Implement timeout mechanisms");

	println!("\n   ✅ Verification Process:");
	println!("     • Always verify signatures before trust");
	println!("     • Check public key authenticity");
	println!("     • Validate address derivation");
	println!("     • Implement proper error handling");

	// 9. Common pitfalls
	println!("\n9. ⚠️ Common Pitfalls to Avoid:");
	println!("   • Signing raw messages instead of hashes");
	println!("   • Reusing signatures for different messages");
	println!("   • Not validating public key authenticity");
	println!("   • Exposing private keys in logs or errors");
	println!("   • Missing timestamp validation");
	println!("   • Inadequate entropy for key generation");

	// 10. Integration patterns
	println!("\n10. 🔧 Integration Patterns:");

	println!("\n   🌐 Web Applications:");
	println!("     • Browser wallet integration");
	println!("     • MetaMask-style signing");
	println!("     • QR code authentication");
	println!("     • Mobile wallet connections");

	println!("\n   📡 API Authentication:");
	println!("     • HTTP header signatures");
	println!("     • Request body signing");
	println!("     • JWT alternative approaches");
	println!("     • Webhook verification");

	println!("\n   🔗 Cross-Chain Interoperability:");
	println!("     • Bridge message authentication");
	println!("     • Multi-signature coordination");
	println!("     • Oracle data verification");
	println!("     • Layer 2 state validation");

	println!("\n🎉 Message signing concepts covered!");
	println!("💡 Key takeaways for secure message signing:");
	println!("   • Always sign hashes, never raw messages");
	println!("   • Verify both signature and public key authenticity");
	println!("   • Implement proper key management practices");
	println!("   • Include replay protection mechanisms");
	println!("   • Follow established cryptographic standards");

	println!("\n🚀 For working implementation examples:");
	println!("   • Atipicial cryptography documentation");
	println!("   • ECDSA signing libraries");
	println!("   • Message signing standards");

	Ok(())
}
