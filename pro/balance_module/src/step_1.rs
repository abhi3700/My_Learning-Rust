use std::collections::HashMap;

pub struct BalanceModule {
    pub balance: HashMap<u32, u32>,
}

impl BalanceModule {
    pub fn new() -> Self {
        BalanceModule {
            balance: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: u32, balance: u32) {
        self.balance.insert(who, balance);
    }

    pub fn get_balance(&self, who: &u32) -> &u32 {
        self.balance.get(&who).unwrap_or(&0)
    }

    pub fn transfer(&mut self, from: u32, to: u32, amount: u32) -> Result<(), String> {
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
