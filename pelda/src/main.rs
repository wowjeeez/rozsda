rozsda::rozsda! {
    külső láda rozsda;

    használ std::collections::Szótár ahogy Könyv;

    tulajdonság KulcsÉrték {
        függvény ír(&magam, kulcs: Szöveg, érték: Szöveg);
        függvény olvas(&magam, kulcs: Szöveg) -> Eredmény<Opció<&Szöveg>, Szöveg>;
    }

    statikus mutálható SZÓTÁR: Opció<Könyv<Szöveg, Szöveg>> = Semmi;

    struktúra Beton;

    implementáció KulcsÉrték ciklus Beton { //kicsit fura elnevezés a fornak
        függvény ír(&magam, kulcs: Szöveg, érték: Szöveg) {
            legyen Könyv = veszélyes {
                SZÓTÁR.kap_vagy_beilleszt(Alapértelmezett::alapértelmezett)
            };
            Könyv.beilleszt(kulcs, érték);
        }
        függvény olvas(&magam, kulcs: Szöveg) -> Eredmény<Opció<&Szöveg>, Szöveg> {
            ha legyen Valami(Könyv) = veszélyes { SZÓTÁR.refként() } {
                Jó(Könyv.kap(&kulcs))
            } különben {
                Hibás("fetchez le Könyv".ebbe())
            }
        }
    }

    publikus(láda) függvény opció(i: u32) -> Opció<Eredmény<u32, Szöveg>> {
        ha i % 2 == 1 {
            ha i == 42 {
                Valami(Hibás(Szöveg::ből("basszameg")))
            } különben {
                Valami(Jó(33))
            }
        } különben {
            Semmi
        }
    }

    aszinkron függvény példa() {
    }

    aszinkron függvény példa2() {
        példa().megvár;
    }

    függvény fő() {
        legyen mutálható x = 31;

        pár x {
            42 => {
                sornyomtat!("pálinka")
            }
            _ => sornyomtat!("helo helo helo")
        }

        ciklus i ban 0..10 {
            legyen val = hurok {
                tör i;
            };

            amíg he x < val {
                x += 1;
            }

            x = ha legyen Valami(kimenet) = opció(i) {
                kimenet.kicsomagol()
            } különben {
                12
            };
        }

        //másodlagos();
    }

    #[engedélyez(kód_nem_elérhető)]
    függvény másodlagos() {
        kurva_anyád!("oh ne"); // for the true Hungarian experience
        a_gecibe!("megrohdasz");
        hoppá!("hiba történt xd"); // in SFW contexts
    }
}
