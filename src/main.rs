use rustlisp::value::Value;
use rustlisp::env::Env;

fn main() {
    let bottom = &Env::bottom();
    let e = Env::new("aaa".to_string(), &Value::Nil, &bottom);
    println!("{}", e.name);
}
