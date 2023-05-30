//! Here, we have to define the types: `AccountId` and `Balance`.

use std::collections::HashMap;

/// Hardcoded types
type AccountId = u32;
type Balance = u32;

pub struct BalanceModule {
    pub balance: HashMap<AccountId, Balance>,
}

impl BalanceModule {
    pub fn new() -> Self {
        BalanceModule {
            balance: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: AccountId, balance: Balance) {
        self.balance.insert(who, balance);
    }

    pub fn get_balance(&self, who: &AccountId) -> &Balance {
        self.balance.get(&who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), String> {
        let balance_from = self
            .balance
            .get(&from)
            .ok_or("from balance does not exist")?;
        let balance_to = self.balance.get(&to).ok_or("to balance does not exist")?;

        if *balance_from == 0 {
            return Err("Account from does not exist".to_string());
        }

        let new_balance_from = balance_from.checked_sub(amount).unwrap();
        let new_balance_to = balance_to.checked_add(amount).unwrap();

        self.balance.insert(from, new_balance_from);
        self.balance.insert(to, new_balance_to);

        Ok(())
    }
}
