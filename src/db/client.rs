/*-------------
/db/client.rs

This file is for modeling the Client data and functions.

Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/

/// The client struct is used for holding the client infomation.
/// 
/// # Info
/// * `client` - (u16) The Unqiue Client ID.
/// * `available` - (f32) Total funds that are avaiable for trading, staking, withdrawls etc.
/// * `held` - (f32) Total fund that are held for dispute.
/// * `total` - (f32) Total funds that are avaiable or held.
/// * `locked` - (bool) Wether a Client is locked. A Client is locked if a charge back occurs.
#[derive(Debug)]
pub struct Client {
    pub client: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub lock: bool,
}

impl Client {
    /// Creates the client allowing for basic manipluation of the struct
    pub fn new(client: u16, deposit: f32) -> Client {
        Client {client: client, total: deposit, held: 0.0, available: deposit, lock: false,}
    }

    /// Allows a user despoit funds into client.
    /// 
    /// # Arguments
    /// * `deposit` - (f32) Represents a monetary value to add to the client.
    pub fn deposit(&mut self, deposit: f32) -> &mut Client {
		self.available += deposit;
        self.total += deposit;
        return self;
    }

    /// A user can withdraw money from the client.
    /// Note this will allow them to go into the negative
    /// 
    /// # Arguments
    /// * `withdraw` - (f32) Represents a monetary value to remove from the client.
    pub fn withdrawal(&mut self, withdrawl: f32) -> &mut Client {
        if self.available < withdrawl {
            return self;
        }
		self.available -= withdrawl;
        self.total -= withdrawl;
        return self;
    }

    /// Allows the system to dispute the charge and remove the avaiable amount 
    /// 
    /// # Arguments
    /// * `amount` - (f32) Represents a monetary value to manipulate in the client
    pub fn dispute(&mut self, amount: f32) -> &mut Client {
        self.available -= amount;
        self.held += amount;
        return self;
    }

    /// Allows the system to resolve the charge moving the money back into the client
    /// 
    /// # Arguments
    /// * `amount` - (f32) Represents a monetary value to manipulate in the client
    pub fn resolve(&mut self, amount: f32) -> &mut Client {
		self.available += amount;
        self.held -= amount;
        return self;
    }

    /// Allows the system to chargeback the charge moving the money from the account and locking it.
    /// 
    /// # Arguments
    /// * `amount` - (f32) Represents a monetary value to manipulate in the client
    pub fn chargeback(&mut self, amount: f32) -> &mut Client {
        self.held -= amount;
        self.total -= amount;
		self.lock = true;
        return self;
    }
}