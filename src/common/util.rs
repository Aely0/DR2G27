use hidapi::HidError;
use std::io::Error;

// [GREEN_1, GREEN_2, ORANGE_4, ORANGE_8, RED_16]

pub const G27_VID: u16 = 1133; // Vendor ID
pub const G27_PID: u16 = 49819; // Product ID

pub type DR2G27Result = Result<(), DR2G27Error>;

#[derive(Debug)]
pub enum DR2G27Error {
    DR2UdpSocketError,
    G27ConnectionLostError,
}

impl From<Error> for DR2G27Error {
    fn from(_: Error) -> Self {
        Self::DR2UdpSocketError
    }
}

impl From<HidError> for DR2G27Error {
    fn from(_: HidError) -> Self {
        Self::G27ConnectionLostError
    }
}
