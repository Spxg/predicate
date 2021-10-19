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

pub struct PredicateRet<T> {
    inner: OpUnitRcType<T>,
}

impl<T> PredicateRet<T>
where
    T: Predicate,
{
    pub fn new(inner: OpUnitRcType<T>) -> PredicateRet<T> {
        PredicateRet { inner }
    }

    pub fn get_inner_by_clone(&self) -> OpUnitRcType<T> {
        self.inner.clone()
    }

    pub fn predicate_ref_double(&self) -> Box<dyn Fn(&&<T as Predicate>::Item) -> bool> {
        let root = self.get_inner_by_clone();
        let f = move |item: &&<T as Predicate>::Item| root.get_op_unit().check(&item);
        Box::new(f)
    }

    pub fn predicate_ref_one(&self) -> Box<dyn Fn(&<T as Predicate>::Item) -> bool> {
        let root = self.get_inner_by_clone();
        let f = move |item: &<T as Predicate>::Item| root.get_op_unit().check(&item);
        Box::new(f)
    }

    pub fn predicate_self(&self) -> Box<dyn Fn(<T as Predicate>::Item) -> bool> {
        let root = self.get_inner_by_clone();
        let f = move |item: <T as Predicate>::Item| root.get_op_unit().check(&item);
        Box::new(f)
    }
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

    pub fn get_lhs_and_rhs(&self) -> (Option<OpUnitRcType<T>>, Option<OpUnitRcType<T>>) {
        (self.lhs.clone(), self.rhs.clone())
    }

    pub fn check(&self, item: &<T as Predicate>::Item) -> bool {
        let (lhs, rhs) = self.get_lhs_and_rhs();

        match &self.op {
            Operation::And => {
                let lhs = lhs.expect("lhs is none");
                let rhs = rhs.expect("rhs is none");
                lhs.get_op_unit().check(item) && rhs.get_op_unit().check(item)
            },
            Operation::Or => {
                let lhs = lhs.expect("lhs is none");
                let rhs = rhs.expect("rhs is none");
                lhs.get_op_unit().check(item) || rhs.get_op_unit().check(item)
            },
            Operation::Single => {
                let lhs = lhs.expect("lhs is none");
                lhs.as_ref().rules(item)
            },
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

pub trait OpUnitTrait: Sized {
    fn get_op_unit(self: &OpUnitRcType<Self>) -> OpUnitRcType<OpUnit<Self>>;
}

pub trait Predicate: OpUnitTrait + 'static {
    type Item;

    fn rules(&self, item: &Self::Item) -> bool;

    fn wrap_ret(self) -> PredicateRet<Self> {
        PredicateRet::new(OpUnitRcType::new(self))
    }
}
