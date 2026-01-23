## 🧾 Simple Escrow – Anchor Breakdown

---

### 1️⃣ Imports

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer as TokenTransfer, transfer};
use anchor_spl::associated_token::AssociatedToken;
```

**Purpose:**

* `anchor_lang` → core Anchor macros & types
* `anchor_spl::token` → SPL token operations (transfer, accounts)
* `associated_token` → auto-create token accounts

**Analogy:**
Importing SDKs instead of writing raw blockchain code.

---

### 2️⃣ Program ID

```rust
declare_id!("...");
```

**Purpose:**
Defines the **on-chain identity** of this contract.

**Analogy:**
Your app’s permanent blockchain address.

---

### 3️⃣ Program (Actions)

```rust
#[program]
pub mod simple_escrow { ... }
```

This contract supports **two actions**:

---

#### 🔐 `initialize_escrow`

```rust
pub fn initialize_escrow(ctx, amount, receiver) -> Result<()>
```

**What it does:**

* Creates an escrow record
* Creates a vault (token account owned by PDA)
* Moves tokens from payer → vault

**Analogy:**
Putting money into a locker and giving the key to code.

---

#### 💸 `claim_escrow`

```rust
pub fn claim_escrow(ctx) -> Result<()>
```

**What it does:**

* Verifies caller is the receiver
* PDA signs the transaction
* Moves tokens from vault → receiver

**Analogy:**
Receiver opens the locker and takes the money.

---

### 4️⃣ Escrow Account (State)

```rust
#[account]
pub struct Escrow { ... }
```

**Purpose:**
Stores **metadata**, not tokens:

* who paid
* who receives
* how much
* token type
* PDA bump

**Analogy:**
A receipt describing what’s inside the locker.

---

### 5️⃣ InitializeEscrow Accounts (Permissions)

```rust
#[derive(Accounts)]
pub struct InitializeEscrow { ... }
```

**What’s enforced:**

* Escrow account is created
* Initializer pays rent
* Vault PDA is derived safely
* Vault token account is created
* Tokens are allowed to move

**Key rule:**
Initializer must **sign** and **own the tokens**.

---

### 6️⃣ ClaimEscrow Accounts (Permissions)

```rust
#[derive(Accounts)]
pub struct ClaimEscrow { ... }
```

**What’s enforced:**

* Only the stored receiver can claim
* Vault authority must match PDA
* Vault can be debited safely

**Key rule:**
If you’re not the receiver → transaction fails.

---

### One-line mental model (lock this in)

> **This escrow locks tokens and lets a pre-defined receiver withdraw them anytime.**

