# Filter
Use enum to filter something, support | and & operator.

Just need to implement Filter Trait with [filter-macros](https://github.com/Spxg/filter-macros) crate.

## How to work
![how_to_work](https://github.com/Spxg/filter/blob/master/how_to_work.png)

## Example
```rust
#[add_fields]
#[derive(BitAnd, BitOr, OpUnitTrait)]
enum NumType {
    Odd,
    Even,
    DivByThree,
    DivByFour,
    DivByFive,
    IsMagicNum(i32),
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
            NumType::IsMagicNum(num) => item == num,
            _ => false,
        }
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 9, 12, 15, 16, 20, 22, 24, 1024];
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

    let test = NumType::IsMagicNum(1024);
    assert!(test.self_filter()(1024));
}
```
