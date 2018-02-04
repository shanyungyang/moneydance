# moneydance-rs

[Moneydance](http://moneydance.com/) is a financial software made by [The Infinite Kind](https://infinitekind.com/). This crate can read raw JSON file exported from Moneydance so you can easily transfer your financial data to another software.

## Sample Code

``` rust
extern crate moneydance;

use std::fs::File;

fn main() {
    let input = File::Open("exported_data.json").unwrap();
    let db = moneydance::Database::load(input).unwrap();
    
    println!("Read {} accounts:", db.accounts.len());
    
    for (_, account) in &db.accounts {
        println!("{}", account.name);
    }

    println!("Read {} transactions.", db.transactions.len());
}
```