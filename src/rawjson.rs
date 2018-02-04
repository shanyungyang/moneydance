use uuid::Uuid;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct BankAccount {
    pub id: Uuid,
    pub name: String,
    pub bank_name: Option<String>,
    pub bank_account_number: Option<String>,
    pub parentid: Uuid,
    pub currid: Uuid,
    pub sbal: String,
    pub comment: Option<String>,
}

#[derive(Deserialize)]
pub struct CreditCardAccount {
    pub id: Uuid,
    pub name: String,
    pub parentid: Uuid,
    pub currid: Uuid,
    pub sbal: String,
    pub comment: Option<String>,
    pub bank_name: Option<String>,
}

#[derive(Deserialize)]
pub struct InvestmentAccount {
    pub id: Uuid,
    pub name: String,
    pub parentid: Uuid,
    pub currid: Uuid,
    pub sbal: String,
    pub comment: Option<String>,
}

#[derive(Deserialize)]
pub struct AssetAccount {
    pub id: Uuid,
    pub name: String,
    pub parentid: Uuid,
    pub currid: Uuid,
    pub sbal: String,
    pub comment: Option<String>,
}

#[derive(Deserialize)]
pub struct LiabilityAccount {
    pub id: Uuid,
    pub name: String,
    pub parentid: Uuid,
    pub currid: Uuid,
    pub sbal: String,
    pub comment: Option<String>,
}

#[derive(Deserialize)]
pub struct LoanAccount {
    pub id: Uuid,
    pub name: String,
    pub parentid: Uuid,
    pub currid: Uuid,
    pub sbal: String,
    pub comment: Option<String>,
    pub init_principal: String,
}

#[derive(Deserialize)]
pub struct ExpenseAccount {
    pub id: Uuid,
    pub name: String,
    pub parentid: Uuid,
    pub currid: Uuid,
    pub sbal: String,
    pub comment: Option<String>,
}

#[derive(Deserialize)]
pub struct IncomeAccount {
    pub id: Uuid,
    pub name: String,
    pub parentid: Uuid,
    pub currid: Uuid,
    pub sbal: String,
    pub comment: Option<String>,
}

#[derive(Deserialize)]
pub struct RootAccount {
    pub id: Uuid,
    pub name: String,
    pub currid: Uuid,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Account {
    B(BankAccount),
    C(CreditCardAccount),
    V(InvestmentAccount),
    A(AssetAccount),
    L(LiabilityAccount),
    O(LoanAccount),
    E(ExpenseAccount),
    I(IncomeAccount),
    R(RootAccount),
}

impl Account {
    pub fn id(&self) -> Uuid {
        match self {
            &Account::B(BankAccount { id, .. }) => id,
            &Account::C(CreditCardAccount { id, .. }) => id,
            &Account::V(InvestmentAccount{ id, .. }) => id,
            &Account::A(AssetAccount { id, .. }) => id,
            &Account::L(LiabilityAccount { id, .. }) => id,
            &Account::O(LoanAccount{ id, .. }) => id,
            &Account::E(ExpenseAccount { id, .. }) => id,
            &Account::I(IncomeAccount { id, .. }) => id,
            &Account::R(RootAccount { id, .. }) => id,
        }
    }

    pub fn parent(&self) -> Option<Uuid> {
        match self {
            &Account::B(BankAccount { parentid, .. }) => Some(parentid),
            &Account::C(CreditCardAccount { parentid, .. }) => Some(parentid),
            &Account::V(InvestmentAccount { parentid, .. }) => Some(parentid),
            &Account::A(AssetAccount { parentid, .. }) => Some(parentid),
            &Account::L(LiabilityAccount { parentid, .. }) => Some(parentid),
            &Account::O(LoanAccount { parentid, .. }) => Some(parentid),
            &Account::E(ExpenseAccount { parentid, .. }) => Some(parentid),
            &Account::I(IncomeAccount { parentid, .. }) => Some(parentid),
            &Account::R(_) => None,
        }
    }
}

#[derive(Deserialize)]
pub struct Currency {
    pub id: Uuid,
    pub currid: String,
    pub rate: String,
    pub dec: String,
}

#[derive(Clone)]
pub struct Txn {
    pub acctid: Uuid,
    pub pamt: String,
    pub samt: String,
    pub tags: String,
    pub desc: String,
}

impl Default for Txn {
    fn default() -> Self {
        Txn {
            acctid: Uuid::nil(),
            pamt: String::default(),
            samt: String::default(),
            tags: String::default(),
            desc: String::default(),
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "obj_type", rename_all = "lowercase")]
pub enum Item {
    Acct(Account),                // Accounts
    Txn(HashMap<String, String>), // Transactions
    Curr(Currency),               // Currencies
    Csnap {},                     // ignored
    Oltxns {},                    // ignored
}

#[derive(Deserialize)]
pub struct ExportedData {
    pub all_items: Vec<Item>,
}
