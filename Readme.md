# FairSelection: FHE-Powered Hidden State Engine for On-chain Games

## 🎮 Vision
**FairSelection** is a specialized privacy primitive designed to solve the "Transparency Trap" in decentralized gaming. In strategy, card, and social deduction games, exposing hands, player positions, or selection seeds leads to game-breaking MEV (Maximum Extractable Value) and cheating. 

By leveraging **Fully Homomorphic Encryption (FHE)** via Arcium, FairSelection allows game logic to be computed in an encrypted state, ensuring that "Hidden Information" stays truly hidden until the rules dictate a reveal.

## 🛡️ Privacy Benefits
- **Shielded State Computation:** All selection logic (e.g., dealing cards, determining loot drops) occurs within Arcium’s Multi-Party Execution (MXE) environment.
- **Anti-Predictive Gameplay:** Prevents validators or bots from inspecting the mempool to predict outcomes of "random" events.
- **Verifiable Fairness:** Players can verify the integrity of the selection without the server or any single node ever seeing the raw entropy or user data.

## 🧠 Technical Implementation
The project is split into two specialized layers:

### 1. The Confidential Circuit (`/encrypted-ixs`)
The core selection logic is written in Arcis Rust. It implements `execute_secure_selection`, which:
- Maps encrypted entropy seeds to a finite candidate pool using modulo operations.
- Deducts a minimal 0.1% maintenance fee entirely on ciphertexts.
- Ensures the selection index is never leaked during the calculation phase.

### 2. The Protocol Ledger (`/programs/fair_pool`)
A Solana Anchor program that acts as the "Rule Enforcer." It manages:
- **Protocol Initialization:** Setting up the link between Solana and Arcium.
- **Encrypted Requests:** Securely queuing game moves for private computation.
- **Verified Callbacks:** Receiving and acting upon the finalized (decrypted) game outcomes.

## 🌟 Strategic Use Cases
- **Secret Hand Dealing:** In on-chain Poker or TCGs, cards are dealt such that only the recipient and the rule-engine know the value.
- **Fog of War:** Strategic positions in 4X games are computed privately; players only "see" what their encrypted units detect.
- **Hidden Inventory:** In RPGs, player loot can remain confidential to prevent targeted theft or meta-gaming.

## 📜 Project Structure
```text
.
├── encrypted-ixs/          # FHE Circuit Logic (The "Brain")
│   └── src/lib.rs          # execute_secure_selection implementation
├── programs/fair_pool/     # Solana Ledger Logic (The "Enforcer")
│   └── src/lib.rs          # Protocol management and callbacks
└── README.md               # Documentation
```

## 📄 License
MIT License. Developed for the Arcium RTG #0.1 Hidden-Information Games Track.