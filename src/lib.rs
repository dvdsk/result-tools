pub trait ResultToolsA {
    type Error;
    fn ok_if<P: FnMut(&Self::Error) -> bool>(self, predicate: P) -> Self;
}

pub trait ResultToolsB {
    type Error;
    fn ok_if_is<U>(self, accaptable: U) -> Self
    where
        Self::Error: PartialEq<U>;
}

pub trait ResultToolsC {
    type Error;
    type Value;
    fn ok_none_if(
        self,
        f: impl FnOnce(&Self::Error) -> bool,
    ) -> Result<Option<Self::Value>, Self::Error>;
}

pub trait ResultToolsD {
    type Error;
    type Value;
    fn ok_none_if_is<U>(self, accaptable: U) -> Result<Option<Self::Value>, Self::Error>
    where
        Self::Error: PartialEq<U>;
}

impl<E> ResultToolsA for Result<(), E> {
    type Error = E;
    fn ok_if<P: FnMut(&Self::Error) -> bool>(self, mut predicate: P) -> Self {
        match self {
            Ok(_) => Ok(()),
            Err(e) if predicate(&e) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl<E> ResultToolsB for Result<(), E> {
    type Error = E;
    fn ok_if_is<U>(self, accaptable: U) -> Self
    where
        Self::Error: PartialEq<U>,
    {
        match self {
            Ok(_) => Ok(()),
            Err(e) if e == accaptable => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl<T, E> ResultToolsC for Result<T, E> {
    type Error = E;
    type Value = T;
    fn ok_none_if(
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

impl<T, E: PartialEq> ResultToolsD for Result<T, E> {
    type Error = E;
    type Value = T;
    fn ok_none_if_is<U>(self, accaptable: U) -> Result<Option<Self::Value>, Self::Error>
    where
        Self::Error: PartialEq<U>,
    {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(e) if e == accaptable => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, ErrorKind};

    #[derive(PartialEq, Debug)]
    enum Error {
        A,
        B,
    }

    mod accept_fn {
        use super::*;

        #[test]
        fn predicate_returns_true() {
            let result: Result<(), io::Error> = Err(io::Error::new(ErrorKind::AlreadyExists, ""));
            result
                .ok_if(|e| e.kind() == ErrorKind::AlreadyExists)
                .unwrap();
        }
        #[test]
        fn predicate_returns_false() {
            let result: Result<(), io::Error> = Err(io::Error::new(ErrorKind::NotFound, ""));
            result
                .ok_if(|e| e.kind() == ErrorKind::AlreadyExists)
                .unwrap_err();
        }
    }

    mod accept {
        use super::*;

        #[test]
        fn matching() {
            let result: Result<(), _> = Err(Error::A);
            result.ok_if_is(Error::A).unwrap();
        }

        #[test]
        fn not_matching() {
            let result: Result<(), _> = Err(Error::B);
            result.ok_if_is(Error::A).unwrap_err();
        }
    }

    mod accept_transform_fn {
        use super::*;

        #[test]
        fn predicate_returns_true() {
            let result: Result<&str, io::Error> = Err(io::Error::new(ErrorKind::AlreadyExists, ""));
            let result = result
                .ok_none_if(|e| e.kind() == ErrorKind::AlreadyExists)
                .unwrap();
            assert_eq!(result, None);
        }

        #[test]
        fn predicate_returns_false() {
            let result: Result<&str, io::Error> = Err(io::Error::new(ErrorKind::NotFound, ""));
            result
                .ok_none_if(|e| e.kind() == ErrorKind::AlreadyExists)
                .unwrap_err();
        }
    }

    mod accept_transform {
        use super::*;

        #[test]
        fn matching() {
            let result: Result<(), _> = Err(Error::A);
            let result = result.ok_none_if_is(Error::A).unwrap();
            assert_eq!(result, None);
        }

        #[test]
        fn not_matching() {
            let result: Result<(), _> = Err(Error::B);
            result.ok_none_if_is(Error::A).unwrap_err();
        }
    }
}
