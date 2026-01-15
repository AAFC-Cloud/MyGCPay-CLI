use facet::Facet;
use std::ops::Deref;

#[derive(Facet)]
pub struct ChequeNumber(pub String);
impl Deref for ChequeNumber {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<str> for ChequeNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
