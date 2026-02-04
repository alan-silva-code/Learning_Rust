fn main(){
    /* 
    let mut _x: i32 = 5;
    let _r: &mut i32 = &mut _x;
    *_r += 1;
    println!("The value of _x is {}", _x);
    */

    let mut account: BankAccount = BankAccount{
        owner: "Alice".to_string(),
        balance: 876.32,
    };

    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(50.00);
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}