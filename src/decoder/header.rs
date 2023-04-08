use nom::{
    bytes::complete::tag, number::complete::be_u32, number::complete::be_u8, sequence::tuple,
    IResult,
};

pub struct QOIHeader {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub colorspace: u8,
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], QOIHeader> {
    let (input, _) = tag("qoif")(input)?;
    let (input, (width, height, channels, colorspace)) =
        tuple((be_u32, be_u32, be_u8, be_u8))(input)?;
    let header = QOIHeader {
        width,
        height,
        channels,
        colorspace,
    };

    Ok((input, header))
}
