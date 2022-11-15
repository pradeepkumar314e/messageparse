
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, PartialEq, Default,Clone)]
pub struct Msg {
    pub segments:  Vec<Segment>,
    pub separator: char,
}


#[derive(Debug, PartialEq, Default, Deserialize, Serialize,Clone)]
pub struct Segment {
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Default,Deserialize, Serialize,Clone)]
pub struct Field {
    pub components: Vec<String>,
}