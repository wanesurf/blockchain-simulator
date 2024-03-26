use std::fs::OpenOptions;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod blockchain;
use blockchain::{Blockchain, Transaction, TransactionType};

use std::io::{Read, Write};

fn append_to_file(transaction: &Transaction) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("transactions.json")
        .unwrap();
    writeln!(&file, "{}", serde_json::to_string(&transaction).unwrap()).unwrap();
}
fn process_command(command: &str, blockchain: &Arc<Mutex<Blockchain>>) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();

    match parts.as_slice() {
        ["create-account", id, balance] => {
            let balance: u64 = match balance.parse() {
                Ok(b) => b,
                Err(_) => return "Invalid balance amount.".to_string(),
            };
            let transaction = Transaction {
                transaction_type: TransactionType::CreateAccount {
                    account_id: id.to_string(),
                    balance,
                },
            };
            append_to_file(&transaction);

            format!("Account {} with balance {} added to mempool", id, balance)
        }
        ["transfer", from_id, to_id, amount] => {
            let amount: u64 = match amount.parse() {
                Ok(a) => a,
                Err(_) => return "Invalid transfer amount.".to_string(),
            };
            let transaction = Transaction {
                transaction_type: TransactionType::Transfer {
                    from_account: from_id.to_string(),
                    to_account: to_id.to_string(),
                    amount,
                },
            };

            append_to_file(&transaction);
            format!(
                "Transaction {} with balance {} added to mempool",
                from_id, amount
            )
        }
        ["balance", id] => {
            let bc = blockchain.lock().unwrap();
            match bc.accounts.get_balance(id) {
                Ok(balance) => format!("Balance for {}: {}", id, balance),
                Err(_) => "Account not found.".to_string(),
            }
        }
        _ => "Invalid command.".to_string(),
    }
}

fn handle_client(mut stream: std::net::TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            let command = std::str::from_utf8(&buffer[..size]).expect("Invalid UTF-8 sequence");
            let response = process_command(command.trim(), &blockchain);
            stream
                .write_all(response.as_bytes())
                .expect("Failed to write to stream");
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind to port");

    let blockchain: Arc<Mutex<Blockchain>> = Arc::new(Mutex::new(Blockchain::new()));

    let blockchain_clone = blockchain.clone();
    thread::spawn(move || {
        let mut interval = 0;
        loop {
            thread::sleep(Duration::from_secs(10));
            let mut blockchain = blockchain_clone.lock().unwrap();
            blockchain.mine_block();
            println!("Block #{} has been mined.", interval);
            interval += 1;
        }
    });

    for stream in listener.incoming() {
        print!("Incoming connection...");
        match stream {
            Ok(stream) => {
                let blockchain = Arc::clone(&blockchain);
                thread::spawn(move || handle_client(stream, blockchain));
            }
            Err(e) => println!("Failed to establish a connection: {}", e),
        }
    }
}
