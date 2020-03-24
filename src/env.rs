use super::value::Value;

pub struct Env<'v, 'p> {
  pub name: String,
  pub value: &'v Value,
  pub prev: Option<&'p Env<'p, 'p>>
}

impl<'v, 'p> Env<'v, 'p> {
  pub fn bottom() -> Env<'v, 'p> {
    Env {
      name: "".to_string(),
      value: &Value::Nil,
      prev: None
    }
  }

  pub fn new(name: String, value: &'v Value, prev: &'p Env) -> Env<'v, 'p> {
    Env {
      name: name,
      value: value,
      prev: Some(prev)
    }
  }
}
