pub trait ResultToolsA {
    type Error;
    fn accept_fn<P: FnMut(&Self::Error) -> bool>(self, predicate: P) -> Self;
}

pub trait ResultToolsB {
    type Error;
    type Value;
    fn accept_err_with(
        self,
        f: impl FnOnce(&Self::Error) -> bool,
    ) -> Result<Option<Self::Value>, Self::Error>;
}

impl<E> ResultToolsA for Result<(), E> {
    type Error = E;
    fn accept_fn<P: FnMut(&Self::Error) -> bool>(self, mut predicate: P) -> Self {
        match self {
            Ok(_) => Ok(()),
            Err(e) if predicate(&e) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl<T, E> ResultToolsB for Result<T, E> {
    type Error = E;
    type Value = T;
    fn accept_err_with(
        self,
        f: impl FnOnce(&Self::Error) -> bool,
    ) -> Result<Option<Self::Value>, Self::Error> {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(e) if f(&e) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
