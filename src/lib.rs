#[macro_use]
extern crate typed_builder;
extern crate chrono;
extern crate image;
extern crate rayon;
use rayon::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

pub extern crate nalgebra as na;
pub use log::{debug, error, info, trace, warn};

pub type Result<T> = std::result::Result<T, String>;

pub type Point = na::Point3<f64>;
pub type Vector = na::Vector3<f64>;

pub mod identities;
pub mod object;
pub mod transform;
pub use crate::object::Sdf;
pub use crate::transform::*;
pub use identities::*;

fn setup_logger() -> std::result::Result<(), fern::InitError> {
    let colors = fern::colors::ColoredLevelConfig::new()
        .error(fern::colors::Color::Red)
        .warn(fern::colors::Color::Yellow)
        .info(fern::colors::Color::White)
        .debug(fern::colors::Color::Blue)
        .trace(fern::colors::Color::Magenta);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("pathmarch.log")?)
        .apply()?;
    Ok(())
}

fn map<Input, Output>(
    val: Input,
    input_range: (Input, Input),
    output_range: (Output, Output),
) -> Output
where
    Input: Copy,
    Output: Copy
        + From<Input>
        + Sub<Output = Output>
        + Div<Output = Output>
        + Mul<Output = Output>
        + Add<Output = Output>,
{
    return (Output::from(val) - Output::from(input_range.0))
        / (Output::from(input_range.1) - Output::from(input_range.0))
        * (output_range.1 - output_range.0)
        + output_range.0;
}

#[derive(TypedBuilder)]
pub struct RenderArgs {
    #[builder(default=(1000, 1000))]
    image_dimensions: (u32, u32),
    #[builder(default = String::from("out.png"))]
    image_output: String,
    #[builder(default=std::f64::consts::PI / 2.0f64)]
    fov: f64,
    #[builder(default = 128)]
    spp: u32,
    #[builder(default)]
    objects: Vec<Box<dyn Sdf + Sync>>,
}

fn render_pixel(xy: (f64, f64)) -> Vector {
    return Vector::new(0.0, 0.0, 0.0);
}

pub fn render(args: RenderArgs) -> Result<()> {
    let dims = (args.image_dimensions.0, args.image_dimensions.1);
    match setup_logger() {
        Err(e) => Err(format!("{}", e)),
        _ => Ok(()),
    }?;
    let mut imgbuf = image::RgbImage::new(args.image_dimensions.0, args.image_dimensions.1);
    let mut buffer: Vec<((u32, u32), Vector)> = (0..args.image_dimensions.0)
        .map(|x| {
            return (0..args.image_dimensions.1).map(move |y| {
                return ((x, y), Vector::new(0.0, 0.0, 0.0));
            });
        })
        .flatten()
        .collect();
    buffer.par_iter_mut().for_each(|((x, y), pixel)| {
        for _sample in 0..args.spp {
            *pixel = *pixel + render_pixel((*x as f64, *y as f64)) / (args.spp as f64);
        }
        pixel[0] = map(*x, (0u32, args.image_dimensions.0), (0.0, 1.0));
        pixel[1] = map(*y, (0u32, args.image_dimensions.0), (0.0, 1.0));
    });
    buffer.iter().for_each(|((x, y), pixel)| {
        imgbuf.put_pixel(
            *x,
            *y,
            image::Rgb([
                (pixel[0] * 255.0) as u8,
                (pixel[1] * 255.0) as u8,
                (pixel[2] * 255.0) as u8,
            ]),
        );
    });
    match imgbuf.save(args.image_output) {
        Err(e) => error!("{:?}", e),
        _ => (),
    };
    Ok(())
}
