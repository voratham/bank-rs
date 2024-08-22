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

You can create many read-only (immutable) references to a value. These refs can all exist at the same time.

```rust
fn print_account(account: &Account) {
    println!("{:#?}", account);
}
fn main() {
    let account = Account::new(1, String::from("Voratham"));

    let account_ref1 = &account;
    let account_ref2 = &account;

    print_account(account_ref1);
    print_account(account_ref2);

    println!("{:#?}", account)
}

```

You can't move a value while a ref to the value exists

When in doubt, remember that Rust wants to minimize unexpected updates to data


!! moving ownership to update something really tedious

You can make a writeable (mutable) reference to a value only if there are no read-only references currently in use. One mutable ref to a value can exist at a time.

You can't mutate a value through the owner when any ref (mutable or immutable) to the value exists


```rust

fn add_account(bank: &mut Bank , account:Account ) {
    bank.accounts.push(account);
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("me"));
    
    // TODO: call the 'add_account' function here
    add_account(&mut bank, account);
    
    
    // Note: we're using the Bank value here, so 'bank' still
    // needs ownership of that value
    println!("print bank {:#?}", bank);

    // after that we cannot print account again because account value moved
    println!("print account {:#?}", account);
    
}

```

Some types of values like numbers, booleans, etc are going to appear to break the rules of ownership !!


```rust
fn main() {
    let id = 1;
    let holder = String::from("me");
 
    let account = Account::new(id, holder);
 
    println!("{:#?} {:#?}", id, holder); // holder is Ownerd data type
}
```