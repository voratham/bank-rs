Lifetime
- Refers to how long an owner/reference exists

Lifetime Generic or Annotation
- Extra syntax added in to clarify relationship between different lifetimes


```rust
// it should error about lifetime
fn make_and_print_account() -> &Account {
    let account = Account::new(1, String::from("voratham"));
    println!("{:#?}", account);
    &account 
} //<-- because when at the end function account_ref will out-of-scope

fn main() {
    let account_ref=  make_and_print_account();
    println!("account_ref : {:#?}",account_ref)
}

```

 Function argumentTypes
Need to store the argument somewhere ? -> Favor taking ownership (receive a value)
Need to do a calculation with the value? -> Favor receiving a read-only ref
Need to change the value in some way? -> Favor receiving a mutable ref

