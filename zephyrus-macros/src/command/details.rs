use darling::FromMeta;
use darling::export::NestedMeta;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::Token;
use syn::{Attribute, Result};
use syn::punctuated::Punctuated;

use crate::extractors::{Either, FixedList, FunctionPath, Ident, List};

#[derive(Default, FromMeta)]
/// The details of a given command
pub struct CommandDetails {
    /// The description of this command
    pub description: Either<String, FixedList<1, String>>,
    #[darling(default)]
    pub required_permissions: Option<List<Ident>>,
    #[darling(default)]
    pub checks: Either<List<FunctionPath>, Punctuated<FunctionPath, Token![,]>>,
    #[darling(default)]
    pub error_handler: Option<Either<FunctionPath, FixedList<1, FunctionPath>>>,
    #[darling(default)]
    pub nsfw: bool,
    #[darling(default)]
    pub only_guilds: bool
}

impl CommandDetails {
    pub fn parse(attrs: &mut Vec<Attribute>) -> Result<Self> {
        let meta = attrs
            .drain(..)
            .map(|item| item.meta)
            .map(NestedMeta::Meta)
            .collect::<Vec<_>>();

        Self::from_list(meta.as_slice())
            .map_err(From::from)
    }
}

impl ToTokens for CommandDetails {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let d = self.description.inner();
        tokens.extend(quote::quote!(.description(#d)));

        if let Some(permissions) = &self.required_permissions {
            let mut permission_stream = TokenStream2::new();

            for (index, permission) in permissions.iter().enumerate() {
                if index == 0 || permissions.len() == 1 {
                    permission_stream
                        .extend(quote::quote!(zephyrus::twilight_exports::Permissions::#permission))
                } else {
                    permission_stream.extend(
                        quote::quote!( | zephyrus::twilight_exports::Permissions::#permission),
                    )
                }
            }

            tokens.extend(quote::quote!(.required_permissions(#permission_stream)));
        }

        let mut checks = Vec::new();
        self.checks.map_1(
            &mut checks,
            |checks, a| checks.extend(a.iter().cloned()),
            |checks, b| checks.extend(b.iter().cloned())
        );

        tokens.extend(quote::quote! {
            .checks(vec![#(#checks()),*])
        });

        if let Some(error_handler) = &self.error_handler {
            let error_handler = error_handler.inner();
            tokens.extend(quote::quote!(.error_handler(#error_handler())));
        }

        let nsfw = self.nsfw;
        let only_guilds = self.only_guilds;

        tokens.extend(quote::quote!(
            .nsfw(#nsfw)
            .only_guilds(#only_guilds)
        ));
    }
}