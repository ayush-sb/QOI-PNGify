use std::u8;

use nom::{
    bits::complete::tag,
    bits::complete::take,
    branch::alt,
    error::{make_error, ErrorKind},
    number::complete::be_u8,
    sequence::tuple,
    Err, IResult,
};

pub const QOI_OP_RGB_HEADER: u8 = 0b11111110;
pub const QOI_OP_RGBA_HEADER: u8 = 0b11111111;
pub const QOI_OP_INDEX_HEADER: usize = 0b00;
pub const QOI_OP_DIFF_HEADER: usize = 0b01;
pub const QOI_OP_LUMA_HEADER: usize = 0b10;
pub const QOI_OP_RUN_HEADER: usize = 0b11;

#[derive(Debug, Eq, PartialEq)]
pub struct RGBChunk {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RGBAChunk {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct IndexChunk {
    pub index: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct DiffChunk {
    pub dr: u8,
    pub dg: u8,
    pub db: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct LumaChunk {
    pub dg: u8,
    pub dr_dg: u8,
    pub db_dg: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RunChunk {
    pub run: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CHUNK {
    RGBChunk(RGBChunk),
    RGBAChunk(RGBAChunk),
    IndexChunk(IndexChunk),
    DiffChunk(DiffChunk),
    LumaChunk(LumaChunk),
    RunChunk(RunChunk),
}

pub fn parse_rgb(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let (input, tag) = be_u8(input)?;
    if tag != QOI_OP_RGB_HEADER {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let (input, (r, g, b)) = tuple((be_u8, be_u8, be_u8))(input)?;

    Ok((input, CHUNK::RGBChunk(RGBChunk { r, g, b })))
}

pub fn parse_rgba(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let (input, tag) = be_u8(input)?;
    if tag != QOI_OP_RGBA_HEADER {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let (input, (r, g, b, a)) = tuple((be_u8, be_u8, be_u8, be_u8))(input)?;

    Ok((input, CHUNK::RGBAChunk(RGBAChunk { r, g, b, a })))
}

pub fn parse_index(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let temp: IResult<(&[u8], usize), _> = tag(QOI_OP_INDEX_HEADER, 2usize)((input, 0));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, offset), _) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = take(6usize)((input, offset));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, _), index) = temp.unwrap();

    Ok((input, CHUNK::IndexChunk(IndexChunk { index })))
}

pub fn parse_diff(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let temp: IResult<(&[u8], usize), _> = tag(QOI_OP_DIFF_HEADER, 2usize)((input, 0));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, offset), _) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = take(2usize)((input, offset));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, offset), dr) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = take(2usize)((input, offset));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, offset), dg) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = take(2usize)((input, offset));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, _), db) = temp.unwrap();

    Ok((input, CHUNK::DiffChunk(DiffChunk { dr, dg, db })))
}

pub fn parse_luma(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let temp: IResult<(&[u8], usize), _> = tag(QOI_OP_LUMA_HEADER, 2usize)((input, 0));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, offset), _) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = take(6usize)((input, offset));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, offset), dg) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = take(4usize)((input, offset));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, offset), dr_dg) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = take(4usize)((input, offset));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, _), db_dg) = temp.unwrap();

    Ok((input, CHUNK::LumaChunk(LumaChunk { dg, dr_dg, db_dg })))
}

pub fn parse_run(input: &[u8]) -> IResult<&[u8], CHUNK> {
    let temp: IResult<(&[u8], usize), _> = tag(QOI_OP_RUN_HEADER, 2usize)((input, 0));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, offset), _) = temp.unwrap();

    let temp: IResult<(&[u8], usize), u8> = take(6usize)((input, offset));
    if temp.is_err() {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let ((input, _), run) = temp.unwrap();

    Ok((input, CHUNK::RunChunk(RunChunk { run })))
}

pub fn parse_chunks(input: &[u8]) -> IResult<&[u8], CHUNK> {
    alt((
        parse_rgb,
        parse_rgba,
        parse_index,
        parse_diff,
        parse_luma,
        parse_run,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb() {
        let bytes = [0xFE, 0x11, 0x05, 0x13];
        let chunk = CHUNK::RGBChunk(RGBChunk {
            r: 0x11,
            g: 0x05,
            b: 0x13,
        });

        let result = parse_rgb(&bytes).unwrap();
        assert_eq!(result.1, chunk);
    }

    #[test]
    fn test_rgba() {
        let bytes = [0xFF, 0xFF, 0x12, 0x07, 0x10];
        let chunk = CHUNK::RGBAChunk(RGBAChunk {
            r: 0xFF,
            g: 0x12,
            b: 0x07,
            a: 0x10,
        });

        let result = parse_rgba(&bytes).unwrap();
        assert_eq!(result.1, chunk);
    }

    #[test]
    fn test_index() {
        let bytes = [0b00101010];
        let chunk = CHUNK::IndexChunk(IndexChunk { index: 42 });

        let result = parse_index(&bytes).unwrap();
        assert_eq!(result.1, chunk);
    }

    #[test]
    fn test_diff() {
        let bytes = [0b01110100];
        let chunk = CHUNK::DiffChunk(DiffChunk {
            dr: 3,
            dg: 1,
            db: 0,
        });

        let result = parse_diff(&bytes).unwrap();
        assert_eq!(result.1, chunk);
    }

    #[test]
    fn test_luma() {
        let bytes = [0b10011010, 0b10010110];
        let chunk = CHUNK::LumaChunk(LumaChunk {
            dg: 0b011010,
            dr_dg: 0b1001,
            db_dg: 0b0110,
        });

        let result = parse_luma(&bytes).unwrap();
        assert_eq!(result.1, chunk);
    }

    #[test]
    fn test_run() {
        let bytes = [0b11011010];
        let chunk = CHUNK::RunChunk(RunChunk { run: 0b011010 });

        let result = parse_run(&bytes).unwrap();
        assert_eq!(result.1, chunk);
    }
}
