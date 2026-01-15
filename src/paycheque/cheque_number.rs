use facet::Facet;
use std::ops::Deref;

#[derive(Facet, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
impl core::fmt::Display for ChequeNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // https://mina86.com/2024/fmt-display-impl/
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::paycheque::ChequeNumber;

    #[test]
    pub fn it_works() -> eyre::Result<()> {
        assert_eq!(ChequeNumber("123".to_string()).to_string(), "123");
        Ok(())
    }
}
