use proc_macro::token_stream::IntoIter as TTIter;
use proc_macro::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};

mod logfunc;
use logfunc::LogFunction;
mod entryiter;
use entryiter::{EntryIter, EntryLine};
mod entryvaluebuilder;
mod typefns;

#[cfg(debug_assertions)]
#[proc_macro]
pub fn debug(ts: TokenStream) -> TokenStream {
    log(ts, build_log_level("Debug"))
}

#[cfg(not(debug_assertions))]
#[proc_macro]
pub fn debug(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn info(ts: TokenStream) -> TokenStream {
    log(ts, build_log_level("Info"))
}

#[proc_macro]
pub fn warn(ts: TokenStream) -> TokenStream {
    log(ts, build_log_level("Warn"))
}

#[proc_macro]
pub fn error(ts: TokenStream) -> TokenStream {
    log(ts, build_log_level("Error"))
}

fn log(ts: TokenStream, level: Vec<TokenTree>) -> TokenStream {
    let mut it = ts.into_iter();

    // the first tt MUST be a static str
    let msg = match it.next() {
        None => panic!("log macros must have message"),
        Some(tt) => {
            if let TokenTree::Literal(lit) = tt {
                lit.to_string()
            } else {
                panic!("log macros must have message");
            }
        }
    };

    let mut log_function = LogFunction::new(level, msg);

    // Following the message we either have EOS or a comma
    if let Some(tt) = it.next() {
        if let TokenTree::Punct(pct) = tt {
            if pct.as_char() == ',' {
                add_log_entries(it, &mut log_function);
            } else {
                panic!(", is only valid tt following log message");
            }
        } else {
            panic!(", is only valid tt following log message");
        }
    }

    log_function.into_token_stream()
}

fn add_log_entries(mut it: TTIter, log_function: &mut LogFunction) {
    // We MUST have exactly ont tt in the iterator, a group
    match it.next() {
        None => panic!("expected log entries {{}} following comma"),
        Some(tt) => {
            if let TokenTree::Group(grp) = tt {
                add_log_entries_from_group(grp.stream(), log_function);
            } else {
                panic!("expected log entries {{}} following comma")
            }
        }
    }

    if it.next().is_some() {
        panic!("log macros cannot have tokens following entries {{}}");
    }
}

fn add_log_entries_from_group(ts: TokenStream, log_function: &mut LogFunction) {
    for entry_line in EntryIter::new(ts.into_iter()) {
        match entry_line {
            EntryLine::Entry(e) => log_function.add_entry(e.into_entry_args()),
            EntryLine::EntryArray(ea) => log_function.add_entry(ea.into_entry_args()),
        }
    }
}

// return logger::Level::Info logger::Level::Debug etc.
fn build_log_level(level: &str) -> Vec<TokenTree> {
    vec![
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Level", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(level, Span::call_site())),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
