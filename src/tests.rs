use super::*;

#[test]
fn test_account_import() {
    let input = include_str!("test_account_import.json");
    let db = Database::load(input.as_bytes()).unwrap();

    let mut income_num = 0;
    let mut expense_num = 0;

    for acct in db.accounts.values() {
        use super::AccountInfo::*;
        match  &acct.info {
            &Bank(_) => {
                assert!(acct.name == "Checking");
            }
            &CreditCard(_) => {
                assert!(acct.name == "My Credit Card");
            },
            &Investment(_) => {
                assert!(acct.name == "My Fund");
            },
            &Asset(_) => {
                assert!(acct.name == "My Asset");
            },
            &Loan(_) => {
                assert!(acct.name == "House Loan");
            },
            &Liability(_) => {
                assert!(acct.name == "Some Liability");
            },
            &Root => {
                assert!(acct.name == "Personal Finances");
            },
            &Expense(_) => { expense_num += 1; },
            &Income(_) => { income_num += 1; },
        }
    }
    assert!(expense_num == 9);
    assert!(income_num == 12);
}