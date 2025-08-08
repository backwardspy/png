use std::{fs::File, io::BufReader, path::PathBuf, time::Instant};

use binrw::BinReaderExt as _;
use clap::{Parser as _, ValueEnum as _};

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg(short, long)]
    format: Format,
    path: PathBuf,
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
enum Format {
    Avif,
    Bmp,
    Cr2,
    Dng,
    Eps,
    Exr,
    Gif,
    Heic,
    Jpeg,
    Jpg,
    Jxl,
    Kra,
    Pdf,
    Png,
    Psd,
    Raw,
    Svg,
    Tif,
    Tiff,
    Webp,
    Xcf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let file = File::open(&args.path)?;
    let mut reader = BufReader::new(file);

    let start = Instant::now();
    let png: png::PNG = reader.read_le()?;

    println!(
        "read {}x{} PNG in a blazing {} seconds",
        png.width.0,
        png.height.0,
        start.elapsed().as_secs()
    );
    eprintln!(
        "error: selected format \"{}\" is too simple to adequately contain a PNG. please try a better format.",
        args.format.to_possible_value().unwrap().get_name()
    );
    Ok(())
}
