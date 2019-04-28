#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_parens)]
// compile-flags: -Z borrowck=compare

#![recursion_limit = "256"]

use std::cell::Cell;
use std::mem::swap;

// Just a grab bag of stuff that you wouldn't want to actually write.

fn strange() -> bool { let _x: bool = return true; }

fn funny() {
    fn f(_x: ()) { }
    f(return);
}

fn what() {
    fn the(x: &Cell<bool>) {
        return while !x.get() { x.set(true); };
    }
    let i = &Cell::new(false);
    let dont = {||the(i)};
    dont();
    assert!((i.get()));
}

fn zombiejesus() {
    loop {
        while (return) {
            if (return) {
                match (return) {
                    1 => {
                        if (return) {
                            return
                        } else {
                            return
                        }
                    }
                    _ => { return }
                };
            } else if (return) {
                return;
            }
        }
        if (return) { break; }
    }
}

fn notsure() {
    let mut _x: isize;
    let mut _y = (_x = 0) == (_x = 0);
    let mut _z = (_x = 0) < (_x = 0);
    let _a = (_x += 0) == (_x = 0);
    let _b = swap(&mut _y, &mut _z) == swap(&mut _y, &mut _z);
}

fn canttouchthis() -> usize {
    fn p() -> bool { true }
    let _a = (assert!((true)) == (assert!(p())));
    let _c = (assert!((p())) == ());
    let _b: bool = (println!("{}", 0) == (return 0));
}

fn angrydome() {
    loop { if break { } }
    let mut i = 0;
    loop { i += 1; if i == 1 { match (continue) { 1 => { }, _ => panic!("wat") } }
      break; }
}

fn evil_lincoln() { let _evil = println!("lincoln"); }

fn dots() {
    assert_eq!(String::from(".................................................."),
               format!("{:?}", .. .. .. .. .. .. .. .. .. .. .. .. ..
                               .. .. .. .. .. .. .. .. .. .. .. ..));
}

fn u8(u8: u8) {
    if u8 != 0u8 {
        assert_eq!(8u8, {
            macro_rules! u8 {
                (u8) => {
                    mod u8 {
                        pub fn u8<'u8: 'u8 + 'u8>(u8: &'u8 u8) -> &'u8 u8 {
                            "u8";
                            u8
                        }
                    }
                };
            }

            u8!(u8);
            let &u8: &u8 = u8::u8(&8u8);
            ::u8(0u8);
            u8
        });
    }
}

fn fishy() {
    assert_eq!(String::from("><>"),
               String::<>::from::<>("><>").chars::<>().rev::<>().collect::<String>());
}

fn union() {
    union union<'union> { union: &'union union<'union>, }
}

fn special_characters() {
    let val = !((|(..):(_,_),__@_|__)((&*"\\",'🤔')/**/,{})=={&[..=..][..];})//
    ;
    assert!(!val);
}

fn punch_card() -> impl std::fmt::Debug {
    ..=..=.. ..    .. .. .. ..    .. .. .. ..    .. ..=.. ..
    ..=.. ..=..    .. .. .. ..    .. .. .. ..    ..=..=..=..
    ..=.. ..=..    ..=.. ..=..    .. ..=..=..    .. ..=.. ..
    ..=..=.. ..    ..=.. ..=..    ..=.. .. ..    .. ..=.. ..
    ..=.. ..=..    ..=.. ..=..    .. ..=.. ..    .. ..=.. ..
    ..=.. ..=..    ..=.. ..=..    .. .. ..=..    .. ..=.. ..
    ..=.. ..=..    .. ..=..=..    ..=..=.. ..    .. ..=.. ..
}

fn r#match() {
    let val = match match match match match () {
        () => ()
    } {
        () => ()
    } {
        () => ()
    } {
        () => ()
    } {
        () => ()
    };
    assert_eq!(val, ());
}

pub fn main() {
    strange();
    funny();
    what();
    zombiejesus();
    notsure();
    canttouchthis();
    angrydome();
    evil_lincoln();
    dots();
    u8(8u8);
    fishy();
    union();
    special_characters();
    punch_card();
    r#match();
}