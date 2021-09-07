#[cfg(test)]
mod tests;

use std::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    And,
    Or,
    Single,
}

#[derive(Debug, Clone)]
pub struct OpUnit<T> {
    op: Operation,
    lhs: Option<T>,
    rhs: Option<T>,
}

impl<T> OpUnit<T>
where
    T: Filter,
{
    pub fn new(lhs: Option<T>, rhs: Option<T>, op: Operation) -> OpUnit<T> {
        OpUnit { op, lhs, rhs }
    }

    pub fn op(&mut self, item: &<T as Filter>::Item) -> bool {
        let mut inner = || {
            let lhs = self.lhs.take().unwrap_or_default();
            let rhs = self.rhs.take().unwrap_or_default();
            (lhs, rhs)
        };

        match self.op {
            Operation::And => {
                let (lhs, rhs) = inner();
                lhs.op_unit().op(item) & rhs.op_unit().op(item)
            }
            Operation::Or => {
                let (lhs, rhs) = inner();
                lhs.op_unit().op(item) | rhs.op_unit().op(item)
            }
            Operation::Single => {
                let (lhs, _) = inner();
                lhs.rules(item)
            }
        }
    }
}

pub trait OpUnitTrait: Sized {
    fn op_unit(&self) -> OpUnit<Self>;
}

pub trait Filter: OpUnitTrait + BitAnd + BitOr + Clone + Default + 'static {
    type Item;

    fn rules(&self, item: &Self::Item) -> bool;
    fn ref_double_filter(self) -> Box<dyn FnMut(&&Self::Item) -> bool> {
        let f = move |item: &&Self::Item| self.op_unit().op(item);
        Box::new(f)
    }
    fn ref_one_filter(self) -> Box<dyn FnMut(&Self::Item) -> bool> {
        let f = move |item: &Self::Item| self.op_unit().op(item);
        Box::new(f)
    }
    fn self_filter(self) -> Box<dyn FnMut(Self::Item) -> bool> {
        let f = move |item: Self::Item| self.op_unit().op(&item);
        Box::new(f)
    }
}
