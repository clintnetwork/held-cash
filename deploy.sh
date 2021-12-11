#!/bin/bash
# Held.Cash Deploy Script for DevNet
# DevNet Temporary Address: HeLD7sSTdmTmnajapRaCGnZAUBrN4M8NDbThwMkQrThj
# solana airdrop 0.5 HeLD7sSTdmTmnajapRaCGnZAUBrN4M8NDbThwMkQrThj --url https://api.devnet.solana.com
cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=dist/ && solana program deploy --keypair ./held-cash-keypair.json dist/held_cash.so --program-id ./program-id.json --url https://api.devnet.solana.com
