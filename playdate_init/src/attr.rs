use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::{punctuated::Punctuated, Ident, LitStr, Result, Token};

#[derive(Debug)]
pub(crate) struct AttrPair {
    pub name: Ident,
    pub _eq: Token![=],
    pub value: LitStr,
}

impl Parse for AttrPair {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            name: Ident::parse(input)?,
            _eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub(crate) struct AppArgs {
    pub init_name: String,
    pub update_name: String,
    pub state_name: Option<String>,
}

impl Parse for AppArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let pairs = Punctuated::<AttrPair, Token![,]>::parse_terminated(input)?;
        let mut args = HashMap::new();

        for attr in pairs {
            let key = attr.name.to_string();
            if args.contains_key(&key) {
                panic!("argument {} specified multiple times", attr.name);
            }

            args.insert(key, attr.value);
        }

        let init_name = args
            .remove("init")
            .expect("argument 'init' is missing")
            .value();
        let update_name = args
            .remove("update")
            .expect("argument 'update' is missing")
            .value();
        let state_name = args.remove("state").map(|s| s.value());

        Ok(Self {
            init_name,
            update_name,
            state_name,
        })
    }
}
