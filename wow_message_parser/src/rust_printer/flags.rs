use crate::parser::types::definer::Definer;
use crate::rust_printer::enums::print_wowm_definition;
use crate::rust_printer::{
    print_docc_description_and_comment, print_member_docc_description_and_comment,
    print_serde_derive, Writer,
};
use crate::Objects;

pub(crate) fn print_flag(e: &Definer, o: &Objects) -> Writer {
    let mut s = Writer::new();

    declaration(&mut s, e, o);

    common_impls(&mut s, e, o);

    s
}

fn declaration(s: &mut Writer, e: &Definer, o: &Objects) {
    print_docc_description_and_comment(s, e.tags(), o, e.tags());

    print_wowm_definition("flag", s, e);

    s.wln("#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]");
    print_serde_derive(s, e.tags().is_in_base());
    s.new_flag(e.name(), e.ty().rust_str(), |_| {});
}

fn common_impls(s: &mut Writer, e: &Definer, o: &Objects) {
    s.bodyn(format!("impl {name}", name = e.name()), |s| {
        s.funcn_pub_const(
            format!("new(inner: {ty})", ty = e.ty().rust_str()),
            "Self",
            |s| {
                s.wln("Self { inner }");
            },
        );

        print_fields(s, e, o);

        let func_type = if e.tags().is_in_base() {
            Writer::funcn_pub_const
        } else {
            Writer::funcn_const
        };

        func_type(s, "as_int(&self)", e.ty().rust_str(), |s: &mut Writer| {
            s.wln("self.inner");
        });
    });

    for fmt in [
        "std::fmt::UpperHex",
        "std::fmt::LowerHex",
        "std::fmt::Octal",
        "std::fmt::Binary",
    ] {
        s.bodyn(format!("impl {fmt} for {ty}", ty = e.name()), |s| {
            s.body(
                "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result",
                |s| {
                    s.wln(format!("{fmt}::fmt(&self.inner, f)"));
                },
            );
        });
    }

    for (tr, operator) in [
        ("BitAnd", "bitand"),
        ("BitOr", "bitor"),
        ("BitXor", "bitxor"),
    ] {
        s.bodyn(
            format!("impl std::ops::{tr} for {ty}", ty = e.name()),
            |s| {
                s.wln("type Output = Self;");
                s.body(
                    format!("fn {operator}(self, rhs: Self) -> Self::Output"),
                    |s| {
                        s.wln(format!(
                            "Self {{ inner: self.inner.{operator}(rhs.inner), }}"
                        ));
                    },
                );
            },
        );

        s.bodyn(
            format!("impl std::ops::{tr}Assign for {ty}", ty = e.name()),
            |s| {
                s.body(format!("fn {operator}_assign(&mut self, rhs: Self)"), |s| {
                    s.wln(format!("self.inner.{operator}_assign(rhs.inner)"));
                });
            },
        );
    }
}

fn print_fields(s: &mut Writer, e: &Definer, o: &Objects) {
    for f in e.fields() {
        s.wln(format!(
            "pub const {name}: {ty} = {value:#04x};",
            name = f.name(),
            ty = e.ty().rust_str(),
            value = f.value().int(),
        ));
    }
    s.newline();

    s.funcn_pub_const("empty()", "Self", |s| {
        s.wln("Self { inner: 0 }");
    });

    s.funcn_pub_const("is_empty(&self)", "bool", |s| {
        s.wln("self.inner == 0");
    });

    s.funcn_pub_const("all()", "Self", |s| {
        s.body("Self", |s| {
            s.wln(format!("inner: Self::{name}", name = e.fields()[0].name()));
            s.inc_indent();
            for (i, f) in e.fields().iter().enumerate() {
                if i == 0 {
                    continue;
                }
                s.wln(format!("| Self::{name}", name = f.name()));
            }
            s.dec_indent();
        });
    });

    for f in e.fields() {
        if f.value().int() != 0 {
            s.funcn_pub_const(format!("is_{name}(&self)", name = f.name()), "bool", |s| {
                if e.tags().zero_is_always_valid() {
                    s.wln(format!(
                        "((self.inner & Self::{name}) != 0) || self.inner == 0",
                        name = f.name()
                    ));
                } else {
                    s.wln(format!("(self.inner & Self::{name}) != 0", name = f.name()));
                }
            });

            print_member_docc_description_and_comment(s, f.tags(), o, e.tags());
            s.funcn_pub_const(format!("new_{name}()", name = f.name()), "Self", |s| {
                s.wln(format!("Self {{ inner: Self::{name} }}", name = f.name()));
            });

            s.funcn_pub(
                format!("set_{name}(&mut self)", name = f.name()),
                "Self",
                |s| {
                    s.wln(format!("self.inner |= Self::{name};", name = f.name()));
                    s.wln("*self");
                },
            );

            s.funcn_pub(
                format!("clear_{name}(&mut self)", name = f.name()),
                "Self",
                |s| {
                    s.wln(format!(
                        "self.inner &= Self::{name}.reverse_bits();",
                        name = f.name()
                    ));
                    s.wln("*self");
                },
            );
        }
    }
}
