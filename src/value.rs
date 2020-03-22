pub enum Arity {
  Fixed { n: i64 },
  Variable { min: i64 },
  Range { min: i64, max: i64 }
}

pub enum Value {
    Nil,
    Sym { s: String },
    Num { n: i64 },
    Bool { b: bool },
    Pair { car: Box<Value>, cdr: Box<Value> },
    SpecialForm { name: String, arity: Arity, body: Box<dyn Fn(&Vec<&Value>) -> Value> }
}
