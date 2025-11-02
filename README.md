# On-chain VAULT PROGRAM

This is a vault program that allows users to deposit SOL, withdraw SOL (if they're the vault authority), and toggle the Vault's lock state.

## Program Instructions
This vault program consists of three key instructions:

- **`deposit`** - Allows users to deposit SOL into any vault (if unlocked)
- **`withdraw`** - Allow vault authorities to withdraw SOL from their vaults (if unlocked)  
- **`toggle_lock`** - Allow vault authorities to lock/unlock their vaults


### How to run this Project Locally

**Clone the repo to your local dev environment:**
Use the git clone command

**Install dependencies:**
```bash
yarn install
```

**Build the project:**
```bash
anchor build
```

**Test your implementation:**
```bash
anchor test
```

All test cases are expected to pass ✔️

