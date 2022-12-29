use std::collections::HashMap;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Crud, attributes(entity, model))]
pub fn derive_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    dbg!(name.to_owned());
    dbg!(input.attrs[0].path.is_ident("entity"));
    // dbg!(input.attrs[0].);

    // let mut hashMap = HashMap::new();
    // input.attrs.iter().fold(&mut hashMap, | attr | {
    //     match input.attrs[0].path {
            
    //     };
    //     hashMap.insert(k, v)
    // });

    let expanded = quote! {
        use async_trait::async_trait;

        #[async_trait]
        trait Crud {
            fn test();
            // async fn find_by_id(db: &DbConnection) -> Result<model::Type, ()>;
        }

        #[async_trait]
        impl Crud for #name {
            fn test() {
                println!("{}", "hello from #name");
            }

            // async fn find_by_id(db: &DbConnection) -> Result<model::Type, ()> {
            //     entity::Type::find_by_id(id).one(db).await
            // }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
