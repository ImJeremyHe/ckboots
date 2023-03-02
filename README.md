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

### Write your contract logic and just add an attribute macro

```rust
#[contract]
pub fn complete_mission_1(hero: &mut Hero){
    let mut hero = hero;
    hero.exp.level += 1;
    hero.next_exp = 1000;
    hero
}
```

Here, `hero` is an on-chain status that will be updated,
which is implied by an mutable reference.

You are able to use change mutltiple on-chain statuses in a contract, just like this:

```rust
#[contract]
pub fn complete_mission_2(hero: &mut Hero, item: &mut Item) {
    todo!()
}
```

Note that `Hero` and `Item` should derive `OnChain` and be assigned an `id`.

If your contract need to look at some on-chain statuses to make a decision, you can use unmutable references like:

```rust
#[contract(Mission2, id="mission2")]
pub fn complete_mission_2(hero: &mut Hero, item: &Item) {
    todo!()
}
```

Some times some arguments should be passed because of users' actions. In these cases, you should create an on-chain struct first like:

```rust
#[derive(OnChain)]
pub struct UserInput {
    /// some fields
}
```

And then your contract function can be like:

```rust
// user_action is not a reference.
#[contract(Mission2, id="mission2")]
pub fn complete_mission_2(hero: &mut Hero, user_action: UserInput);
```

Now, you can create your application and registering on-chain types and contracts.

```rust
use ckboots::create_app;

create_app!(TravelFrog {
    types: [Frog],
    contracts: [Travel],
})

```

App will automatically build your transaction and send it.

### Example
