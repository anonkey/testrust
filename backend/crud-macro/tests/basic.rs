#[cfg(test)]
mod basic {
  use crud_macro::Crud;

  #[derive(Debug)]
  struct PostEntiy {}

  #[derive(Debug)]
  struct PostModel {}

  #[derive(Crud)]
  #[entity("Test")]
  #[model(PostModel)]
  struct MyStruct {
    id: String,
  }

  #[test]
  pub fn test() {
    MyStruct::test();
  }
}
