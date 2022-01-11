use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenStream, TokenTree};

#[allow(clippy::enum_variant_names)]
pub(super) enum Error {
    TheOnlyArgumentMustBeAStringLiteral,
    TheMacroExpectedAnArgument,
    TheMacroAcceptsOnlyOneArgument,
}

struct ArgsTokenStream(TokenStream);

impl ArgsTokenStream {
    fn new(ts: TokenStream) -> Self {
        ArgsTokenStream(ts)
    }

    fn extract_arg_token_trees(self) -> core::result::Result<ArgTokenTrees, Error> {
        let mut arg_tt_iter: TokenTreeIter = self.0.into_iter();

        let presumed_string_literal_with_path_to_tex: TokenTree = arg_tt_iter
            .next()
            .ok_or(Error::TheMacroExpectedAnArgument)?;

        if arg_tt_iter.next().is_some() {
            return Err(Error::TheMacroAcceptsOnlyOneArgument);
        };

        Ok(ArgTokenTrees {
            presumed_string_literal_with_path_to_tex,
        })
    }
}

struct ArgTokenTrees {
    presumed_string_literal_with_path_to_tex: TokenTree,
}

pub(super) struct Args {
    pub(super) presumed_path_to_tex: PresumedPathToTex,
}

pub(super) trait TryGetArgs {
    fn try_get_args(self) -> Result<Args, Error>;
}

impl From<String> for PresumedPathToTex {
    fn from(path: String) -> Self {
        PresumedPathToTex(std::path::PathBuf::from(path))
    }
}

impl TryGetArgs for ArgTokenTrees {
    fn try_get_args(self) -> core::result::Result<Args, Error> {
        // Currently, string literal is parsed twice.
        // The first time by `proc_macro` crate and the second
        // time by `syn` crate.

        let proc_macro_string_literal_with_presumed_path_to_tex =
            match self.presumed_string_literal_with_path_to_tex {
                TokenTree::Literal(lit) => lit,
                _ => return Err(Error::TheOnlyArgumentMustBeAStringLiteral),
            };
        // String representations of proc_macro::TokenTree::Literal and syn::lit::LitStr
        // are identical
        let string_repr_of_literal_with_presumed_path_to_tex: String =
            proc_macro_string_literal_with_presumed_path_to_tex.to_string();
        let syn_string_literal_with_presumed_path_to_tex: syn::LitStr =
            syn::parse_str(string_repr_of_literal_with_presumed_path_to_tex.as_str())
                .map_err(|_| Error::TheOnlyArgumentMustBeAStringLiteral)?;
        Ok(Args {
            presumed_path_to_tex: PresumedPathToTex::from(
                syn_string_literal_with_presumed_path_to_tex.value(),
            ),
        })
    }
}

impl TryGetArgs for TokenStream {
    fn try_get_args(self) -> Result<Args, Error> {
        ArgsTokenStream::new(self)
            .extract_arg_token_trees()?
            .try_get_args()
    }
}

pub(super) struct PresumedPathToTex(pub(super) std::path::PathBuf);
