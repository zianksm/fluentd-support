use crate::color::Color;

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

pub trait Keyword: ToString + Color {}

impl<T> Keyword for T where T: ToString + Color {}

pub mod directives {
    use crate::{ color::Color, const_key };

    const_key!(
    SOURCE: "source"
    MATCH: "match"
    FILTER: "filter"
    SYSTEM: "system"
    LABEL: "label"
    WORKER: "worker"
    INCLUDE: "include"
    );

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Directives {
        Source,
        Match,
        Filter,
        System,
        Label,
        Worker,
        Include,
    }

    impl ToString for Directives {
        fn to_string(&self) -> String {
            self.to_str().to_string()
        }
    }

    impl Color for Directives {
        fn color(&self) -> &'static str {
            crate::color::Red.color()
        }
    }

    impl Directives {
        const_key!(
        SOURCE
        MATCH
        FILTER
        SYSTEM
        LABEL
        WORKER
        INCLUDE
    );

        pub fn from_str(s: &str) -> Option<Self> {
            match s.to_lowercase().as_str() {
                SOURCE => Some(Directives::Source),
                MATCH => Some(Directives::Match),
                FILTER => Some(Directives::Filter),
                SYSTEM => Some(Directives::System),
                LABEL => Some(Directives::Label),
                WORKER => Some(Directives::Worker),
                INCLUDE => Some(Directives::Include),
                _ => None,
            }
        }

        pub fn to_str(&self) -> &str {
            match self {
                Directives::Source => SOURCE,
                Directives::Match => MATCH,
                Directives::Filter => FILTER,
                Directives::System => SYSTEM,
                Directives::Label => LABEL,
                Directives::Worker => WORKER,
                Directives::Include => INCLUDE,
            }
        }

        pub fn is_directive(s: &str) -> bool {
            match s {
                SOURCE | MATCH | FILTER | SYSTEM | LABEL | WORKER | INCLUDE => true,
                _ => false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use directives::Directives;

    use super::*;
    #[test]
    fn test_directives() {
        let source = Directives::Source;
        assert_eq!(source.to_string(), "source");
        assert_eq!(source.color(), "red");
        assert_eq!(Directives::from_str("source"), Some(Directives::Source));
        assert_eq!(Directives::from_str("match"), Some(Directives::Match));
        assert_eq!(Directives::from_str("filter"), Some(Directives::Filter));
        assert_eq!(Directives::from_str("system"), Some(Directives::System));
        assert_eq!(Directives::from_str("label"), Some(Directives::Label));
        assert_eq!(Directives::from_str("worker"), Some(Directives::Worker));
        assert_eq!(Directives::from_str("include"), Some(Directives::Include));
        assert_eq!(Directives::from_str("invalid"), None);
    }
}
