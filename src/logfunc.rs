use std::str::FromStr;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

pub struct LogFunction {
    level: Vec<TokenTree>,
    msg: String,
    entries: Vec<TokenStream>,
}

impl LogFunction {
    pub fn new(level: Vec<TokenTree>, msg: String) -> Self {
        Self {
            level,
            msg,
            entries: vec![],
        }
    }

    pub fn into_token_stream(self) -> TokenStream {
        let mut out = TokenStream::new();
        let mut stdout_log_args = TokenStream::new();

        // msg
        stdout_log_args.extend([
            TokenTree::Literal(Literal::from_str(&self.msg).expect("invalid msg string literal")),
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
        ]);

        // level
        stdout_log_args.extend(self.level);
        stdout_log_args.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);

        // Tags
        stdout_log_args.extend([
            TokenTree::Group(Group::new(Delimiter::Bracket, entries_group(self.entries))),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("into_iter", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        ]);

        out.extend([
            TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("stdout_log", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, stdout_log_args)),
        ]);

        out
    }

    pub fn add_entry(&mut self, ts: TokenStream) {
        self.entries.push(ts);
    }
}

fn entries_group(entries: Vec<TokenStream>) -> TokenStream {
    let mut stream = TokenStream::new();

    for (n, e) in entries.into_iter().enumerate() {
        if n != 0 {
            stream.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
        }

        stream.extend([
            TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("Entry", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Brace, e)),
        ]);
    }

    stream
}
