use {
    failure::{Backtrace, Fail},
    std::fmt,
    webrtc::error::Error as RTCError,
    webrtc::util::Error as RTCUtilError,
};

#[derive(Debug)]
pub struct WebRTCError {
    pub value: WebRTCErrorValue,
}

#[derive(Debug, Fail)]
pub enum WebRTCErrorValue {
    #[fail(display = "webrtc error: {}", _0)]
    RTCError(#[cause] RTCError),
    #[fail(display = "webrtc util error: {}", _0)]
    RTCUtilError(#[cause] RTCUtilError),
    #[fail(display = "cannot get local description")]
    CanNotGetLocalDescription,
}

impl From<RTCError> for WebRTCError {
    fn from(error: RTCError) -> Self {
        WebRTCError {
            value: WebRTCErrorValue::RTCError(error),
        }
    }
}

impl From<RTCUtilError> for WebRTCError {
    fn from(error: RTCUtilError) -> Self {
        WebRTCError {
            value: WebRTCErrorValue::RTCUtilError(error),
        }
    }
}

impl fmt::Display for WebRTCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

impl Fail for WebRTCError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.value.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.value.backtrace()
    }
}
