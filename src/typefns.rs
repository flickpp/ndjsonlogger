use proc_macro::TokenTree;

#[derive(Clone, Copy)]
pub struct EntryType {
    atom_type: AtomType,
    opt: bool,
}

impl EntryType {
    pub fn new() -> Self {
        Self {
            atom_type: AtomType::String,
            opt: false,
        }
    }

    pub fn from_type(tts: &[&TokenTree]) -> Self {
        for (atom_type, type_fn) in TYPE_FNS.iter() {
            if type_fn(tts) {
                return Self {
                    atom_type: *atom_type,
                    opt: false,
                };
            }

            if type_fn_opt(tts, type_fn) {
                return Self {
                    atom_type: *atom_type,
                    opt: true,
                };
            }
        }

        panic!("unrecognised type for entry value");
    }

    pub fn is_opt(self) -> bool {
        self.opt
    }

    pub fn is_str(self) -> bool {
        self.atom_type == AtomType::String
    }

    pub fn atom_type(self) -> (&'static str, Option<&'static str>) {
        self.atom_type.atom_type()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AtomType {
    String,
    U64,
    I64,
    U32,
    I32,
    U16,
    I16,
    U8,
    I8,
    Usize,
    Bool,
}

impl AtomType {
    fn atom_type(self) -> (&'static str, Option<&'static str>) {
        match self {
            AtomType::String => ("String", None),
            AtomType::U64 => ("Uint", None),
            AtomType::I64 => ("Int", None),
            AtomType::U32 => ("Uint", Some("u64")),
            AtomType::I32 => ("Int", Some("i64")),
            AtomType::U16 => ("Uint", Some("u64")),
            AtomType::I16 => ("Int", Some("i64")),
            AtomType::U8 => ("Uint", Some("u64")),
            AtomType::I8 => ("Int", Some("i64")),
            AtomType::Usize => ("Uint", Some("u64")),
            AtomType::Bool => ("Bool", None),
        }
    }
}

#[allow(clippy::type_complexity)]
const TYPE_FNS: &[(AtomType, fn(&[&TokenTree]) -> bool)] = &[
    (AtomType::String, type_fn_string),
    (AtomType::U64, type_fn_u64),
    (AtomType::I64, type_fn_i64),
    (AtomType::U32, type_fn_u32),
    (AtomType::I32, type_fn_i32),
    (AtomType::U16, type_fn_u16),
    (AtomType::I16, type_fn_i16),
    (AtomType::U8, type_fn_u8),
    (AtomType::I8, type_fn_i8),
    (AtomType::Usize, type_fn_usize),
    (AtomType::Bool, type_fn_bool),
];

fn type_fn_string(tts: &[&TokenTree]) -> bool {
    if tts.len() != 2 {
        return false;
    }

    if let Some(TokenTree::Punct(pct)) = tts.get(0) {
        if pct.as_char() == '&' {
            if let Some(TokenTree::Ident(ident)) = tts.get(1) {
                if ident.to_string() == "str" {
                    return true;
                }
            }
        }
    }

    false
}

fn type_fn_u64(tts: &[&TokenTree]) -> bool {
    type_fn_single_ident(tts, "u64")
}

fn type_fn_i64(tts: &[&TokenTree]) -> bool {
    type_fn_single_ident(tts, "i64")
}

fn type_fn_u32(tts: &[&TokenTree]) -> bool {
    type_fn_single_ident(tts, "u32")
}

fn type_fn_i32(tts: &[&TokenTree]) -> bool {
    type_fn_single_ident(tts, "i32")
}

fn type_fn_u16(tts: &[&TokenTree]) -> bool {
    type_fn_single_ident(tts, "u16")
}

fn type_fn_i16(tts: &[&TokenTree]) -> bool {
    type_fn_single_ident(tts, "i16")
}

fn type_fn_u8(tts: &[&TokenTree]) -> bool {
    type_fn_single_ident(tts, "u8")
}

fn type_fn_i8(tts: &[&TokenTree]) -> bool {
    type_fn_single_ident(tts, "i8")
}

fn type_fn_usize(tts: &[&TokenTree]) -> bool {
    type_fn_single_ident(tts, "usize")
}

fn type_fn_bool(tts: &[&TokenTree]) -> bool {
    type_fn_single_ident(tts, "bool")
}

fn type_fn_single_ident(tts: &[&TokenTree], ident_str: &str) -> bool {
    if tts.len() != 1 {
        return false;
    }

    if let Some(TokenTree::Ident(ident)) = tts.get(0) {
        if ident.to_string() == ident_str {
            return true;
        }
    }

    false
}

fn type_fn_opt(tts: &[&TokenTree], inner_type_fn: &fn(&[&TokenTree]) -> bool) -> bool {
    if tts.len() < 4 {
        return false;
    }

    if let Some(TokenTree::Ident(ident)) = tts.get(0) {
        if ident.to_string() == "Option" {
            if let Some(TokenTree::Punct(pct)) = tts.get(1) {
                if pct.as_char() == '<' {
                    if let Some(TokenTree::Punct(pct)) = tts.get(tts.len() - 1) {
                        if pct.as_char() == '>' {
                            return inner_type_fn(&tts[2..(tts.len() - 1)]);
                        }
                    }
                }
            }
        }
    }

    false
}
