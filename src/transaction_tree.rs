use crate::date::Date;
use crate::money::Money;
use crate::tag::{Tag, TagSlice};
use crate::transaction::Transaction;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct TransactionTree<'a> {
    tree: BTreeMap<TagSlice<'a>, Category<'a>>,
}

#[derive(Debug)]
enum Category<'a> {
    Leaf {
        limit: Option<Money>,
        spent: Money,
        transactions: Vec<&'a Transaction>,
    },
    NonLeaf {
        limit: Option<Money>,
        spent: Money,
    },
}

impl<'a> Category<'a> {
    fn new_non_leaf() -> Category<'a> {
        Self::NonLeaf {
            limit: None,
            spent: Money::from_cents(0),
        }
    }

    fn new_leaf() -> Category<'a> {
        Self::Leaf {
            limit: None,
            spent: Money::from_cents(0),
            transactions: Vec::new(),
        }
    }

    fn limit(&self) -> Option<Money> {
        match *self {
            Category::Leaf { limit, .. } | Category::NonLeaf { limit, .. } => limit,
        }
    }

    fn spent(&self) -> Money {
        match *self {
            Category::Leaf { spent, .. } | Category::NonLeaf { spent, .. } => spent,
        }
    }

    fn percent_for_month(&self) -> Option<i64> {
        self.spent().checked_div(self.limit()?)
    }

    fn left_to_date(&self) -> Option<Money> {
        // TODO: really this should validate that we are currently in the
        // month corresponding to this TransactionTree, and should probably error
        // or be none if we are not.

        Money::left_to_date_in_month(Date::now(), self.limit()?, self.spent())
    }
}

impl<'a> TransactionTree<'a> {
    pub fn from_transactions_and_limits(
        transactions: &'a Vec<Transaction>,
        limits: &'a HashMap<Tag, Money>,
    ) -> Result<TransactionTree<'a>, ()> {
        // as you go through the transactions, as you go up the parents, ensure each parent has no transactions, otherwise it violates constraints

        let mut map = BTreeMap::new();

        for t in transactions {
            let leaf_tag = t.tag();

            let mut leaf = map.entry(leaf_tag).or_insert_with(Category::new_leaf);
            match &mut leaf {
                Category::NonLeaf { .. } => todo!("Add an error here."),
                Category::Leaf {
                    spent,
                    transactions,
                    ..
                } => {
                    *spent += t.amount;
                    transactions.push(&t);
                }
            }

            for p in leaf_tag.parents() {
                dbg!("Adding", p);

                let mut non_leaf = map.entry(p).or_insert_with(Category::new_non_leaf);
                match &mut non_leaf {
                    Category::Leaf { .. } => todo!("Add error here"),
                    Category::NonLeaf { spent, .. } => *spent += t.amount,
                }
            }
        }

        for (tag, amount) in limits {
            let tag = tag.as_slice();

            let mut entry = map.entry(tag).or_insert_with(Category::new_non_leaf);
            match &mut entry {
                Category::NonLeaf { limit, .. } | Category::Leaf { limit, .. } => {
                    *limit = Some(*amount)
                }
            }

            for p in tag.parents() {
                dbg!("Adding for limits", p);

                let mut non_leaf = map.entry(p).or_insert_with(Category::new_non_leaf);
                match &mut non_leaf {
                    Category::Leaf { .. } => todo!("Add error here"),
                    Category::NonLeaf { .. } => {}
                }
            }
        }

        for (_k, value) in &mut map {
            if let Category::Leaf { transactions, .. } = value {
                transactions.sort();
            }
        }

        return Ok(TransactionTree { tree: map });
    }
}

impl<'a> Display for TransactionTree<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (tag, category) in &self.tree {
            // create display for categories, then just print each category
            // if there's no limit, just print the amount spent for the month

            let indentation = tag.depth().checked_mul(2).ok_or(fmt::Error)?;

            write!(f, "{}{}", " ".repeat(indentation), tag)?;

            let left_to_date = category.left_to_date();
            let percent_for_month = category.percent_for_month();
            let limit = category.limit();

            match (left_to_date, percent_for_month, limit) {
                (Some(left), Some(percent), Some(limit)) => write!(
                    f,
                    ": {} left to date, {}% of {} used for the month",
                    left, percent, limit
                )?,
                (_, _, _) => {}
            }

            write!(f, "\n")?;

            if let Category::Leaf { transactions, .. } = category {
                for t in transactions.iter().take(3) {
                    write!(
                        f,
                        "{}• {}",
                        " ".repeat(indentation.checked_add(2).ok_or(fmt::Error)?),
                        t
                    )?;
                }
            }
        }
        Ok(())
    }
}
