//! Here, we DON'T have to define the types: `AccountId` and `Balance` explicitly
//! as the structs are defined in a generic way like struct<T, U> i.e. BalanceModule<AccountId, Balance>

use std::{cmp::Eq, collections::HashMap, hash::Hash};

use num::{CheckedAdd, CheckedSub, Zero};

pub struct BalanceModule<AccountId, Balance> {
    pub balance: HashMap<AccountId, Balance>,
}

impl<AccountId, Balance> BalanceModule<AccountId, Balance>
where
    AccountId: Eq + Hash,
    Balance: Zero + CheckedAdd + CheckedSub + PartialEq + Copy,
{
    pub fn new() -> Self {
        Self {
            balance: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: AccountId, balance: Balance) {
        self.balance.insert(who, balance);
    }

    pub fn get_balance(&self, who: &AccountId) -> Balance {
        *self.balance.get(&who).unwrap_or(&Balance::zero())
    }

    pub fn transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), String> {
        // let balance_from = self.get_balance(from).unwrap();
        // let balance_to = self.get_balance(to).unwrap();
        let balance_from = self
            .balance
            .get(&from)
            .ok_or("from balance does not exist")?;
        let balance_to = self.balance.get(&to).ok_or("to balance does not exist")?;

        if balance_from.eq(&Balance::zero()) {
            return Err("Account from does not exist".to_string());
        }

        let new_balance_from = balance_from.checked_sub(&amount).unwrap();
        let new_balance_to = balance_to.checked_add(&amount).unwrap();

        self.balance.insert(from, new_balance_from);
        self.balance.insert(to, new_balance_to);

        Ok(())
    }
}
