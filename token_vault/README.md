## 🧾 Token Vault Staking (SPL Token) – Anchor Breakdown

---

### 1️⃣ Imports

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
```

**Purpose:**
Anchor framework + SPL Token CPI helpers.

**Analogy:**
Solana toolbox + token transfer machine.

---

### 2️⃣ Core Concept: Vault + UserStake

This contract has:

* **1 Vault** (global pool state)
* **Many UserStake PDAs** (each user’s staking diary)

---

### 3️⃣ Vault Initialization (`initialize`)

Creates:

* `vault` PDA (stores config + reward math)
* `vault_authority` PDA (signer)
* `token_account` (staking vault)
* `reward_vault` (reward pool)

**Analogy:**
A big box + a robot key + separate reward box.

---

### 4️⃣ Admin Funding (`fund_rewards`)

Admin transfers reward tokens into `reward_vault`.

**Purpose:**
Reward pool refill (no minting happens here).

---

### 5️⃣ User Staking (`stake`)

* updates rewards globally (`update_rewards`)
* updates user’s pending rewards
* transfers tokens into vault
* increases `total_staked`

**Analogy:**
User puts coins in the box + diary entry updated.

---

### 6️⃣ Claim Rewards (`claim`)

* calculates pending rewards
* transfers from `reward_vault` → user destination ATA

**Analogy:**
User collects prize from reward box.

---

### 7️⃣ Unstake (`unstake`)

* harvest rewards
* transfers staked tokens back to user
* decreases `total_staked`

---

### 8️⃣ Admin Withdraw (`withdraw`)

Authority can withdraw tokens from vault (if not locked).

⚠️ Trust-based admin power.

---

### 9️⃣ Lock / Unlock (`lock`, `unlock`)

Admin can lock vault until timestamp.

**Purpose:**
Pause withdrawals temporarily.

---

### One-line mental model

> **Users stake SPL tokens into a shared vault, rewards accumulate per second, and users claim from a pre-funded reward pool.**
