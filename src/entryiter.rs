use proc_macro::token_stream::IntoIter as TTIter;
use proc_macro::{Delimiter, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use std::str::FromStr;

use crate::entryvaluebuilder::{build_entry_value, build_entry_value_array};
use crate::typefns::EntryType;

pub enum EntryLine {
    Entry(Entry),
    EntryArray(EntryArray),
}

pub struct EntryIter {
    entries: Vec<Vec<TokenTree>>,
}

impl EntryIter {
    pub fn new(it: TTIter) -> Self {
        let mut entries = vec![];
        let mut e = vec![];

        // Split on our commas
        for tt in it {
            if let TokenTree::Punct(ref pct) = tt {
                if pct.as_char() == ',' {
                    entries.push(e);
                    e = vec![];
                    continue;
                }
            }

            e.push(tt);
        }

        entries.push(e);

        Self { entries }
    }
}

impl Iterator for EntryIter {
    type Item = EntryLine;

    fn next(&mut self) -> Option<Self::Item> {
        if self.entries.is_empty() {
            return None;
        }
        let entry = parse_entry(self.entries.remove(0));
        Some(entry)
    }
}

fn parse_entry(mut tts: Vec<TokenTree>) -> EntryLine {
    if tts.len() == 1 {
        match tts.remove(0) {
            TokenTree::Ident(ident) => return EntryLine::Entry(ident_entry(ident)),
            TokenTree::Group(grp) => {
                if grp.delimiter() == Delimiter::Bracket {
                    // Array
                    return parse_array(grp.stream());
                }

                panic!("invalid log entry, expected ident or group");
            }
            _ => panic!("invalid log entry, exected ident or group"),
        }
    }

    // The first tt is the key - may be either an ident or a literal
    let key = match tts.get(0).unwrap() {
        TokenTree::Literal(lit) => lit.to_string(),
        TokenTree::Ident(ident) => format!("\"{}\"", ident),
        _ => panic!("invalid tt - expected ident or literal for entry key"),
    };

    // Second tt may be either an = or a :
    let (entry_type, value_tts) = if let Some(TokenTree::Punct(ref pct)) = tts.get(1) {
        if pct.as_char() == '=' {
            // The value is all tt after the =
            (EntryType::new(), &tts[2..])
        } else if pct.as_char() == ':' {
            // Parse the type
            let (entry_type, num_tts) = parse_type(&tts[2..]);
            (entry_type, &tts[(num_tts + 2)..])
        } else {
            panic!("expected : or = following entry key");
        }
    } else {
        panic!("expected : or = following entry key");
    };

    EntryLine::Entry(Entry::new(key, entry_type, value_tts))
}

fn parse_array(stream: TokenStream) -> EntryLine {
    let tts = stream.into_iter().collect::<Vec<TokenTree>>();
    // The first tt is the key - may be either an ident or a literal
    let key = match tts.get(0).unwrap() {
        TokenTree::Literal(lit) => lit.to_string(),
        TokenTree::Ident(ident) => format!("\"{}\"", ident),
        _ => panic!("invalid tt - expected ident or literal for entry key"),
    };

    // Second tt may be either an = or a :
    let (entry_type, value_tts) = if let Some(TokenTree::Punct(ref pct)) = tts.get(1) {
        if pct.as_char() == '=' {
            // The value is all tt after the =
            (EntryType::new(), &tts[2..])
        } else if pct.as_char() == ':' {
            // Parse the type
            let (entry_type, num_tts) = parse_type(&tts[2..]);

            (entry_type, &tts[(num_tts + 2)..])
        } else {
            panic!("expected : or = following entry key");
        }
    } else {
        panic!("expected : or = following entry key");
    };

    EntryLine::EntryArray(EntryArray::new(key, entry_type, value_tts))
}

fn parse_type(tts: &[TokenTree]) -> (EntryType, usize) {
    let mut type_tts = vec![];
    let mut num_tts = 0;
    let mut found_eq = false;

    for (n, tt) in tts.iter().enumerate() {
        if let TokenTree::Punct(pct) = tt {
            if pct.as_char() == '=' {
                num_tts = n + 1;
                found_eq = true;
                break;
            }
        }

        type_tts.push(tt);
    }

    if !found_eq {
        panic!("expected = tt following type declaration");
    }

    (EntryType::from_type(&type_tts), num_tts)
}

fn ident_entry(ident: Ident) -> Entry {
    Entry::new(
        format!("\"{}\"", ident),
        EntryType::new(),
        &[TokenTree::Ident(ident)],
    )
}

pub struct Entry {
    key: String,
    value_group: TokenStream,
}

impl Entry {
    fn new(key: String, entry_type: EntryType, value_tts: &[TokenTree]) -> Self {
        Self {
            key,
            value_group: build_entry_value(entry_type, value_tts),
        }
    }

    pub fn into_entry_args(self) -> TokenStream {
        new_entry_args(&self.key, self.value_group)
    }
}

pub struct EntryArray {
    key: String,
    value_group: TokenStream,
}

impl EntryArray {
    fn new(key: String, entry_type: EntryType, value_tts: &[TokenTree]) -> Self {
        Self {
            key,
            value_group: build_entry_value_array(entry_type, value_tts),
        }
    }

    pub fn into_entry_args(self) -> TokenStream {
        new_entry_args(&self.key, self.value_group)
    }
}

fn new_entry_args(key: &str, value: TokenStream) -> TokenStream {
    let mut entry_args = TokenStream::new();
    entry_args.extend([
        TokenTree::Ident(Ident::new("key", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Literal(Literal::from_str(key).expect("invalid entry key")),
        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
        TokenTree::Ident(Ident::new("value", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
    ]);
    entry_args.extend(value);

    entry_args
}
