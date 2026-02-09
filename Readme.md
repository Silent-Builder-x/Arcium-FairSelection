# FairSelection: FHE-Powered Confidential Resource Allocation

## 🎰 Overview

**FairSelection** is a decentralized protocol built on **Arcium** and **Solana** that provides mathematically verifiable, unbiased, and confidential randomness for resource allocation.

By executing the selection logic (Modulo Mapping) entirely within the **Multi-Party Execution (MXE)** environment using **Fully Homomorphic Encryption (FHE)**, the protocol ensures that neither node operators nor miners can observe or manipulate the outcome before it is finalized on the ledger.

## 🚀 Live Deployment Status (Devnet)

The protocol has been successfully built and deployed to the Arcium Devnet.

- **MXE Address:** `EicCcGkRCvRWEXzRN2pKHvTfz33be6erxeCt8shDsHbX`
- **MXE Program ID:** `FaAWo6F9CVDDD6eKyiHbhN7DKXWKrpFrutJMWr7XW7Zx`
- **Authority:** `AjUstj3Qg296mz6DFcXAg186zRvNKuFfjB7JK2Z6vS7R`
- **Cluster:** `DzaQCyfybroycrNqE5Gk7LhSbWD2qfCics6qptBFbr95`
- **Status:** `Active`

## 🧠 Core Innovation

- **Confidential Modulo Mapping:** Maps high-entropy system seeds into a finite candidate space. The "Winner Index" remains encrypted during the entire computation.
- **Automated Protocol Economics:** Hardcoded 0.1% maintenance fee calculated on ciphertexts, ensuring the sustainability of the protocol without revealing treasury volumes.
- **MEV Resistance:** Eliminates front-running risks by hiding the "intent" and "result" of the allocation from the public mempool.

## 🛠 Build & Deploy Instructions

### Prerequisites

- Solana Agave Toolsuite (v3.1.8+)
- Arcium CLI
- Rust (Stable)

### 1. Build the Artifacts

This command compiles the Arcis circuit logic and the Solana Anchor program.

```
arcium build

```

### 2. Deploy to Network

Deploy the compiled MXE and Program to the Arcium Devnet cluster.

```
arcium deploy --cluster-offset 456 --recovery-set-size 4 --keypair-path ~/.config/solana/id.json -u d

```

## 📄 Technical Specification

- **Encrypted IX:** `execute_secure_selection`
- **Callback Pattern:** Verified MXE execution with secure on-chain settlement.
- **Security:** Protected by Arcium's Recovery Set and Homomorphic encryption.