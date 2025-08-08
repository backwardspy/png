use std::{fs::File, io::BufWriter, path::PathBuf};

use binrw::BinWriterExt as _;
use clap::Parser as _;
use image::ImageReader;

#[derive(Debug, clap::Parser)]
struct Args {
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let img = ImageReader::open(&args.path)?.decode()?;

    let png = png::PNG::new(
        img.width(),
        img.height(),
        img.into_rgba8()
            .pixels()
            .map(|pix| png::Pixel::new_rgba(pix.0[0], pix.0[1], pix.0[2], pix.0[3]))
            .collect(),
    );

    let outpath = args.path.with_extension("png_real");
    let outfile = File::create(&outpath)?;
    let mut writer = BufWriter::new(outfile);
    writer.write_le(&png)?;

    println!(
        "wrote {}x{} PNG into {}",
        png.width.0,
        png.height.0,
        outpath.to_string_lossy(),
    );

    Ok(())
}
