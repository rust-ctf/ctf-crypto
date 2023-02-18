#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Message {
    pub(crate) data: Vec<u8>,
}

pub enum MessageError {
    DecodeError(String),
}

impl Message {
    pub fn new(data: Vec<u8>) -> Message {
        Message { data }
    }

    pub fn from<T>(data: &T) -> Message
    where
        T: AsRef<[u8]> + ?Sized,
    {
        Message {
            data: data.as_ref().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::message::Message;

    #[test]
    fn test_new() {
        assert_eq!(Message::new(vec![]).data, Vec::<u8>::new());
        assert_eq!(Message::new(vec![1]).data, vec![1]);
        assert_eq!(Message::new(vec![1, 2, 3]).data, vec![1, 2, 3]);
    }

    #[test]
    fn test_from_slice() {
        assert_eq!(Message::from(&[]), Message::new(vec![]));
        assert_eq!(Message::from(&[1]), Message::new(vec![1]));
        assert_eq!(Message::from(&[1, 2, 3]), Message::new(vec![1, 2, 3]));
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Message::from(""), Message::new(vec![]));
        assert_eq!(Message::from("\x5F"), Message::new(vec![0x5F]));
        assert_eq!(Message::from("aB9"), Message::new(vec![97, 66, 57]));
        assert_eq!(Message::from("\u{1234}"), Message::new(vec![225, 136, 180]));
    }
}
