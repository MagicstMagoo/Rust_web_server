#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninhabited,
}

impl From<&str> for Method {
    fn from(service: &str) -> Method{
        match service {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninhabited,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    Uninhabited,
}

impl From<&str> for Version{
    fn from(service: &str) -> Version{
        match service {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninhabited,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_method_into(){
        let message: Method = "GET".into();
        assert_eq!(message, Method::Get);
    }

    #[test]
    fn test_version_into(){
        let version: Version = "HTTP/1.1".into();
        assert_eq!(version, Version::V1_1);
    }
}