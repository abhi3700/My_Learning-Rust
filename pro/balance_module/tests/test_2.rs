#[path = "../src/step_2.rs"]
mod step_2;
use step_2::BalanceModule;

use std::collections::HashMap;

#[cfg(test)]
mod test_step_2 {
    use super::*;

    #[test]
    fn step_2() {
        // ===new()===
        let mut balance = BalanceModule::new();
        balance.balance.insert(1, 100);
        assert_eq!(balance.balance.get(&1), Some(&100));

        // ===set_balance()===
        let mut balance = BalanceModule::new();

        balance.set_balance(1, 100);
        assert_eq!(*balance.get_balance(&1), 100);

        // ===get_balance()===
        let mut balance = BalanceModule::new();

        balance.set_balance(1, 100);
        assert_eq!(*balance.get_balance(&1), 100);

        // ===transfer()===
        let mut balance = BalanceModule::new();

        balance.set_balance(1, 100);
        balance.set_balance(2, 100);

        balance.transfer(1, 2, 50).unwrap();

        assert_eq!(*balance.get_balance(&1), 50);
        assert_eq!(*balance.get_balance(&2), 150);
    }
}
