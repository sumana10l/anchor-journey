## 🧾 SOL Staking + Points – Anchor Breakdown

---

### 1️⃣ Imports & Constants

```rust
use anchor_lang::prelude::*;
```

**Purpose:**
Anchor macros + Solana primitives.
Constants define **reward rate, time units, and SOL math**.

**Analogy:**
Rulebook + calculator for rewards.

---

### 2️⃣ Program ID

```rust
declare_id!("EFpkThxpS78297...");
```

**Purpose:**
Unique on-chain identity of this staking program.

---

### 3️⃣ Program (Core Actions)

```rust
#[program]
pub mod staking_contract { ... }
```

This contract supports **staking, rewards, and treasury payouts**.

---

#### 🏦 `initialize_treasury`

Creates a PDA treasury controlled by an admin.

**Analogy:**
Company wallet with an owner.

---

#### 💰 `fund_treasury`

Admin deposits SOL for future payouts.

---

#### 👤 `create_pda_account`

Creates a **user-specific PDA** to track stake + points.

**Analogy:**
Personal scorecard.

---

#### 🔒 `stake / unstake`

Moves SOL in/out and updates points based on **time × amount**.

**Analogy:**
Interest accumulating while money is locked.

---

#### ⭐ `claim_points`

Resets accumulated points (no SOL yet).

---

#### 🔄 `convert_points_to_sol`

Turns points into SOL, paid from treasury.

**Key rule:**
Treasury must have enough **rent-safe balance**.

---

### 4️⃣ State Accounts

#### `StakeAccount`

Stores:

* owner
* staked SOL
* points
* last update time

**Analogy:**
Your staking ledger.

#### `Treasury`

Stores:

* admin
* funded / paid SOL
* paused flag

**Analogy:**
Reward pool vault.

---

### One-line mental model

> **Users stake SOL, earn time-based points, and redeem them for SOL from a shared treasury—if the admin allows it.**
