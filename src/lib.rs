#[cfg(test)]
mod tests;

#[cfg(feature = "arc")]
pub type OpUnitRcType<T> = std::sync::Arc<T>;

#[cfg(feature = "rc")]
pub type OpUnitRcType<T> = std::rc::Rc<T>;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    And,
    Or,
    Single,
}

pub struct OpUnit<T> {
    op: Operation,
    lhs: Option<OpUnitRcType<T>>,
    rhs: Option<OpUnitRcType<T>>,
}

impl<T> OpUnit<T>
where
    T: Predicate,
{
    pub fn new(
        lhs: Option<OpUnitRcType<T>>,
        rhs: Option<OpUnitRcType<T>>,
        op: Operation,
    ) -> OpUnit<T> {
        OpUnit { op, lhs, rhs }
    }

    pub fn get_lhs_and_rhs(&self) -> (OpUnitRcType<T>, OpUnitRcType<T>) {
        let default = OpUnitRcType::new(T::default());
        let lhs = match self.lhs.as_ref() {
            Some(lhs) => lhs.clone(),
            None => default.clone(),
        };
        let rhs = match self.rhs.as_ref() {
            Some(rhs) => rhs.clone(),
            None => default,
        };
        (lhs, rhs)
    }

    pub fn check(&self, item: &<T as Predicate>::Item) -> bool {
        let (lhs, rhs) = self.get_lhs_and_rhs();

        match &self.op {
            Operation::And => lhs.get_op_unit().check(item) && rhs.get_op_unit().check(item),
            Operation::Or => lhs.get_op_unit().check(item) || rhs.get_op_unit().check(item),
            Operation::Single => lhs.as_ref().rules(item),
        }
    }
}

impl<T> std::fmt::Debug for OpUnit<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpUnit")
            .field("op", &self.op)
            .field("lhs", &self.lhs)
            .field("rhs", &self.rhs)
            .finish()
    }
}

pub trait OpUnitTrait: Sized + Default {
    fn get_op_unit(self: &OpUnitRcType<Self>) -> OpUnitRcType<OpUnit<Self>>;
}

pub trait Predicate: OpUnitTrait + 'static {
    type Item;

    fn rules(&self, item: &Self::Item) -> bool;

    fn predicate_ref_double(self) -> Box<dyn FnMut(&&Self::Item) -> bool> {
        let root_unit = OpUnitRcType::new(self);
        let f = move |item: &&Self::Item| root_unit.get_op_unit().check(item);
        Box::new(f)
    }

    fn predicate_ref_one(self) -> Box<dyn FnMut(&Self::Item) -> bool> {
        let root_unit = OpUnitRcType::new(self);
        let f = move |item: &Self::Item| root_unit.get_op_unit().check(item);
        Box::new(f)
    }

    fn predicate_self(self) -> Box<dyn FnMut(Self::Item) -> bool> {
        let root_unit = OpUnitRcType::new(self);
        let f = move |item: Self::Item| root_unit.get_op_unit().check(&item);
        Box::new(f)
    }
}
