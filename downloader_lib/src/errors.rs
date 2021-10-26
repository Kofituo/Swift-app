use rifgen::rifgen_attr::generate_interface;
#[generate_interface]
pub enum ResponseErrors {
    ErrorParsingRequest,
    UnableToDecodeRequest,
    RedirectedManyTimes,
    ConnectionTimeout,
    UnknownError,
}
