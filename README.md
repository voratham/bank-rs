


Note:
Ownership

- the goal of ownership is to limit the ways you can reference and change data

- This limitation will reduce the number of bugs + make your code easier to understand

- Every value is 'owned' by a single variable at at time

- reassigning the value to another variable moves the value. The old variable can't be used to access the value anymore !


```rust
 let bank = Bank::new();
 let other_bank = bank;
 println!("{:#?}", bank) <--- because bank move to other_bank
```

playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=67bbf9c23c1b45cfacf008d5de8a6e33


### incase ownership will cannot access variable 
```rust
    // case 1
    let bank = Bank::new();
    let other_bank = bank;
    println!("{:#?}", bank)

    // case 2
    let account = Account::new(1, String::from("Voratham"));
    print_account(account);
    print_account(account);


    // case 3
    let bank = Bank::new();
    let accounts = bank.accounts;
    println!("{:#?}", bank.accounts);

    // case 4
    let account = Account::new(1, String::from("Voratham"));
    print_account(account);
    println!("{}", account.holder);

    // case5
    let account = Account::new(1, String::from("Voratham"));
    print_holder(account.holder);
    print_account(account);

    println!("{:#?}", account.balance) // can print because balance field not take ownership
```


### how to working around but not pretty ownership
```rust
fn print_account(account: Account) -> Account {
    println!("{:#?}", account);
    account
}
fn main() {
    let mut account = Account::new(1, String::from("Voratham"));
    account = print_account(account);
    account = print_account(account);

    println!("{:#?}", account)
}
```