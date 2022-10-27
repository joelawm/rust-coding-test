/*-------------
/engine.rs

This file is for creating the main process that moves the transaction data.

Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/

use std::collections::HashMap;
use ::log::{error};
use crate::db::{client::Client, transaction::{TransactionType, Transaction}};

/// A hashmap of the current memory of the transactions
pub struct PaymentEngine {
    pub transactions: HashMap<u32, Vec<Transaction>>,
	pub clients: HashMap<u16, Client>,
}

impl PaymentEngine {
	/// Creates a new payment engine and initializes the hashmaps.
    pub fn new() -> PaymentEngine {
        PaymentEngine { transactions: HashMap::new(), clients: HashMap::new() }
    }

	/// Runs the transactions and 
    pub fn run_transaction(&mut self, tx: &Transaction) -> &mut PaymentEngine {
		// Check to see if a transaction id exists
		if self.transactions.contains_key(&tx.tx) {
			// If it does were gonna add to the vector which is making essentially
			// a LinkedList Hashmap
			let transaction = self.transactions.get_mut(&tx.tx).unwrap();
			transaction.push(tx.clone());
		}
		else {
			// Insert if it does not exist already
			self.transactions.insert(tx.tx, vec![tx.clone()]);
		}

		// If the client does not exist create it so we can create its totals
		if !self.clients.contains_key(&tx.client_id) {
			if let TransactionType::Deposit = tx.t {
				self.clients.insert(tx.client_id, Client::new(tx.client_id, tx.amount.unwrap()));
			} else {error!("File not in chronological order.")}

			// Already did the basic depositing or withdraw so it should work
			return self;
		}

		// Match the transaction to the right enum type
        match tx.t {
            TransactionType::Deposit => {self.deposit(tx);},
            TransactionType::Withdrawal => {self.withdrawal(tx);},
            TransactionType::Dispute => {self.dispute(tx);},
            TransactionType::Resolve => {self.resolve(tx);},
            TransactionType::Chargeback => {self.chargeback(tx);},
        }
        self
    }

	/// Allows you to output to a csv accounts file.
	pub fn print_output(&self) {
        // Print heading
        println!("client,available,held,total,locked");

        // Print data
        for user in &self.clients {
            println!("{},{},{},{},{}", user.1.client, user.1.available, user.1.held, user.1.total, user.1.lock);
        }
    }

	/// Allows a user to deposit into a account
    fn deposit(&mut self, tx: &Transaction) -> &mut PaymentEngine {
        let client = self.clients.get_mut(&tx.client_id).unwrap();

		// Check to see if the account is already locked.
        if client.lock {
			error!("The account is locked.");
            return self
        }

		// Add to the clients total
        client.deposit(tx.amount.unwrap());

        self
    }

	/// Allows a user to withdraw from the account
    fn withdrawal(&mut self, tx: &Transaction) -> &mut PaymentEngine {
        let client = self.clients.get_mut(&tx.client_id).unwrap();

		// Check to see if the account is already locked.
        if client.lock {
			error!("The account is locked.");
            return self
        }

		// Hit the client withdraw function
        client.withdrawal(tx.amount.unwrap());

        self
    }

	/// Allows the system to dispute a charge
    fn dispute(&mut self, tx: &Transaction) -> &mut PaymentEngine {
        let client = self.clients.get_mut(&tx.client_id).unwrap();

		// Check to see if the account is already locked.
        if client.lock {
			error!("The account is locked.");
            return self;
        }

		// Check to see if the transaction exists
        if !self.transactions.contains_key(&tx.tx) {
            return self;
        }

		// Grab the transaction
        let tx = self.transactions.get_mut(&tx.tx).unwrap();

		// Iterate through each transaction
		for tr in tx {
			// If the amount is not nothing then grab it because its the original deposit
			if tr.amount != None {
				tr.dispute = true;
				client.dispute(tr.amount.unwrap());
			}
		}

        self
    }

	/// Allow the system to resolve the current transaction
    fn resolve(&mut self, tx: &Transaction) -> &mut PaymentEngine {
        let client = self.clients.get_mut(&tx.client_id).unwrap();

		// Check to see if the account is already locked.
        if client.lock {
			error!("The account is locked.");
            return self;
        }

		// Check to see it contains the key
        if !self.transactions.contains_key(&tx.tx) {
            return self;
        }

		// Grab the transaction
        let tx = self.transactions.get_mut(&tx.tx).unwrap();

		// Iterate through the transaction
		for tr in tx {
			// Check to see if its disputed
			if !tr.dispute {
				return self;
			}

			// Grab the transaction thats not Null
			if tr.amount != None {
				// Using a mutable reference so no need to re-insert.
				client.resolve(tr.amount.unwrap());
				tr.dispute = false;
			}
		}

        self
    }

	/// Allow the system to lock the account and set it as a charge back
    fn chargeback(&mut self, tx: &Transaction) -> &mut PaymentEngine {
        let client = self.clients.get_mut(&tx.client_id).unwrap();

		// Check to see if the account is already locked.
        if client.lock {
			error!("The account is locked.");
            return self;
        }

		// Check to see if the transaction exists
        if !self.transactions.contains_key(&tx.tx) {
            return self;
        }

		// Grab a mutable transaction
        let tx = self.transactions.get_mut(&tx.tx).unwrap();

		// Iterates through the transaction
		for tr in tx {
			// Check to see if the transaction is disputed
			if !tr.dispute {
				error!("Failed to dispute the transaction.");
				return self;
			}
	
			// Set the client to chargeback
			client.chargeback(tr.amount.unwrap());
			tr.dispute = false;
		}

        self
    }
}