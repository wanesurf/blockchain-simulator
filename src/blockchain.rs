use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    CreateAccount {
        account_id: String,
        balance: u64,
    },
    Transfer {
        from_account: String,
        to_account: String,
        amount: u64,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub transaction_type: TransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub account_id: String,
    pub balance: u64,
}
#[derive(Debug, Default)]
pub struct AccountsDatabase {
    accounts: HashMap<String, Account>,
}
impl AccountsDatabase {
    pub fn new() -> Self {
        AccountsDatabase {
            accounts: HashMap::new(),
        }
    }

    pub fn create_account(&mut self, account_id: String, initial_balance: u64) {
        let account = Account {
            account_id: account_id.clone(),
            balance: initial_balance,
        };
        self.accounts.insert(account_id, account);
    }

    pub fn transfer(
        &mut self,
        from_account: &str,
        to_account: &str,
        amount: u64,
    ) -> Result<(), &'static str> {
        let from_balance = self
            .accounts
            .get_mut(from_account)
            .ok_or("From account not found")?
            .balance;

        if from_balance < amount {
            return Err("Insufficient balance");
        }

        self.accounts.get_mut(from_account).unwrap().balance -= amount;
        self.accounts.get_mut(to_account).unwrap().balance += amount;

        Ok(())
    }

    pub fn get_balance(&self, account_id: &str) -> Result<u64, &'static str> {
        self.accounts
            .get(account_id)
            .map(|account| account.balance)
            .ok_or("Account not found")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, prev_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let hash: String = Self::calculate_hash(index, timestamp, &transactions, &prev_hash);

        Block {
            index,
            timestamp,
            transactions,
            prev_hash,
            hash,
        }
    }

    fn calculate_hash(
        index: u64,
        timestamp: i64,
        transactions: &[Transaction],
        prev_hash: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string().as_bytes());
        hasher.update(timestamp.to_string().as_bytes());
        hasher.update(serde_json::to_string(transactions).unwrap().as_bytes());
        hasher.update(prev_hash.as_bytes());

        let hash = hasher.finalize();
        let mut hash_str = String::new();
        for byte in hash.iter() {
            hash_str.push_str(&format!("{:02x}", byte));
        }

        hash_str
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub accounts: AccountsDatabase,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, vec![], String::from("0"));
        Blockchain {
            chain: vec![genesis_block],
            pending_transactions: Vec::new(),
            accounts: AccountsDatabase::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    fn read_and_clear_transactions() -> Vec<Transaction> {
        let file = File::open("transactions.json").unwrap();
        let reader = BufReader::new(file);

        let transactions: Vec<Transaction> = reader
            .lines()
            .filter_map(|line| serde_json::from_str(&line.unwrap()).ok())
            .collect();

        // Clear the file after reading
        File::create("transactions.json").unwrap();

        transactions
    }

    pub fn execute_transaction(&mut self, transactions: Vec<Transaction>) {
        for transaction in transactions {
            match transaction.transaction_type {
                TransactionType::CreateAccount {
                    account_id,
                    balance,
                } => {
                    if let Some(_) = self.accounts.accounts.get(&account_id) {
                        println!("Account {} already exists", account_id);
                    } else {
                        let account_id_clone = account_id.clone();
                        self.accounts.create_account(account_id, balance);
                        println!(
                            "Creating account {} with balance {}",
                            account_id_clone, balance
                        );
                    }
                }
                TransactionType::Transfer {
                    from_account,
                    to_account,
                    amount,
                } => {
                    if let Err(e) = self.accounts.transfer(&from_account, &to_account, amount) {
                        println!("Error: {}", e);
                    } else {
                        println!(
                            "Transferring {} from account {} to account {}",
                            amount, from_account, to_account
                        );
                    }
                }
            }
        }
    }

    pub fn mine_block(&mut self) {
        Self::read_and_clear_transactions()
            .iter()
            .for_each(|transaction| {
                self.pending_transactions.push(transaction.clone());
            });

        let transactions: Vec<Transaction> = self.pending_transactions.drain(..).collect();
        println!("Mining block with {} transactions", transactions.len());
        self.execute_transaction(transactions.clone());
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let block = Block::new(self.chain.len() as u64, transactions, prev_hash);

        self.chain.push(block);
    }
}
