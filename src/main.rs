use rustlisp::value::Value;
use rustlisp::env::Env;

fn main() {
    let x = 10;
    let e = Env::new("aaa".to_string(), &Value::Nil, &Env::bottom());
    println!("{}, {}", x, e);
}
