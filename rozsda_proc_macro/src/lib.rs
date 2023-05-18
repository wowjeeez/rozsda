rozsda_fordito::rozsda! {
    használ procedurális_makró::{Csoport, Azonosító, ZsetonFa, ZsetonFolyó};

    függvény azonosító_cseréje(azonosító: Azonosító) -> Opció<ZsetonFa> {
        legyen lánc_azonosító = azonosító.szöveggé();

        legyen lánc_azonosító = pár lánc_azonosító.szövegként() {
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
            _ => &lánc_azonosító,
        };

        legyen új_azonosító = Azonosító::új(lánc_azonosító, azonosító.befog());
        Valami(ZsetonFa::Azonosító(új_azonosító))
    }

    függvény fa_cseréje(zseton: ZsetonFa, kimenent: &mutálható Lista<ZsetonFa>) {
        pár zseton {
            ZsetonFa::Csoport(csoport) => {
                legyen mutálható csoport_tagok = Lista::új();
                folyó_csere(csoport.folyó(), &mutálható csoport_tagok);
                legyen mutálható zseton_folyó = ZsetonFolyó::új();
                zseton_folyó.kiterjeszt(csoport_tagok);
                kimenent.nyomás(ZsetonFa::Csoport(Csoport::új(csoport.delimitáló(), zseton_folyó)));
            }
            ZsetonFa::Azonosító(azonosító) => {
                ha legyen Valami(azonosító) = azonosító_cseréje(azonosító) {
                    kimenent.nyomás(azonosító);
                }
            }
            ZsetonFa::Interpunkció(..) | ZsetonFa::SzóSzerint(..) => {
                kimenent.nyomás(zseton);
            }
        }
    }

    függvény folyó_csere(zseton_folyó: ZsetonFolyó, kimenet: &mutálható Lista<ZsetonFa>) {
        ciklus zseton ban zseton_folyó {
            fa_cseréje(zseton, kimenet)
        }
    }

    #[procedurális_makró]
    publikus függvény rozsda(elem: ZsetonFolyó) -> ZsetonFolyó {
        legyen mutálható vissza = Lista::új();
        folyó_csere(elem, &mutálható vissza);
        legyen mutálható kimenet = ZsetonFolyó::új();
        kimenet.kiterjeszt(vissza);
        kimenet
    }
}
