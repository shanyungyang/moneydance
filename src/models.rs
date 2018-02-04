use std::rc::Rc;
use time::Tm;

pub struct Currency {
    pub code: String, // ISO 4217 Currency Code
    pub rate: f64,
    pub decimal: i32,
}

pub struct BankInfo {
    pub bank_name: String,
    pub account_number: String,
    pub parent: Rc<Account>,
}

pub struct CreditCardInfo {
    pub bank_name: String,
    pub parent: Rc<Account>,
}

pub struct InvestmentInfo {
    pub parent: Rc<Account>,
}

pub struct AssetInfo {
    pub parent: Rc<Account>,
}

pub struct LiabilityInfo {
    pub parent: Rc<Account>,
}

pub struct LoanInfo {
    pub parent: Rc<Account>,
    pub init_principal: i64,
}

pub struct IncomeInfo {
    pub parent: Rc<Account>,
}

pub struct ExpenseInfo {
    pub parent: Rc<Account>,
}

pub enum AccountInfo {
    Bank(BankInfo),
    CreditCard(CreditCardInfo),
    Investment(InvestmentInfo),
    Asset(AssetInfo),
    Liability(LiabilityInfo),
    Loan(LoanInfo),
    Income(IncomeInfo),
    Expense(ExpenseInfo),
    Root,
}

pub struct Account {
    pub info: AccountInfo,
    pub name: String,
    pub initial: i64,
    pub currency: Rc<Currency>,
    pub comment: String,
}

pub struct Split {
    pub receiver: Rc<Account>, // destination account
    pub given_amount: i64,
    pub received_amount: i64,
    pub tags: String,
    pub description: String,
}

pub enum Status {
    None,
    Cleared,
    Reconciled,
}

pub struct Transaction {
    pub giver: Rc<Account>, // source account
    pub description: String,
    pub splits: Vec<Split>,
    pub date: Tm,
    pub status: Status,
}
