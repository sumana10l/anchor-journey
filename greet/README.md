### 1️⃣ Imports

```rust
use anchor_lang::prelude::*;
```

**Purpose:**
Brings in Anchor’s tools so you don’t write low-level Solana code yourself.
Analogy: importing a standard library.

---

### 2️⃣ Program ID

```rust
declare_id!("...");
```

**Purpose:**
Tells Solana **who this program is**.
Analogy: your contract’s address / identity.

---

### 3️⃣ Program (Actions)

```rust
#[program]
pub mod calc {
    pub fn initialize(...) -> Result<()> {
        ...
    }
}
```

**Purpose:**
This is the **list of actions** your contract supports.
Each function = one action users can trigger from the frontend.

No magic:
user clicks → instruction → this function runs.

---

### 4️⃣ Accounts (Permissions)

```rust
#[derive(Accounts)]
pub struct Initialize { ... }
```

**Purpose:**
Declares **what this action is allowed to read or write**.

Even if empty, you’re saying:

> “This action touches nothing.”

This is Solana’s safety model.

---

### One-line mental model (remember this)

> **Anchor = actions + explicit permissions + no hidden state**

Everything else is ceremony to enforce that rule.

