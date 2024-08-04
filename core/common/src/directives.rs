use highlighther::color::Color;

use crate::{ attributes::*, const_key };

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
    Source(Vec<Attributes>),
    Match(Vec<Attributes>),
    Filter(Vec<Attributes>),
    System(Vec<Records>),
    Label(Vec<Directives>),
    Worker(Vec<Directives>),
    Include(IncludeValue),
}

impl ToString for Directives {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl Color for Directives {
    fn color(&self) -> &'static str {
        highlighther::color::Red.color()
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

    pub fn to_str(&self) -> &str {
        match self {
            Directives::Source(_) => SOURCE,
            Directives::Match(_) => MATCH,
            Directives::Filter(_) => FILTER,
            Directives::System(_) => SYSTEM,
            Directives::Label(_) => LABEL,
            Directives::Worker(_) => WORKER,
            Directives::Include(_) => INCLUDE,
        }
    }

    pub fn is_directive(s: &str) -> bool {
        match s {
            SOURCE | MATCH | FILTER | SYSTEM | LABEL | WORKER | INCLUDE => true,
            _ => false,
        }
    }
}
