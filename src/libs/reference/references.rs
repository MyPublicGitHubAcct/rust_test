struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}.",
            amount, self.owner
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance of {}.",
            self.owner, self.balance
        )
    }
}

pub fn print_reference_examples() {
    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // immutable borrow to check the balance
    account.check_balance();

    // mutable borrow to withdraw money
    account.withdraw(45.50);
    account.check_balance();
}
