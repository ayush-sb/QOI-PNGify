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
