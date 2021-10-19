use crate::{OpUnit, OpUnitRcType, OpUnitTrait, Operation, Predicate};
use predicate_macros::{add_field, BitAnd, BitOr, OpUnitTrait};

#[add_field]
#[derive(BitAnd, BitOr, OpUnitTrait)]
enum NumType {
    Odd,
    Even,
    DivByThree,
    DivByFour,
    DivByFive,
    IsMagicNum(i32),
    More(Box<dyn Fn(&i32) -> bool>),
}

impl Predicate for NumType {
    type Item = i32;

    fn rules(&self, item: &Self::Item) -> bool {
        match self {
            NumType::Odd => item % 2 != 0,
            NumType::Even => item % 2 == 0,
            NumType::DivByThree => item % 3 == 0,
            NumType::DivByFour => item % 4 == 0,
            NumType::DivByFive => item % 5 == 0,
            NumType::IsMagicNum(num) => item == num,
            NumType::More(f) => f(item),
            _ => false,
        }
    }
}

#[test]
fn filter_test() {
    let nums = vec![1, 2, 3, 4, 5, 6, 9, 12, 15, 16, 20, 22, 24, 1024];
    let test = NumType::Odd
        | NumType::Even & NumType::DivByThree & NumType::DivByFour
        | NumType::DivByFive;
    let result = nums
        .clone()
        .into_iter()
        .filter(test.wrap_ret().predicate_ref_one())
        .collect::<Vec<_>>();
    assert_eq!(vec![1, 3, 5, 9, 12, 15, 20, 24], result);

    let test = NumType::Odd & NumType::Even;
    let result = nums
        .clone()
        .into_iter()
        .filter(test.wrap_ret().predicate_ref_one())
        .collect::<Vec<_>>();
    assert!(result.is_empty());

    let test = NumType::Odd | NumType::Even;
    let result = nums
        .clone()
        .into_iter()
        .filter(test.wrap_ret().predicate_ref_one())
        .collect::<Vec<_>>();
    assert_eq!(result, nums);

    let test = NumType::DivByThree & NumType::Odd | NumType::DivByFive;
    let result = nums
        .clone()
        .iter()
        .filter(test.wrap_ret().predicate_ref_double())
        .map(|num| *num)
        .collect::<Vec<_>>();
    assert_eq!(vec![3, 5, 9, 15, 20], result);

    let test = NumType::More(Box::new(|i| i % 6 == 0));
    let ret = test.wrap_ret();
    let result = nums
        .clone()
        .into_iter()
        .filter(ret.predicate_ref_one())
        .collect::<Vec<_>>();
    assert_eq!(vec![6, 12, 24], result);
    assert!(ret.predicate_self()(36));

    let test = NumType::IsMagicNum(1024);
    assert!(test.wrap_ret().predicate_self()(1024));
}
