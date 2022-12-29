#[cfg(test)]
mod basic {
  use crud_macro::Trait;

  #[derive(Trait)]
  struct MyStruct{}

  #[test]
  pub fn test() {
  }
}
