use std::{collections::BTreeSet, cmp::Ordering};

use candid::{CandidType, Deserialize};

#[derive(Debug, Clone, Eq, CandidType, Deserialize)]
pub struct Address {
    pub name: Option<String>,
    pub id: u32,
}

impl Address {
    pub fn new(id: u32, name: Option<String>) -> Address {
        Address {
            id, 
            name
        }
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Address {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}


#[derive(Default, Clone)]
pub struct AddressBook(BTreeSet<Address>);

impl AddressBook {
    #[inline]
    pub fn insert(&mut self, entry: Address) {
        if let Some(mut existing) = self.0.take(&entry) {
            if entry.name.is_some() {
                existing.name = entry.name;
            }
            self.0.insert(existing);
        } else {
            self.0.insert(entry);
        }
    }
    #[inline]
    pub fn find(&self, id: u32) -> Option<&Address> {
        for a in &self.0 {
            if a.id == id {
                return Some(a);
            }
        }

        None
    }
    #[inline]
    pub fn remove(&mut self, id: u32) {
        self.0.remove(&Address::new(id, None));
    }
    #[inline]
    pub fn take(&mut self, id: u32) -> Option<Address> {
        self.0.take(&Address::new(id, None))
    }
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Address> {
        self.0.iter()
    }
}

