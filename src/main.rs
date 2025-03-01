use monkey::repl::{self, repl::start};

fn main() {
    println!("Hello, world!");
    start();
    let slice = vec![1, 2, 3, 4, 5];
    let doubled = slice.iter().map(|x| {
        println!("正在处理: {}", x);
        x * 2
    });
    for value in doubled {
        println!("最终值: {}", value);
    }
}
