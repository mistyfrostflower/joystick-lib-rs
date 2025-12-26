
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use meby::Meby;
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::tungstenite::http;

pub type JSOptionalResult<T> = Meby<T, JoystickErr>;
pub type JSResult<T> = Result<T, JoystickErr>;

pub enum JoystickErr {
    NotConnected,
    AlreadyConnected,
    WebsocketError(tokio_tungstenite::tungstenite::Error),
    InvalidUri(http::uri::InvalidUri),
}

fn fmt_joystick_err(err: &JoystickErr, f: &mut Formatter<'_>) -> std::fmt::Result {
    match err {
        JoystickErr::NotConnected => f.write_str("Client is not connected to the joystick api."),
        JoystickErr::AlreadyConnected => {
            f.write_str("Client is already connected to the joystick api.")
        }
        JoystickErr::WebsocketError(ws_err) => Debug::fmt(&ws_err, f),
        JoystickErr::InvalidUri(err) => Debug::fmt(&err, f),
    }
}

impl Display for JoystickErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_joystick_err(self, f)
    }
}

impl Debug for JoystickErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_joystick_err(self, f)
    }
}

impl Error for JoystickErr {}

impl<R> From<JoystickErr> for Meby<R, JoystickErr> {
    fn from(value: JoystickErr) -> Self {
        Meby::Oops(value)
    }
}

impl<R> From<JoystickErr> for Result<R, JoystickErr> {
    fn from(value: JoystickErr) -> Self {
        Err(value)
    }
}

impl From<tungstenite::Error> for JoystickErr {
    fn from(value: tungstenite::Error) -> Self {
        JoystickErr::WebsocketError(value)
    }
}

impl From<http::uri::InvalidUri> for JoystickErr {
    fn from(value: http::uri::InvalidUri) -> Self {
        JoystickErr::InvalidUri(value)
    }
}
