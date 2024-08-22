#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_holder(holder: String) {
    println!("{}", holder)
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn change_account(account: &mut Account) {
    account.balance = 10;
    println!("change_account print {}", account.holder);
}

// fn main() {
//     let mut account = Account::new(1, String::from("Voratham"));

//     let account_ref = &mut account;

//     account.balance = 10;

//     change_account(account_ref);

//     println!("{:#?}", account)
// }

fn set_account_id(account: &mut Account, id: u32) { // Line 1
    account.id = id;
}
 
// fn main() {
// let  account = Account::new(1, String::from("me"));    
// let  other_account = account;
// println!("{:#?}", account)
//     let num = 5;
//     let other_num = num;
//     println!("{} , {}", num , other_num)
// }



// fn main() {
//     let id = 1;
//     let holder = String::from("me");
 
//     let account = Account::new(id, holder);
 
//     println!("{:#?} {:#?}", id, holder);
// }