use syn::{
    Ident, Token, braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub(crate) struct TeeServiceInput {
    pub(crate) commands: Vec<Ident>,
    pub(crate) context: Option<Ident>,
}

mod keywords {
    syn::custom_keyword!(context);
    syn::custom_keyword!(commands);
}

impl Parse for TeeServiceInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut commands: Option<Vec<Ident>> = None;
        let mut context: Option<Ident> = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keywords::context) {
                if context.is_some() {
                    return Err(syn::Error::new(input.span(), "Duplicate 'context' section"));
                }
                input.parse::<keywords::context>()?;
                let ctx_ident: Ident = input.parse()?;
                context = Some(ctx_ident);
            } else if lookahead.peek(keywords::commands) {
                if commands.is_some() {
                    return Err(syn::Error::new(
                        input.span(),
                        "Duplicate 'commands' section",
                    ));
                }
                input.parse::<keywords::commands>()?;
                let content;
                braced!(content in input);
                let idents = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;
                if idents
                    .iter()
                    .any(|ident| ident.to_string().eq_ignore_ascii_case("unknown"))
                {
                    return Err(syn::Error::new(
                        input.span(),
                        "Command name 'Unknown' is reserved for the default case",
                    ));
                }
                commands = Some(idents.into_iter().collect());
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(TeeServiceInput {
            commands: commands.unwrap_or_default(),
            context,
        })
    }
}
