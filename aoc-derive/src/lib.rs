use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitInt, Token,
};

struct DayRange {
    start: LitInt,
    end: LitInt,
    function_ident: Ident,
}

impl Parse for DayRange {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let start = input.parse()?;
        input.parse::<Token![,]>()?;
        let end = input.parse()?;
        input.parse::<Token![,]>()?;
        let function_ident = input.parse()?;

        Ok(Self {
            start,
            end,
            function_ident,
        })
    }
}

#[proc_macro]
pub fn build_days(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DayRange {
        start,
        end,
        function_ident,
    } = parse_macro_input!(input as DayRange);

    let start_n: usize = start.base10_parse().unwrap();
    let end_n: usize = end.base10_parse().unwrap();
    let struct_idents = (start_n..=end_n)
        .map(|i| format_ident!("Day{}", format!("{}", i)))
        .collect::<Vec<Ident>>();

    let imports = (start_n..=end_n).map(|i| {
        let crate_ident = format_ident!("day{}", format!("{}", i));
        let struct_ident = &struct_idents[i - start_n];

        quote! {
            mod #crate_ident;
            use #crate_ident::#struct_ident;
        }
    });

    let match_cases = (start_n..=end_n).map(|i| {
        let struct_ident = &struct_idents[i - start_n];

        quote! {
            #i => #struct_ident::#function_ident,
        }
    });

    let q = quote! {
        #(#imports)*

        pub fn get_runner<R: ::std::io::BufRead>(day: usize) -> fn(R) {
            match day {
                #(#match_cases)*
                _ => todo!("This day is not implemented yet"),
            }
        }
    };

    q.into()
}
