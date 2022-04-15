pub trait ResultToolsA {
    type Error;
    fn accept_fn<P: FnMut(&Self::Error) -> bool>(self, predicate: P) -> Self;
}

pub trait ResultToolsB {
    type Error;
    fn accept(self, accaptable: Self::Error) -> Self;
}

pub trait ResultToolsC {
    type Error;
    type Value;
    fn accept_transform_fn(
        self,
        f: impl FnOnce(&Self::Error) -> bool,
    ) -> Result<Option<Self::Value>, Self::Error>;
}

pub trait ResultToolsD {
    type Error;
    type Value;
    fn accept_transform(self, accaptable: Self::Error) -> Result<Option<Self::Value>, Self::Error>;
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

impl<E: PartialEq> ResultToolsB for Result<(), E> {
    type Error = E;
    fn accept(self, accaptable: E) -> Self {
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
    fn accept_transform_fn(
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
    fn accept_transform(self, accaptable: Self::Error) -> Result<Option<Self::Value>, Self::Error> {
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
                .accept_fn(|e| e.kind() == ErrorKind::AlreadyExists)
                .unwrap();
        }
        #[test]
        fn predicate_returns_false() {
            let result: Result<(), io::Error> = Err(io::Error::new(ErrorKind::NotFound, ""));
            result
                .accept_fn(|e| e.kind() == ErrorKind::AlreadyExists)
                .unwrap_err();
        }
    }

    mod accept {
        use super::*;

        #[test]
        fn matching() {
            let result: Result<(), _> = Err(Error::A);
            result.accept(Error::A).unwrap();
        }

        #[test]
        fn not_matching() {
            let result: Result<(), _> = Err(Error::B);
            result.accept(Error::A).unwrap_err();
        }
    }

    mod accept_transform_fn {
        use super::*;

        #[test]
        fn predicate_returns_true() {
            let result: Result<&str, io::Error> = Err(io::Error::new(ErrorKind::AlreadyExists, ""));
            let result = result
                .accept_transform_fn(|e| e.kind() == ErrorKind::AlreadyExists)
                .unwrap();
            assert_eq!(result, None);
        }

        #[test]
        fn predicate_returns_false() {
            let result: Result<&str, io::Error> = Err(io::Error::new(ErrorKind::NotFound, ""));
            result
                .accept_transform_fn(|e| e.kind() == ErrorKind::AlreadyExists)
                .unwrap_err();
        }
    }

    mod accept_transform {
        use super::*;

        #[test]
        fn matching() {
            let result: Result<(), _> = Err(Error::A);
            let result = result.accept_transform(Error::A).unwrap();
            assert_eq!(result, None);
        }

        #[test]
        fn not_matching() {
            let result: Result<(), _> = Err(Error::B);
            result.accept_transform(Error::A).unwrap_err();
        }
    }
}
