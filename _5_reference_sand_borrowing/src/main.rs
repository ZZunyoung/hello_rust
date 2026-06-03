// Reference and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts

// Understanding References
// References: Enable you to borrow values without taking ownership.
// Immutable Reference.
// Mutable Reference.
// Create Reference by add "&"
// -I- Immutable Reference

fn main() {
    // let mut _x = 5;
    // let _r = &mut _x;

    // *_r += 1;
    // *_r -= 3;

    // println!("Value of x: {}", _x);
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 1000.1,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(200.5);
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing ${} from {}'s account", amount, self.owner);
        self.balance -= amount;
    }
    fn check_balance(&self) {
        println!("{}'s account balance: ${}", self.owner, self.balance);
    }
}
