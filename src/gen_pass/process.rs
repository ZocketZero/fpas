use clap::ValueEnum;

use super::{byte_mode, chain_mode, normal_mode};
#[derive(Clone, Debug, Default, ValueEnum, PartialEq)]
pub enum Mode {
    #[default]
    N,
    Normal,
    B,
    Byte,
}

pub fn process(msg: String, mode: Mode, looping: u32, chain: bool) -> String {
    let mut msg = msg;
    let callback = match mode {
        Mode::N | Mode::Normal => {
            msg = normal_mode(msg);
            if chain {
                |msg| chain_mode(normal_mode, msg)
            } else {
                |msg| normal_mode(msg)
            }
        }
        Mode::B | Mode::Byte => {
            msg = byte_mode(msg);
            if chain {
                |msg| chain_mode(byte_mode, msg)
            } else {
                |msg| byte_mode(msg)
            }
        }
    };
    for _ in 1..looping {
        msg = callback(msg);
    }
    msg
}
