# Blockchain Simulation in Rust

Toy blockchain controlable through the CLI.

### Key components:

- Block: Data structure that will hold data.
- Blockchain: The chain itself represented by the succesive blocks.
- Transaction: The actions (create account, transfer funds) that will be included in blocks.
- Account: On chain representation of the user

### Features

- Start Node (exec): Launch a local blockchain node that mines blocks at regular intervals (10s).
- Create Account (exec): Create a new account on the blockchain with a specified starting balance.
- Transfer Funds (exec): Transfer funds from one account to another.
- Check Balance (query): Query the balance of a specific account instantaneously.

### Running the Simulation

#### Start the Blockchain Node:

`cargo run --bin server`

### Start the client:

`cargo run --bin client`

#### Create an Account (from client):

`create-account <id-of-account> <starting-balance>`

#### Transfer Funds (from client):

`transfer <from-account> <to-account> <amount>`

#### Check Account Balance (from client):

`balance <account>`

### Architecture

The server (the validator) is adding new transactions to the mempool (here we use a json file).

The blockchain is then responsible to process those transactions and store information related to the accounts onchain (in memory).
