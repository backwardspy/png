use std::{fs::File, io::BufReader, path::PathBuf, sync::Arc, time::Instant};

use binrw::BinReaderExt as _;
use clap::Parser as _;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Debug, clap::Parser)]
struct Args {
    path: PathBuf,
}

struct App {
    wnd: Option<Arc<Window>>,
    pix: Option<Pixels<'static>>,
    png: png::PNG,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let file = File::open(&args.path)?;
    let mut reader = BufReader::new(file);

    println!("decoding {}...", args.path.to_string_lossy());
    let start = Instant::now();
    let png: png::PNG = reader.read_le()?;
    println!(
        "decoded {}x{} PNG in a blazing {} seconds",
        png.width.0,
        png.height.0,
        start.elapsed().as_secs()
    );

    let el = EventLoop::new()?;
    el.set_control_flow(ControlFlow::Poll);
    let mut app = App {
        wnd: None,
        pix: None,
        png,
    };
    el.run_app(&mut app)?;

    Ok(())
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        let w = self.png.width.0.as_u32();
        let h = self.png.height.0.as_u32();
        let size = LogicalSize::new(f64::from(w), f64::from(h));
        let wnd = Arc::new(
            el.create_window(
                WindowAttributes::default()
                    .with_title("viewpng")
                    .with_inner_size(size)
                    .with_min_inner_size(size),
            )
            .unwrap(),
        );
        self.wnd = Some(wnd.clone());
        self.pix = {
            let surf = SurfaceTexture::new(w, h, wnd.clone());
            match Pixels::new(w, h, surf) {
                Ok(pixels) => {
                    wnd.request_redraw();
                    Some(pixels)
                }
                Err(err) => {
                    eprintln!("failed to create rendering context: {err}");
                    el.exit();
                    None
                }
            }
        };
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _: WindowId, ev: WindowEvent) {
        match ev {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::RedrawRequested => {
                let frame = self.pix.as_mut().unwrap().frame_mut();
                let mult: i256::U1024 = i256::U1024::MAX / i256::U1024::from_i32(255);
                for (i, pix) in frame.chunks_exact_mut(4).enumerate() {
                    let glorious = &self.png.pixels[i];
                    let rgba = [
                        (glorious.red.0 / mult).as_u8(),
                        (glorious.green.0 / mult).as_u8(),
                        (glorious.blue.0 / mult).as_u8(),
                        // note: this viewer is intended for use by three-dimensional beings
                        (glorious.alpha3.0 / mult).as_u8(),
                    ];
                    pix.copy_from_slice(&rgba);
                }
                self.pix.as_ref().unwrap().render().unwrap();
            }
            _ => {}
        }
    }
}
