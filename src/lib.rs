//! # Predicate
//! Use enum to predicate something.
//!
//! Just need to implement Predicate Trait with [predicate-macros](https://github.com/Spxg/predicate-macros) crate, support | and & operator.
//! 
//! **Don't implement traits by self.**
//! 
//! How to work: <https://github.com/Spxg/predicate/blob/master/how_to_work.png>.
//!
//! ## Feature
//! * rc (default)
//! * arc
//!
//! Enable Arc Feature:
//! ```toml
//! [dependencies.predicate]
//! version = "0.1"
//! default-features = false
//! features = ["arc"]
//! ```

#[cfg(test)]
mod tests;

/// A thread-safe reference-counting pointer. 'Arc' stands for 'Atomically
/// Reference Counted'.
/// Enable by feature 'arc'.
#[cfg(feature = "arc")]
pub type OpUnitRcType<T> = std::sync::Arc<T>;

/// A single-threaded reference-counting pointer. 'Rc' stands for 'Reference
/// Counted'.
/// Enable by deafult or feature 'rc'.
#[cfg(feature = "rc")]
pub type OpUnitRcType<T> = std::rc::Rc<T>;

/// Operation Type.
///
/// `A & B = A And B`
///
/// `A | B = A Or B`
///
/// `A = A Single None`
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    And,
    Or,
    Single,
}

/// Wrap `OpUnitRcType<T>`.
///
/// Use to store `OpUnitRcType<T>` and generate predicate closure.
pub struct PredicateRet<T> {
    inner: OpUnitRcType<T>,
}

impl<T> PredicateRet<T>
where
    T: Predicate,
{
    /// Wrap `OpUnitRcType<T>` by `new()` method.
    pub fn new(inner: OpUnitRcType<T>) -> PredicateRet<T> {
        PredicateRet { inner }
    }

    /// Get inner by Clone.
    ///
    /// Invoking `clone` on `OpUnitRcType` produces
    /// a new `OpUnitRcType` instance, which points to the same allocation on the heap as the
    /// source `OpUnitRcType`, while increasing a reference count.
    pub fn get_inner_by_clone(&self) -> OpUnitRcType<T> {
        self.inner.clone()
    }

    /// Generate double ref predicate closure.
    /// Double ref means that the input type is &&Item.
    /// Like this:
    /// ```
    /// type Item = i32;
    /// let x = vec![1, 2, 3, 4];
    /// let y = x.iter()
    ///     .filter(|n: &&Item| **n % 2 == 0)
    ///     .map(|n| *n).collect::<Vec<_>>();
    /// assert_eq!(y, vec![2, 4]);
    /// ```
    pub fn predicate_ref_double(&self) -> Box<dyn Fn(&&<T as Predicate>::Item) -> bool> {
        let root = self.get_inner_by_clone();
        let f = move |item: &&<T as Predicate>::Item| root.get_op_unit().check(&item);
        Box::new(f)
    }

    /// Generate one ref predicate closure.
    /// One ref means that the input type is &Item.
    /// Like this:
    /// ```
    /// type Item = i32;
    /// let x = vec![1, 2, 3, 4];
    /// let y = x.into_iter()
    ///     .filter(|n: &Item| *n % 2 == 0)
    ///     .collect::<Vec<_>>();
    /// assert_eq!(y, vec![2, 4]);
    /// ```
    pub fn predicate_ref_one(&self) -> Box<dyn Fn(&<T as Predicate>::Item) -> bool> {
        let root = self.get_inner_by_clone();
        let f = move |item: &<T as Predicate>::Item| root.get_op_unit().check(&item);
        Box::new(f)
    }

    /// Generate self predicate closure.
    /// Self means that the input type is Item.
    /// Like this:
    /// ```
    /// type Item = i32;
    /// let f = |x: Item| x % 2 == 0;
    /// assert!(f(1024));
    /// ```
    pub fn predicate_self(&self) -> Box<dyn Fn(<T as Predicate>::Item) -> bool> {
        let root = self.get_inner_by_clone();
        let f = move |item: <T as Predicate>::Item| root.get_op_unit().check(&item);
        Box::new(f)
    }
}

/// `OpUnit<T>` is a tree structure.
///
/// Operate with `Operation` type. If type is Single, that means it's leaf node.
/// It's a recursive computation.
///
/// [how_to_work](https://github.com/Spxg/predicate/blob/master/how_to_work.png)
pub struct OpUnit<T> {
    op: Operation,
    lhs: Option<OpUnitRcType<T>>,
    rhs: Option<OpUnitRcType<T>>,
}

impl<T> OpUnit<T>
where
    T: Predicate,
{
    /// Produce an instance by `new()` method.
    ///
    /// If op type is `And` or `Or`, `lhs` and `rhs` **must** have value (Not None).
    ///
    /// If op type is `Single`, `lhs` **must** have value (Not None).
    pub fn new(
        lhs: Option<OpUnitRcType<T>>,
        rhs: Option<OpUnitRcType<T>>,
        op: Operation,
    ) -> OpUnit<T> {
        OpUnit { op, lhs, rhs }
    }

    /// Get lhs and rhs by clone().
    ///
    /// In fact, lhs **always** have value and rhs may be `None` in `Single` type.
    pub fn get_lhs_and_rhs(&self) -> (Option<OpUnitRcType<T>>, Option<OpUnitRcType<T>>) {
        (self.lhs.clone(), self.rhs.clone())
    }

    /// Check logic.
    ///
    /// If lhs or rhs is `None` in `And` Or `Or` type, it will panic with msg.
    ///
    /// If lhs is `None` in `Single` type, it will panic with msg.
    pub fn check(&self, item: &<T as Predicate>::Item) -> bool {
        let (lhs, rhs) = self.get_lhs_and_rhs();

        match &self.op {
            Operation::And => {
                let lhs = lhs.expect("lhs is none");
                let rhs = rhs.expect("rhs is none");
                lhs.get_op_unit().check(item) && rhs.get_op_unit().check(item)
            }
            Operation::Or => {
                let lhs = lhs.expect("lhs is none");
                let rhs = rhs.expect("rhs is none");
                lhs.get_op_unit().check(item) || rhs.get_op_unit().check(item)
            }
            Operation::Single => {
                let lhs = lhs.expect("lhs is none");
                lhs.as_ref().rules(item)
            }
        }
    }
}

impl<T> std::fmt::Debug for OpUnit<T>
where
    T: std::fmt::Debug,
{
    /// impl `Debug` for `OpUnit<T>` if `T` impl `Debug`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpUnit")
            .field("op", &self.op)
            .field("lhs", &self.lhs)
            .field("rhs", &self.rhs)
            .finish()
    }
}

/// `OpUnitTrait` is used to get op unit.
pub trait OpUnitTrait: Sized {
    fn get_op_unit(self: &OpUnitRcType<Self>) -> OpUnitRcType<OpUnit<Self>>;
}

/// Only need to impl `Predicate` with `rules` method for `T` if `T` has impl OpUnitTrait.
pub trait Predicate: OpUnitTrait + 'static {
    type Item;

    /// Define check rules.
    fn rules(&self, item: &Self::Item) -> bool;

    /// Wrap `PredicateRet<T>` with `wrap_ret()` method.
    ///
    /// `PredicateRet<Self>` can use many times to generate predicate closure.  
    fn wrap_ret(self) -> PredicateRet<Self> {
        PredicateRet::new(OpUnitRcType::new(self))
    }
}
