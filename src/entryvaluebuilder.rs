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
            panic!("Option<_> not supported inside array");
        }
        (true, false) => {
            panic!("Option<_> not supported inside array");
        }
        (false, true) => entry_str_array(value_tts),
        (false, false) => {
            let (atom_type, primative_cast) = entry_type.atom_type();
            entry_primative_array(atom_type, primative_cast, value_tts)
        }
    }
}

// entry_opt_str builds the Value an Option<&str>
// ndjsonloggercore::Value::Optatom({$value}.map(|s| ndjsonloggercore::Atom::String(s))
fn entry_opt_str(value_tts: &[TokenTree]) -> TokenStream {
    let mut stream = new_ndjsoncore_value("Optatom");

    let mut map_fn = new_mapfn("s", "String");
    map_fn.extend([new_single_ident_group("s", false, None)]);

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

// entry_opt_primative builds the Value of an Option<${prim}>
// where prim is i8..i64, u8..u64, bool, usize, f16..f64
// ndjsonloggercore::Value::Optatom(${value}.map(|p| ndjsonloggercore::Atom::${prim}(s as ${primative_cast}))
fn entry_opt_primative(
    atom_type: &str,
    primative_cast: Option<&str>,
    value_tts: &[TokenTree],
) -> TokenStream {
    let mut stream = new_ndjsoncore_value("Optatom");

    let mut map_fn = new_mapfn("p", atom_type);
    map_fn.extend([new_single_ident_group("p", false, primative_cast)]);

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

// entry_str builds the Value for &str
// ndjsonloggercore::Value::Atom(ndjsonloggercore::Atom::String(${value})
fn entry_str(value_tts: &[TokenTree]) -> TokenStream {
    let mut stream = new_ndjsoncore_value("Atom");

    let mut inner = TokenStream::new();
    inner.extend(value_tts.iter().map(|s| s.to_owned()));

    let mut atom = new_ndjsoncore_atom("String");
    atom.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, inner))]);

    stream.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, atom))]);
    stream
}

// entry_primative builds the Value for a primative
// ndjsonloggercore::Value::Atom(ndjsonloggercore::Atom::${atom_type}(*${value} as ${primative_cast})
fn entry_primative(
    atom_type: &str,
    primative_cast: Option<&str>,
    value_tts: &[TokenTree],
) -> TokenStream {
    let mut stream = new_ndjsoncore_value("Atom");

    let mut inner = TokenStream::new();
    inner.extend(value_tts.iter().map(|p| p.to_owned()));

    if let Some(primative_cast) = primative_cast {
        inner.extend([
            TokenTree::Ident(Ident::new("as", Span::call_site())),
            TokenTree::Ident(Ident::new(primative_cast, Span::call_site())),
        ]);
    }

    let mut atom = new_ndjsoncore_atom(atom_type);
    atom.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, inner))]);

    stream.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, atom))]);
    stream
}

// entry_str_array builds the Value for a [&str]
// ndjsonloggercore::Value::Array(
//    &mut ${value}.iter().map(|s| ndjosnloggercore::Atom::String(s))
// )
fn entry_str_array(value_tts: &[TokenTree]) -> TokenStream {
    let mut stream = new_ndjsoncore_value("Array");

    let mut map_fn = new_mapfn("s", "String");
    map_fn.extend([new_single_ident_group("s", false, None)]);

    let mut iter = new_iter(value_tts, false);
    iter.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, map_fn))]);

    stream.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, iter))]);
    stream
}

// entry_primative_array builds the Value for a [primative]
// ndjsonloggercore::Value::Array(
//    &mut ${value}.iter().map(|p| ndjosnloggercore::Atom::String(*p as ${primative_cast}))
// )
fn entry_primative_array(
    atom_type: &str,
    primative_cast: Option<&str>,
    value_tts: &[TokenTree],
) -> TokenStream {
    let mut stream = new_ndjsoncore_value("Array");

    let mut map_fn = new_mapfn("p", atom_type);
    map_fn.extend([new_single_ident_group("p", true, primative_cast)]);

    let mut iter = new_iter(value_tts, false);
    iter.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, map_fn))]);

    stream.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, iter))]);
    stream
}

fn new_ndjsoncore_value(value_variant: &str) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.extend([
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Value", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(value_variant, Span::call_site())),
    ]);
    stream
}

fn new_ndjsoncore_atom(variant: &str) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.extend([
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Atom", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(variant, Span::call_site())),
    ]);
    stream
}

// ($ident) or (*$ident as $primative_cast)
fn new_single_ident_group(ident: &str, deref: bool, primative_cast: Option<&str>) -> TokenTree {
    let mut ident_group = TokenStream::new();
    if deref {
        ident_group.extend([TokenTree::Punct(Punct::new('*', Spacing::Alone))]);
    }
    ident_group.extend([TokenTree::Ident(Ident::new(ident, Span::call_site()))]);

    if let Some(primative_cast) = primative_cast {
        ident_group.extend([
            TokenTree::Ident(Ident::new("as", Span::call_site())),
            TokenTree::Ident(Ident::new(primative_cast, Span::call_site())),
        ]);
    }

    TokenTree::Group(Group::new(Delimiter::Parenthesis, ident_group))
}

fn new_mapfn(ident: &str, atom_type: &str) -> TokenStream {
    let mut map_fn = TokenStream::new();
    map_fn.extend([
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        TokenTree::Ident(Ident::new(ident, Span::call_site())),
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        TokenTree::Ident(Ident::new("ndjsonloggercore", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Atom", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(atom_type, Span::call_site())),
    ]);
    map_fn
}

fn new_iter(tts: &[TokenTree], flatten: bool) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.extend([
        TokenTree::Punct(Punct::new('&', Spacing::Alone)),
        TokenTree::Ident(Ident::new("mut", Span::call_site())),
    ]);

    stream.extend(tts.iter().map(|tt| tt.to_owned()));
    stream.extend([
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new("iter", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
    ]);
    if flatten {
        stream.extend([
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("flatten", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        ]);
    }
    stream.extend([
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new("map", Span::call_site())),
    ]);

    stream
}
