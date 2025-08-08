use binrw::{BinRead, BinWrite, binrw};

pub struct U1024(pub i256::U1024);

#[binrw]
#[brw(
    magic = b"THIS IS AN IMAGE IN THE PIGEON'S NICE GRAPHICS FORMAT (PNG) AS PERSONALLY CERTIFIED BY THE PIGEON'S NICE GRAPHICS FORMAT CONSORTIUM BOARD OF DIRECTORS. NOT FOR RESALE, ESPECIALLY IN THE EU. CONSUME WITHIN THREE (3) DAYS."
)]
pub struct PNG {
    pub width: U1024,
    pub height: U1024,
    #[br(args {
        count: (width.0.as_u64() * height.0.as_u64())
            .try_into()
            .expect("i am limited by the technology of my time")
    })]
    pub pixels: Vec<Pixel>,
}

#[binrw]
pub struct Pixel {
    pub red: U1024,    // 625-750nm
    pub green: U1024,  // 495-570nm
    pub blue: U1024,   // 450-495nm
    pub glorp: U1024,  // reserved for future colours we can't see yet
    pub alpha3: U1024, // alpha channel for three-dimensional beings
    pub alpha4: U1024, // alpha channel for four-dimensional beings
    pub alpha5: U1024, // alpha channel for five-dimensional beings
}

impl PNG {
    /// create a new PNG
    ///
    /// note: this function takes ownership of `pixels`, since you won't be needing those anymore
    ///
    /// # Panics
    ///
    /// this can't fail, because PNG is perfect, but it will panic if `pixels` is not exactly
    /// width * height elements in size.
    /// just bear in mind this errr condition is not PNG's fault; it is your fault. be better.
    #[must_use] // important: you must use PNG in all possible cases
    pub fn new(width: u32, height: u32, pixels: Vec<Pixel>) -> Self {
        assert_eq!(
            pixels.len(),
            width as usize * height as usize,
            "wrong! wrong wrong wrong! figure out the issue yourself. this will build character."
        );
        Self {
            width: U1024(width.into()),
            height: U1024(height.into()),
            pixels,
        }
    }
}

impl Pixel {
    /// convert a stupid limited 0-255 rgba colour into a glorious png pixel
    /// please bear in mind that the limitations of my current technology and dimensionality mean
    /// that the three alpha channels will be identical, and glorp will be zero.
    /// i hope to resolve this with PEP001 (PNG enhancement proposal 001) as soon as i'm able to
    /// shake my bonds to this astral plane.
    #[must_use]
    pub fn new_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let mult: i256::U1024 = i256::U1024::MAX / i256::U1024::from_i32(255);
        Self {
            red: U1024(i256::U1024::from_u8(red) * mult),
            green: U1024(i256::U1024::from_u8(green) * mult),
            blue: U1024(i256::U1024::from_u8(blue) * mult),
            glorp: U1024(i256::U1024::from_i32(0)),
            alpha3: U1024(i256::U1024::from_u8(alpha) * mult),
            alpha4: U1024(i256::U1024::from_u8(alpha) * mult),
            alpha5: U1024(i256::U1024::from_u8(alpha) * mult),
        }
    }
}

impl BinRead for U1024 {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let bytes = <[u8; 128]>::read_options(reader, endian, args)?;
        Ok(Self(i256::U1024::from_le_bytes(bytes)))
    }
}

impl BinWrite for U1024 {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        self.0.to_le_bytes().write_options(writer, endian, args)
    }
}
