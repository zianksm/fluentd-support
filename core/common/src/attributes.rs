use crate::const_key;

const_key!(
    TYPE: "@type"
    PORT: "port"
    PATH: "path"
    RECORD: "record"
    LABEL: "@label"
    TAG: "tag"
    SAMPLE: "sample"
        INCLUDE: "include"

);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleTypeValue(Box<str>);

impl SingleTypeValue {
    pub fn new(value: &str) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0.into()
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn from_str(s: &str) -> Self {
        Self(s.into())
    }

    pub fn from_boxed_str(s: Box<str>) -> Self {
        Self(s)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeValue(Vec<SingleTypeValue>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Record {
    k: Box<str>,
    v: Option<Box<str>>,
}

impl Record {
    pub fn new(k: Box<str>, v: Option<Box<str>>) -> Self {
        Self { k, v }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Records(Vec<Record>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LabelValue(Box<str>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PathValue(Box<str>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TagValue(Box<str>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PortValue(u16);

#[derive(Debug, PartialEq, Eq, Clone)]
// TODO : change to serde value
pub struct SampleValue(Box<str>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IncludeValue(Box<str>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Attributes {
    Type(TypeValue),
    Port(PortValue),
    Path(PathValue),
    Record(Records),
    Label(LabelValue),
    Tag(TagValue),
    Sample(SampleValue),
    Include(IncludeValue),
}
