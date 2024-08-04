pub trait Color {
    fn color(&self) -> &'static str;
}

#[macro_export]
macro_rules! color {
    ($($color:ident),*) => {
        $(
            paste::paste!{
                pub struct $color;
                impl Color for $color {
                    fn color(&self) -> &'static str {
                        stringify!([<$color:lower>])
                    }
                }
            }
        )*
    };
}

color!(Red, Blue);
