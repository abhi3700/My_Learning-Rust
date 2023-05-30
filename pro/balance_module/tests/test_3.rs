#[path = "../src/step_3.rs"]
mod step_3;
use step_3::BalanceModule;

use std::collections::HashMap;

#[cfg(test)]
mod test_step_3 {
    // imagine `test_step_3` this as runtime. So, you can give any type of AccountId and Balance
    use super::*;

    /// NOTE: Here, runtime is going to see if the given types satisfy the trait bounds
    /// used in the implementation block in `step_3.rs` file.
    type AccountId = u32;
    type Balance = u32;

    #[test]
    fn step_3() {
        // ===new()===
        let mut balance = BalanceModule::new();
        // let mut balance: BalanceModule<AccountId, Balance> = BalanceModule::new();   // can also be defined like this

        balance.balance.insert(1, 100);
        assert_eq!(balance.balance.get(&1), Some(&100));

        // ===set_balance()===
        let mut balance: BalanceModule<AccountId, Balance> = BalanceModule::new();

        balance.set_balance(1, 100);
        assert_eq!(balance.get_balance(&1), 100);

        // ===get_balance()===
        let mut balance: BalanceModule<AccountId, Balance> = BalanceModule::new();

        balance.set_balance(1, 100);
        assert_eq!(balance.get_balance(&1), 100);

        // ===transfer()===
        let mut balance: BalanceModule<AccountId, Balance> = BalanceModule::new();

        balance.set_balance(1, 100);
        balance.set_balance(2, 100);

        balance.transfer(1, 2, 50).unwrap();

        assert_eq!(balance.get_balance(&1), 50);
        assert_eq!(balance.get_balance(&2), 150);
    }
}
