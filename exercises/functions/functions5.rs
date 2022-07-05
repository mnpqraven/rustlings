// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)


fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    // this also works
    // reasons: return ; is a statement, num * num is a operand, rust differentiates these two
    // return num * num;
    num * num
}
