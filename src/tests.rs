use crate::{Filter, OpUnit, OpUnitTrait, Operation};
use filter_macros::{add_fields, BitAnd, BitOr, OpUnitTrait};

#[add_fields]
#[derive(BitAnd, BitOr, OpUnitTrait)]
enum NumType {
    Odd,
    Even,
    DivByThree,
    DivByFour,
    DivByFive,
}

impl Filter for NumType {
    type Item = i32;

    fn rules(&self, item: &Self::Item) -> bool {
        match self {
            NumType::Odd => item % 2 != 0,
            NumType::Even => item % 2 == 0,
            NumType::DivByThree => item % 3 == 0,
            NumType::DivByFour => item % 4 == 0,
            NumType::DivByFive => item % 5 == 0,
            _ => false,
        }
    }
}

#[test]
fn filter_test() {
    let nums = vec![1, 2, 3, 4, 5, 6, 9, 12, 15, 16, 20, 22, 24];
    let test = NumType::Odd
        | NumType::Even & NumType::DivByThree & NumType::DivByFour
        | NumType::DivByFive;
    let result = nums
        .clone()
        .into_iter()
        .filter(test.ref_one_filter())
        .collect::<Vec<_>>();
    assert_eq!(vec![1, 3, 5, 9, 12, 15, 20, 24], result);

    let test = NumType::Odd & NumType::Even;
    let result = nums
        .clone()
        .into_iter()
        .filter(test.ref_one_filter())
        .collect::<Vec<_>>();
    assert!(result.is_empty());

    let test = NumType::Odd | NumType::Even;
    let result = nums
        .clone()
        .into_iter()
        .filter(test.ref_one_filter())
        .collect::<Vec<_>>();
    assert_eq!(result, nums);

    let test = NumType::DivByThree & NumType::Odd | NumType::DivByFive;
    let result = nums
        .clone()
        .into_iter()
        .filter(test.ref_one_filter())
        .collect::<Vec<_>>();
    assert_eq!(vec![3, 5, 9, 15, 20], result);
}
