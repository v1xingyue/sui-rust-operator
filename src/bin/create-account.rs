use std::sync::{Arc, Mutex};
use sui_rust_operator::account::SuiAccount;

use std::thread;

#[tokio::main]
async fn main() {
    let mut handlers = vec![];
    let accounts = vec![];

    let accounts_arc: Arc<Mutex<Vec<SuiAccount>>> = Arc::new(Mutex::new(accounts));

    for thread_no in 1..=3 {
        let handle_resource = Arc::clone(&accounts_arc);
        let handle = thread::spawn(move || {
            for i in 1..=5 {
                let account: SuiAccount = SuiAccount::new_account();
                println!(
                    "thread {:0>4} create account  No.{:0>3} {}",
                    thread_no, i, account
                );
                let mut _guard = handle_resource.lock().unwrap();
                _guard.push(account);
            }
        });
        handlers.push(handle);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    {
        let accounts_main = Arc::clone(&accounts_arc);
        let guard = accounts_main.lock().unwrap();
        println!("Total accounts generated: {}", guard.len());

        for account in &*guard {
            println!("Account: {}", account);
        }
    }
}
