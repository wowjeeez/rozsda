use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Hibás" => "Err",
        "Jó" => "Ok",
        "Szöveg" => "String",
        "Szótár" => "HashMap",
        "Alapértelmezett" => "Default",
        "Hiba" => "Error",
        "Opció" => "Option",
        "Valami" => "Some",
        "Semmi" => "None",
        "Eredmény" => "Result",
        "Magam" => "Self",
        "sornyomtat" => "println",
        "tör" => "break",
        "aszinkron" => "async",
        "megvár" => "await",
        "hurok" => "loop",
        "mozgat" => "move",
        "láda" => "crate",
        "kód_nem_elérhető" => "unreachable_code",
        "ahogy" => "as",
        "konstans" => "const",
        "tulajdonság" => "trait",
        "veszélyes" => "unsafe",
        "ban" => "in",
        "ből" => "from",
        "dinamikus" => "dyn",
        "kicsomagol" => "unwrap",
        "alapértelmezett" => "default",
        "refként" => "as_ref",
        "io" => "io",
        "külső" => "extern",
        "hamis" => "false",
        "függvény" => "fn",
        "szuper" => "super",
        "beilleszt" => "insert",
        "kap" => "get",
        "engedélyez" => "allow",
        "fasz" | "kurva_anyád" | "a_gecibe" | "hoppá" => "panic",
        "modul" => "mod",
        "mutálható" => "mut",
        "új" => "new",
        "ahol" => "where",
        "ciklus" => "for",
        "kap_vagy_beilleszt" => "get_or_insert_with",
        "fő" => "main",
        "publikus" => "pub",
        "he" => None?,
        "cserébe" => "return",
        "implementáció" => "impl",
        "referencia" => "ref",
        "pár" => "match",
        "ha" => "if",
        "különben" => "else",
        "magam" => "self",
        "legyen" => "let",
        "statikus" => "static",
        "struktúra" => "struct",
        "feltételez" => "expect",
        "amíg" => "while",
        "használ" => "use",
        "ebbe" => "into",
        "igaz" => "true",
        "enumeráció" => "enum",
        "Csoport" => "Group",
        "Azonosító" => "Ident",
        "ZsetonFolyó" => "TokenStream",
        "ZsetonFa" => "TokenTree",
        "szöveggé" => "to_string",
        "szövegként" => "as_str",
        "befog" => "span",
        "Lista" => "Vec",
        "folyó" => "stream",
        "nyomás" => "push",
        "kiterjeszt" => "extend",
        "delimitáló" => "delimiter",
        "Interpunkció" => "Punct",
        "SzóSzerint" => "Literal",
        "procedurális_makró" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rozsda(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
