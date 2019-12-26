fn fun_test(value: i32, f: impl Fn(i32) -> i32) -> i32 {
    println!("{}", f(value));
    value
}

fn times2(value: i32) -> i32 {
    2 * value
}

fn main() {
    fun_test(5, |x| x*3);
}
