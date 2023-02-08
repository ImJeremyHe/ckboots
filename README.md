# CKBoots

`CKBoots` is an app framework for [CKB](https://github.com/nervosnetwork/ckb).
You don't need to pay any attention to the implementation of CKB.
Just focus on your App!

## Start to wear the boots

### Config your project

In `ckboots.toml`:

```toml
[chain]
# CKB rpc
rpc = "http://127.0.0.1:8114"
# Your secret key
secreat_key = "0x0"

[boots]
app_name = "your app name"
initial_capacity = "2000000"

[dev]
debug = true
# If you are in dev mode, these contracts will deploy automactically.
contracts_dir = "path to your all contracts"
```

### Define your on-chain status

Define your status using this macro `OnChain` and assign an `id` to it.
It means that there is a cell for storing this on-chain status, and `id`
helps us find it.

```rust
#[derive(OnChain)]
#[onchain(id = "hero")]
pub struct Hero {
    hp: u8,
    mana: u8,
    exp: Exp,
}
```

All the types in your status should be implemented this `OnChain` trait.
But it does not need to have an `id`.

```rust
#[derive(OnChain)]
pub struct Exp {
    pub level: u8,
    pub next_exp: u64,
}
```

### Define your function and specify the address of your contract

todo!()

### Example
