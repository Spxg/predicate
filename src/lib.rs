#[cfg(test)]
mod tests;

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
    T: Predicate + Default,
{
    pub fn new(lhs: Option<T>, rhs: Option<T>, op: Operation) -> OpUnit<T> {
        OpUnit { op, lhs, rhs }
    }

    pub fn get_lhs_and_rhs(&mut self) -> (T, T) {
        let lhs = self.lhs.take().unwrap_or_default();
        let rhs = self.rhs.take().unwrap_or_default();
        (lhs, rhs)
    }

    pub fn check(&mut self, item: &<T as Predicate>::Item) -> bool {
        let (lhs, rhs) = self.get_lhs_and_rhs();
        match &self.op {
            Operation::And => lhs.get_op_unit().check(item) && rhs.get_op_unit().check(item),
            Operation::Or => lhs.get_op_unit().check(item) || rhs.get_op_unit().check(item),
            Operation::Single => lhs.rules(item),
        }
    }
}

pub trait OpUnitTrait: Sized + Default {
    fn get_op_unit(&self) -> OpUnit<Self>;
}

pub trait Predicate: OpUnitTrait + 'static {
    type Item;

    fn rules(&self, item: &Self::Item) -> bool;
    fn predicate_ref_double(self) -> Box<dyn FnMut(&&Self::Item) -> bool> {
        let f = move |item: &&Self::Item| self.get_op_unit().check(item);
        Box::new(f)
    }
    fn predicate_ref_one(self) -> Box<dyn FnMut(&Self::Item) -> bool> {
        let f = move |item: &Self::Item| self.get_op_unit().check(item);
        Box::new(f)
    }
    fn predicate_self(self) -> Box<dyn FnMut(Self::Item) -> bool> {
        let f = move |item: Self::Item| self.get_op_unit().check(&item);
        Box::new(f)
    }
}
