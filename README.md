# Held.cash

Multisig Vault & Escrow Program build on Solana, written in Rust 🦀

## Escrow

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct VaultDetails {
    // Owner's public key
    pub owner: Pubkey,

    // Multisig cosigners
    pub cosigners: [Pubkey],

    // Number of required signatures
    pub requires: i32,
}
```
