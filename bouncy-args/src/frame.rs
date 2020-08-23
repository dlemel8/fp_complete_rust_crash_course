use std::env::Args;

#[derive(Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum FrameParseError {
    TooFewArguments,
    TooManyArguments,
    InvalidValue(String),
    WidthTooSmall(u32),
    HeightTooSmall(u32),
}

fn require_arg(args: &mut Args) -> Result<String, FrameParseError> {
    match args.next() {
        None => Err(FrameParseError::TooFewArguments),
        Some(x) => Ok(x),
    }
}

fn require_no_args(args: &mut Args) -> Result<(), FrameParseError> {
    match args.next() {
        Some(_) => Err(FrameParseError::TooManyArguments),
        None => Ok(()),
    }
}

fn parse_u32(s: String) -> Result<u32, FrameParseError> {
    match s.parse::<u32>() {
        Err(_) => Err(FrameParseError::InvalidValue(s)),
        Ok(x) => Ok(x),
    }
}

pub fn parse_args() -> Result<Frame, FrameParseError> {
    let mut args = std::env::args();
    require_arg(&mut args)?;  // skip the command name

    let width_str = require_arg(&mut args)?;
    let width = parse_u32(width_str)?;
    if width < 10 {
        return Err(FrameParseError::WidthTooSmall(width));
    }

    let height_str = require_arg(&mut args)?;
    let height = parse_u32(height_str)?;
    if height < 10 {
        return Err(FrameParseError::HeightTooSmall(height));
    }

    require_no_args(&mut args)?;
    Ok(Frame { width, height })
}
