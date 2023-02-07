# CKBoots

`CKBoots` is an app framework for [CKB](https://github.com/nervosnetwork/ckb).
You don't need to pay any attention to the implementation of CKB.
Just focus on your App!

## Start to wear the boots

### Define your on-chain status

Define your status using this macro `def_onchain_status!`.

```rust
def_onchain_status! {
    pub struct Hero {
        hp: u8,
        mana: u8,
        exp: Exp,
    }
}
```

All the types in your status should be implemented this trait: `OnChain`.
We provide an easy way for you.

```rust
#[derive(OnChain)]
pub struct Exp {
    pub level: u8,
    pub next_exp: u64,
}
```

### Define your function and specify the address of your contract

todo!()
