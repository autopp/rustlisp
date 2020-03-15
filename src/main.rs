fn main() {
    println!("Hello, world!");
}

enum SExpr {
    Nil,
    Pair { car: Box<SExpr>, cdr: Box<SExpr> }
}
