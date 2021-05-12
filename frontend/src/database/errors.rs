use serde_json::Error as SerdeError;
use std::fmt::{Debug, Display, Formatter};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum FrontendError {
    Pouch(&'static str),
    Js(JsValue),
    Serde(SerdeError),
}

impl From<JsValue> for FrontendError {
    fn from(v: JsValue) -> FrontendError {
        FrontendError::Js(v)
    }
}

impl From<SerdeError> for FrontendError {
    fn from(err: SerdeError) -> FrontendError {
        FrontendError::Serde(err)
    }
}

impl std::error::Error for FrontendError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pouch(_) => None,
            Self::Js(_) => None,
            Self::Serde(err) => err.source(),
        }
    }
}

impl Display for FrontendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pouch(err) => Display::fmt(err, f),
            Self::Js(err) => err.fmt(f),
            Self::Serde(err) => <SerdeError as Display>::fmt(err, f),
        }
    }
}
