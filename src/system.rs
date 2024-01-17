/* TODO: You might need to update your imports. */

use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Pallet {
            block_number: Default::default(),
            nonce: BTreeMap::new()
        }
        /* TODO: Return a new instance of the `Pallet` struct. */
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, user: &String) {
        let nonce = *self.nonce.get(user).unwrap_or(&0);
        // let nonce = self.nonce.get_mut(user).unwrap();
        let new_nonce = nonce.checked_add(1).unwrap();
        self.nonce.insert(user.clone(), new_nonce);
    }
    
}


#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let mut system = super::Pallet::new();
        system.inc_block_number();
        system.inc_nonce(&"alice".to_string());

        assert_eq!(system.block_number(), 1);
        // assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1));
        // assert_eq!(system.nonce.get(&"bob".to_string()), None);
    }
}