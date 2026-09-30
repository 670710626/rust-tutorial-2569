use std::ops::Add;

// [INCORRECT] คอมไพล์ไม่ผ่านหากลืม Trait Bound:
// fn calculate_sum_incorrect<T>(a: T, b: T) -> T {
//     a + b // ERROR: cannot add `T` to `T`
// }

// [CORRECT] กำหนด Trait Bound ให้กับ Generic Type T:
pub fn calculate_sum<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let sum_int = calculate_sum(10, 25);
    println!("Sum of integers: {}", sum_int);

    let sum_float = calculate_sum(12.5, 7.5);
    println!("Sum of floats: {}", sum_float);
}
