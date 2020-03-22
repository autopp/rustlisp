use super::value::Value;

pub struct Env<'a> {
  name: String,
  value: Value,
  prev: Option<&'a Env<'a>>
}

impl<'a> Env<'a> {
  pub fn bottom() -> Env<'a> {
    Env {
      name: "".to_string(),
      value: Value::Nil,
      prev: None
    }
  }

  pub fn new(name: String, value: Value, prev: &'a Env) -> Env<'a> {
    Env {
      name: name,
      value: value,
      prev: Some(prev)
    }
  }
}
