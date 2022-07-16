use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::typefns::EntryType;

pub fn build_entry_value(entry_type: EntryType, value_tts: &[TokenTree]) -> TokenStream {
    match (entry_type.is_opt(), entry_type.is_str()) {
        (true, true) => {
            // Option<&str>
            entry_opt_str(value_tts)
        }
        (true, false) => {
            let (atom_type, primative_cast) = entry_type.atom_type();
            entry_opt_primative(atom_type, primative_cast, value_tts)
        }
        (false, true) => entry_str(value_tts),
        (false, false) => {
            let (atom_type, primative_cast) = entry_type.atom_type();
            entry_primative(atom_type, primative_cast, value_tts)
        }
    }
}

pub fn build_entry_value_array(entry_type: EntryType, value_tts: &[TokenTree]) -> TokenStream {
    match (entry_type.is_opt(), entry_type.is_str()) {
        (true, true) => {
            // Option<&str>
            entry_opt_str(value_tts)
        }
        (true, false) => {
            let (atom_type, primative_cast) = entry_type.atom_type();
            entry_opt_primative(atom_type, primative_cast, value_tts)
        }
        (false, true) => entry_str_array(value_tts),
        (false, false) => {
            let (atom_type, primative_cast) = entry_type.atom_type();
            entry_primative_array(atom_type, primative_cast, value_tts)
        }
    }
}

fn entry_opt_str(value_tts: &[TokenTree]) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.extend([
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Value", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Optatom", Span::call_site())),
    ]);

    let mut s_ident = TokenStream::new();
    s_ident.extend([TokenTree::Ident(Ident::new("s", Span::call_site()))]);

    let mut map_fn = TokenStream::new();
    map_fn.extend([
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        TokenTree::Ident(Ident::new("s", Span::call_site())),
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Atom", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("String", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, s_ident)),
    ]);

    let mut str_option = TokenStream::new();
    str_option.extend(value_tts.iter().map(|tt| tt.to_owned()));
    str_option.extend([
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new("map", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, map_fn)),
    ]);

    stream.extend([TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        str_option,
    ))]);

    stream
}

fn entry_opt_primative(
    atom_type: &str,
    primative_cast: Option<&str>,
    value_tts: &[TokenTree],
) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.extend([
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Value", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Optatom", Span::call_site())),
    ]);

    let mut p_ident = TokenStream::new();
    p_ident.extend([
        TokenTree::Punct(Punct::new('*', Spacing::Alone)),
        TokenTree::Ident(Ident::new("p", Span::call_site())),
    ]);

    if let Some(primative_cast) = primative_cast {
        p_ident.extend([
            TokenTree::Ident(Ident::new("as", Span::call_site())),
            TokenTree::Ident(Ident::new(primative_cast, Span::call_site())),
        ]);
    }

    let mut map_fn = TokenStream::new();
    map_fn.extend([
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        TokenTree::Ident(Ident::new("p", Span::call_site())),
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Atom", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(atom_type, Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, p_ident)),
    ]);

    let mut option = TokenStream::new();
    option.extend(value_tts.iter().map(|tt| tt.to_owned()));
    option.extend([
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new("map", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, map_fn)),
    ]);

    stream.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, option))]);

    stream
}

fn entry_str(value_tts: &[TokenTree]) -> TokenStream {
    let mut stream = TokenStream::new();

    stream.extend([
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Value", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Atom", Span::call_site())),
    ]);

    let mut s_ident = TokenStream::new();
    s_ident.extend(value_tts.iter().map(|s| s.to_owned()));

    let mut atom_str = TokenStream::new();
    atom_str.extend([
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Atom", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("String", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, s_ident)),
    ]);

    stream.extend([TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        atom_str,
    ))]);
    stream
}

fn entry_primative(
    atom_type: &str,
    primative_cast: Option<&str>,
    value_tts: &[TokenTree],
) -> TokenStream {
    let mut stream = TokenStream::new();

    stream.extend([
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Value", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Atom", Span::call_site())),
    ]);

    let mut p_ident = TokenStream::new();
    p_ident.extend(value_tts.iter().map(|p| p.to_owned()));

    if let Some(primative_cast) = primative_cast {
        p_ident.extend([
            TokenTree::Ident(Ident::new("as", Span::call_site())),
            TokenTree::Ident(Ident::new(primative_cast, Span::call_site())),
        ]);
    }

    let mut atom_str = TokenStream::new();
    atom_str.extend([
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Atom", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(atom_type, Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, p_ident)),
    ]);

    stream.extend([TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        atom_str,
    ))]);
    stream
}

fn entry_str_array(value_tts: &[TokenTree]) -> TokenStream {
    let mut stream = TokenStream::new();

    stream.extend([
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Value", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Array", Span::call_site())),
    ]);

    let mut p_ident = TokenStream::new();
    p_ident.extend([TokenTree::Ident(Ident::new("s", Span::call_site()))]);

    let mut map_fn = TokenStream::new();
    map_fn.extend([
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        TokenTree::Ident(Ident::new("s", Span::call_site())),
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Atom", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("String", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, p_ident)),
    ]);

    let mut iter = TokenStream::new();
    iter.extend([
        TokenTree::Punct(Punct::new('&', Spacing::Alone)),
        TokenTree::Ident(Ident::new("mut", Span::call_site())),
    ]);
    iter.extend(value_tts.iter().map(|p| p.to_owned()));
    iter.extend([
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new("iter", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new("map", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, map_fn)),
    ]);

    stream.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, iter))]);
    stream
}

fn entry_primative_array(
    atom_type: &str,
    primative_cast: Option<&str>,
    value_tts: &[TokenTree],
) -> TokenStream {
    let mut stream = TokenStream::new();

    stream.extend([
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Value", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Array", Span::call_site())),
    ]);

    let mut p_ident = TokenStream::new();
    p_ident.extend([
        TokenTree::Punct(Punct::new('*', Spacing::Alone)),
        TokenTree::Ident(Ident::new("p", Span::call_site())),
    ]);

    if let Some(primative_cast) = primative_cast {
        p_ident.extend([
            TokenTree::Ident(Ident::new("as", Span::call_site())),
            TokenTree::Ident(Ident::new(primative_cast, Span::call_site())),
        ]);
    }

    let mut map_fn = TokenStream::new();
    map_fn.extend([
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        TokenTree::Ident(Ident::new("p", Span::call_site())),
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Atom", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(atom_type, Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, p_ident)),
    ]);

    let mut iter = TokenStream::new();
    iter.extend([
        TokenTree::Punct(Punct::new('&', Spacing::Alone)),
        TokenTree::Ident(Ident::new("mut", Span::call_site())),
    ]);
    iter.extend(value_tts.iter().map(|p| p.to_owned()));
    iter.extend([
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new("iter", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new("map", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, map_fn)),
    ]);

    stream.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, iter))]);
    stream
}
