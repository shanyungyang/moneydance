extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate time;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_type;

use time::strptime;

error_type!{
    #[derive(Debug)]
    pub enum Error {
        IoError(std::io::Error) {},
        JsonError(serde_json::Error) {},
        ParseIntError(std::num::ParseIntError) {},
        ParseFloatError(std::num::ParseFloatError) {},
        ParseUuidError(uuid::ParseError) {},
        ParseDateError(time::ParseError) {},
        OtherError(&'static str) {
            desc(e) e;
        },
    }
}

pub type Result<T> = std::result::Result<T, Error>;

mod rawjson;
mod models;
pub use models::*;

impl Currency {
    fn new(curr: rawjson::Currency) -> Result<Self> {
        Ok(Self {
            code: curr.currid,
            decimal: curr.dec.parse()?,
            rate: curr.rate.parse()?,
        })
    }
}


use std::rc::Rc;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Database {
    pub accounts: HashMap<Uuid, Rc<Account>>,
    pub currencies: HashMap<Uuid, Rc<Currency>>,
    pub transactions: Vec<Transaction>,
}

impl Database {
    pub fn load<R: std::io::Read>(input: R) -> Result<Database> {
        let raw: rawjson::ExportedData = serde_json::de::from_reader(input)?;
        let (mut accounts, currencies, transactions) = sort_exported_data(raw);

        let mut db = Database {
            accounts: HashMap::new(),
            currencies: HashMap::new(),
            transactions: Vec::new(),
        };

        for curr in currencies {
            db.currencies.insert(curr.id, Rc::new(Currency::new(curr)?));
        }

        while accounts.len() > 0 {
            let id = match accounts.keys().next() {
                Some(id) => *id,
                None => { break; },
            };

            db.add_accounts(&mut accounts, id)?;
        }

        for txn in transactions {
            db.import_splits(txn)?;

        }
        return Ok(db);
    }

    fn add_accounts(&mut self, parent_set: &mut HashMap<Uuid, rawjson::Account>, id: Uuid) -> Result<Rc<Account>> {
        if let Some(acct) = self.accounts.get(&id) {
            return Ok(acct.clone()); // already added
        }

        let acct = parent_set.remove(&id).ok_or("account not exist")?;

        let result = match acct {
            rawjson::Account::R(root) => Rc::new(self.new_root_account(root)?),
            _ => {
                let parentid = acct.parent().unwrap();
                let parent = self.add_accounts(parent_set, parentid)?;
                Rc::new(self.new_account(acct, parent)?)
            },
        };

        self.accounts.insert(id, result.clone());
        return Ok(result);        
    }

    fn import_splits(&mut self, mut orig: HashMap<String,String>) -> Result<()> {
        let acctid = orig.get("acctid")
                         .map(|s| Uuid::parse_str(&s))
                         .ok_or("invalid_acctid field")??;
        
        let stat = match orig.get("stat") {
            Some(s) if s == "X" => Status::Reconciled,
            _ => Status::None,
        };
        let desc = orig.remove("desc").ok_or("invalid desc field in transaction")?;

        let date = orig.get("dt").map(|s| strptime(&s, "%Y%m%d"))
                                 .ok_or("invalid dt field in transaction")??;

        let mut splits = Vec::new();
        for (k,v) in orig {
            let mut ds = k.split('.');
            match (ds.next(), ds.next()) {
                (Some(digits), Some(remain)) => {
                    if let Ok(index) = digits.parse::<usize>() {
                        if index >= splits.len() {
                            splits.resize(index+1, rawjson::Txn::default());
                        }
                        match remain {
                            "acctid" => {
                                splits[index].acctid = Uuid::parse_str(&v)?;
                            },
                            "pamt" => {
                                splits[index].pamt = v;
                            },
                            "samt" => {
                                splits[index].samt = v;
                            },
                            "tags" => {
                                splits[index].tags = v;
                            },
                            "desc" => {
                                splits[index].desc = v;
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }

        let mut txn_splits = Vec::new();
        for t in splits {
            txn_splits.push(Split {
                receiver: self.get_account(t.acctid)?,
                received_amount: t.pamt.parse()?,
                given_amount: t.samt.parse()?,
                tags: t.tags,
                description: t.desc,
            });
        }

        let giver = self.get_account(acctid)?;

        self.transactions.push(Transaction {
            giver: giver,
            date: date,
            status: stat,
            description: desc,
            splits: txn_splits,
        });

        return Ok(());
    }

    fn get_currency(&self, id: Uuid) -> Result<Rc<Currency>> {
        return Ok(self.currencies.get(&id).ok_or("currency not exist")?.clone());
    }

    fn get_account(&self, id: Uuid) -> Result<Rc<Account>> {
        return Ok(self.accounts.get(&id).ok_or("account not exist")?.clone());
    }


    fn new_root_account(&self, acct: rawjson::RootAccount) -> Result<Account> {
        let curr = self.currencies.get(&acct.currid).ok_or("currency not exist")?;
        return Ok(Account {
            info: AccountInfo::Root,
            currency: curr.clone(),
            comment: String::default(),
            initial: 0,
            name: acct.name,
        });
    }

    fn new_asset_account(&self, asset: rawjson::AssetAccount, parent: Rc<Account>) -> Result<Account> {
        Ok(Account{
            info: AccountInfo::Asset(AssetInfo{ parent: parent }),
            currency: self.get_currency(asset.currid)?,
            comment: asset.comment.unwrap_or_default(),
            initial: asset.sbal.parse()?,
            name: asset.name,
        })
    }

    fn new_bank_account(&self, bank: rawjson::BankAccount, parent: Rc<Account>) -> Result<Account> {
        Ok(Account{
            info: AccountInfo::Bank(
                BankInfo{ 
                    parent: parent,
                    account_number: bank.bank_account_number.unwrap_or_default(),
                    bank_name: bank.bank_name.unwrap_or_default(),
                }),
            currency: self.get_currency(bank.currid)?,
            name: bank.name,
            comment: bank.comment.unwrap_or_default(),
            initial: bank.sbal.parse()?,    
        })
    }

    fn new_credit_card_account(&self, card: rawjson::CreditCardAccount, parent: Rc<Account>) -> Result<Account> {
        Ok(Account{
            info: AccountInfo::CreditCard(
                CreditCardInfo{
                    parent: parent,
                    bank_name: card.bank_name.unwrap_or_default(),
                }),
            currency: self.get_currency(card.currid)?,
            name: card.name,
            comment: card.comment.unwrap_or_default(),
            initial: card.sbal.parse()?,
        })
    }

    fn new_income_account(&self, income: rawjson::IncomeAccount, parent: Rc<Account>) -> Result<Account> {
        Ok(Account{
            info: AccountInfo::Income(IncomeInfo{ parent: parent }),
            currency: self.get_currency(income.currid)?,
            name: income.name,
            comment: income.comment.unwrap_or_default(),
            initial: income.sbal.parse()?,
        })
    }

    fn new_expense_account(&self, expense: rawjson::ExpenseAccount, parent: Rc<Account>) -> Result<Account> {
        Ok(Account{
            info: AccountInfo::Expense(ExpenseInfo{ parent: parent }),
            currency: self.get_currency(expense.currid)?,
            name: expense.name,
            comment: expense.comment.unwrap_or_default(),
            initial: expense.sbal.parse()?,
        })
    }

    fn new_liability_account(&self, liability: rawjson::LiabilityAccount, parent: Rc<Account>) -> Result<Account> {
        Ok(Account{
            info: AccountInfo::Liability(LiabilityInfo{ parent: parent }),
            currency: self.get_currency(liability.currid)?,
            name: liability.name,
            comment: liability.comment.unwrap_or_default(),
            initial: liability.sbal.parse()?,
        })
    }

    fn new_investment_account(&self, investment: rawjson::InvestmentAccount, parent: Rc<Account>) -> Result<Account> {
        Ok(Account{
            info: AccountInfo::Investment(InvestmentInfo{ parent: parent }),
            currency: self.get_currency(investment.currid)?,
            name: investment.name,
            comment: investment.comment.unwrap_or_default(),
            initial: investment.sbal.parse()?,
        })
    }

    fn new_loan_account(&self, loan: rawjson::LoanAccount, parent: Rc<Account>) -> Result<Account> {
        Ok(Account{
            info: AccountInfo::Loan(
                LoanInfo{ 
                    parent: parent,
                    init_principal: loan.init_principal.parse()?,
                }),
            currency: self.get_currency(loan.currid)?,
            name: loan.name,
            comment: loan.comment.unwrap_or_default(),
            initial: loan.sbal.parse()?,
        })
    }

    fn new_account(&self, acct: rawjson::Account, parent: Rc<Account>) -> Result<Account> {
        use rawjson::Account::*;
        match acct {
            A(asset) => self.new_asset_account(asset, parent),
            B(bank) => self.new_bank_account(bank, parent),
            C(card) => self.new_credit_card_account(card, parent),
            E(expense) => self.new_expense_account(expense, parent),
            I(income) => self.new_income_account(income, parent),
            L(liability) => self.new_liability_account(liability, parent),
            O(loan) => self.new_loan_account(loan, parent),
            V(investment) => self.new_investment_account(investment, parent),
            R(_) => Err(Error::OtherError("root account should not have parent")),
        }
    }
}

fn sort_exported_data(raw: rawjson::ExportedData) -> (HashMap<Uuid, rawjson::Account>, Vec<rawjson::Currency>, Vec<HashMap<String,String>>) {
    let mut accounts = HashMap::new();
    let mut currencies = Vec::new();
    let mut transactions = Vec::new();

    use rawjson::Item::*;

    for item in raw.all_items {
        match item {
            Acct(a) => { accounts.insert(a.id(), a); },
            Txn(t) => { transactions.push(t); },
            Curr(c) => { currencies.push(c); },
            _ => {},
        }
    }

    return (accounts, currencies, transactions);
}

#[cfg(test)]
mod tests;
