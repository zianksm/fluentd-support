mod directives;
mod attributes;

#[macro_export(local_inner_macros)]
macro_rules! const_key {
    ($($ident:ident: $literal:literal)*) => {
        $(
            pub const $ident: &str = $literal;

        )*
    };

    ($($ident:ident)*) => {
        $(
            paste::paste!{
                fn [<is_ $ident:lower>](s:&str) -> bool{
                    s == $ident
                }
            }
        )*
    };
}
