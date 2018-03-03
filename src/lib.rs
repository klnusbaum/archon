pub mod messaging;
pub mod algorithm;
pub mod types;
pub mod persistence;

pub mod client {
    use types::RaftError;
    pub trait ArchonClient {
        fn get_value(&self, key: String) -> Result<Option<Vec<u8>>, RaftError>;
        fn put_value(&self, key: String, value: Vec<u8>) -> Result<(), RaftError>;
    }

    pub struct Client {}

    impl Client {
        pub fn new() -> Client {
            Client {}
        }
    }

    impl ArchonClient for Client {
        fn get_value(&self, key: String) -> Result<Option<Vec<u8>>, RaftError> {
            Ok(None)
        }

        fn put_value(&self, key: String, value: Vec<u8>) -> Result<(), RaftError> {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
