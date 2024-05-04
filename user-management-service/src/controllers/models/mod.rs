use serde::{Deserialize, Serialize};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone,Default)]
pub struct ResponseModel<'a, T>
where T: serde::Serialize
{
    pub status: ResponseStatus,
    pub message: &'a str,
    pub data: T,
}

#[derive(Serialize,Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ResponseStatus {
    #[default] Ok=200,
    BadRequest=400,
    NotFound=404,
    Unauthorized=401,
    InternalServerError = 500,
    
}