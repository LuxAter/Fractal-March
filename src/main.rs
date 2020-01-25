#[macro_use]
extern crate clap;
extern crate image;
extern crate indicatif;
extern crate ndarray;
extern crate num;
extern crate rayon;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use num::complex::*;
use rayon::prelude::*;
use std::ffi::OsStr;
use std::fs::*;
use std::path::Path;

const CMAP_VALUES: [[f32; 3]; 256] = [
    [0.2081f32, 0.1663f32, 0.5292f32],
    [0.2091f32, 0.1721f32, 0.5411f32],
    [0.2101f32, 0.1779f32, 0.5530f32],
    [0.2109f32, 0.1837f32, 0.5650f32],
    [0.2116f32, 0.1895f32, 0.5771f32],
    [0.2121f32, 0.1954f32, 0.5892f32],
    [0.2124f32, 0.2013f32, 0.6013f32],
    [0.2125f32, 0.2072f32, 0.6135f32],
    [0.2123f32, 0.2132f32, 0.6258f32],
    [0.2118f32, 0.2192f32, 0.6381f32],
    [0.2111f32, 0.2253f32, 0.6505f32],
    [0.2099f32, 0.2315f32, 0.6629f32],
    [0.2084f32, 0.2377f32, 0.6753f32],
    [0.2063f32, 0.2440f32, 0.6878f32],
    [0.2038f32, 0.2503f32, 0.7003f32],
    [0.2006f32, 0.2568f32, 0.7129f32],
    [0.1968f32, 0.2632f32, 0.7255f32],
    [0.1921f32, 0.2698f32, 0.7381f32],
    [0.1867f32, 0.2764f32, 0.7507f32],
    [0.1802f32, 0.2832f32, 0.7634f32],
    [0.1728f32, 0.2902f32, 0.7762f32],
    [0.1641f32, 0.2975f32, 0.7890f32],
    [0.1541f32, 0.3052f32, 0.8017f32],
    [0.1427f32, 0.3132f32, 0.8145f32],
    [0.1295f32, 0.3217f32, 0.8269f32],
    [0.1147f32, 0.3306f32, 0.8387f32],
    [0.0986f32, 0.3397f32, 0.8495f32],
    [0.0816f32, 0.3486f32, 0.8588f32],
    [0.0646f32, 0.3572f32, 0.8664f32],
    [0.0482f32, 0.3651f32, 0.8722f32],
    [0.0329f32, 0.3724f32, 0.8765f32],
    [0.0213f32, 0.3792f32, 0.8796f32],
    [0.0136f32, 0.3853f32, 0.8815f32],
    [0.0086f32, 0.3911f32, 0.8827f32],
    [0.0060f32, 0.3965f32, 0.8833f32],
    [0.0051f32, 0.4017f32, 0.8834f32],
    [0.0054f32, 0.4066f32, 0.8831f32],
    [0.0067f32, 0.4113f32, 0.8825f32],
    [0.0089f32, 0.4159f32, 0.8816f32],
    [0.0116f32, 0.4203f32, 0.8805f32],
    [0.0148f32, 0.4246f32, 0.8793f32],
    [0.0184f32, 0.4288f32, 0.8779f32],
    [0.0223f32, 0.4329f32, 0.8763f32],
    [0.0264f32, 0.4370f32, 0.8747f32],
    [0.0306f32, 0.4410f32, 0.8729f32],
    [0.0349f32, 0.4449f32, 0.8711f32],
    [0.0394f32, 0.4488f32, 0.8692f32],
    [0.0437f32, 0.4526f32, 0.8672f32],
    [0.0477f32, 0.4564f32, 0.8652f32],
    [0.0514f32, 0.4602f32, 0.8632f32],
    [0.0549f32, 0.4640f32, 0.8611f32],
    [0.0582f32, 0.4677f32, 0.8589f32],
    [0.0612f32, 0.4714f32, 0.8568f32],
    [0.0640f32, 0.4751f32, 0.8546f32],
    [0.0666f32, 0.4788f32, 0.8525f32],
    [0.0689f32, 0.4825f32, 0.8503f32],
    [0.0710f32, 0.4862f32, 0.8481f32],
    [0.0729f32, 0.4899f32, 0.8460f32],
    [0.0746f32, 0.4937f32, 0.8439f32],
    [0.0761f32, 0.4974f32, 0.8418f32],
    [0.0773f32, 0.5012f32, 0.8398f32],
    [0.0782f32, 0.5051f32, 0.8378f32],
    [0.0789f32, 0.5089f32, 0.8359f32],
    [0.0794f32, 0.5129f32, 0.8341f32],
    [0.0795f32, 0.5169f32, 0.8324f32],
    [0.0793f32, 0.5210f32, 0.8308f32],
    [0.0788f32, 0.5251f32, 0.8293f32],
    [0.0778f32, 0.5295f32, 0.8280f32],
    [0.0764f32, 0.5339f32, 0.8270f32],
    [0.0746f32, 0.5384f32, 0.8261f32],
    [0.0724f32, 0.5431f32, 0.8253f32],
    [0.0698f32, 0.5479f32, 0.8247f32],
    [0.0668f32, 0.5527f32, 0.8243f32],
    [0.0636f32, 0.5577f32, 0.8239f32],
    [0.0600f32, 0.5627f32, 0.8237f32],
    [0.0562f32, 0.5677f32, 0.8234f32],
    [0.0523f32, 0.5727f32, 0.8231f32],
    [0.0484f32, 0.5777f32, 0.8228f32],
    [0.0445f32, 0.5826f32, 0.8223f32],
    [0.0408f32, 0.5874f32, 0.8217f32],
    [0.0372f32, 0.5922f32, 0.8209f32],
    [0.0342f32, 0.5968f32, 0.8198f32],
    [0.0317f32, 0.6012f32, 0.8186f32],
    [0.0296f32, 0.6055f32, 0.8171f32],
    [0.0279f32, 0.6097f32, 0.8154f32],
    [0.0265f32, 0.6137f32, 0.8135f32],
    [0.0255f32, 0.6176f32, 0.8114f32],
    [0.0248f32, 0.6214f32, 0.8091f32],
    [0.0243f32, 0.6250f32, 0.8066f32],
    [0.0239f32, 0.6285f32, 0.8039f32],
    [0.0237f32, 0.6319f32, 0.8010f32],
    [0.0235f32, 0.6352f32, 0.7980f32],
    [0.0233f32, 0.6384f32, 0.7948f32],
    [0.0231f32, 0.6415f32, 0.7916f32],
    [0.0230f32, 0.6445f32, 0.7881f32],
    [0.0229f32, 0.6474f32, 0.7846f32],
    [0.0227f32, 0.6503f32, 0.7810f32],
    [0.0227f32, 0.6531f32, 0.7773f32],
    [0.0232f32, 0.6558f32, 0.7735f32],
    [0.0238f32, 0.6585f32, 0.7696f32],
    [0.0246f32, 0.6611f32, 0.7656f32],
    [0.0263f32, 0.6637f32, 0.7615f32],
    [0.0282f32, 0.6663f32, 0.7574f32],
    [0.0306f32, 0.6688f32, 0.7532f32],
    [0.0338f32, 0.6712f32, 0.7490f32],
    [0.0373f32, 0.6737f32, 0.7446f32],
    [0.0418f32, 0.6761f32, 0.7402f32],
    [0.0467f32, 0.6784f32, 0.7358f32],
    [0.0516f32, 0.6808f32, 0.7313f32],
    [0.0574f32, 0.6831f32, 0.7267f32],
    [0.0629f32, 0.6854f32, 0.7221f32],
    [0.0692f32, 0.6877f32, 0.7173f32],
    [0.0755f32, 0.6899f32, 0.7126f32],
    [0.0820f32, 0.6921f32, 0.7078f32],
    [0.0889f32, 0.6943f32, 0.7029f32],
    [0.0956f32, 0.6965f32, 0.6979f32],
    [0.1031f32, 0.6986f32, 0.6929f32],
    [0.1104f32, 0.7007f32, 0.6878f32],
    [0.1180f32, 0.7028f32, 0.6827f32],
    [0.1258f32, 0.7049f32, 0.6775f32],
    [0.1335f32, 0.7069f32, 0.6723f32],
    [0.1418f32, 0.7089f32, 0.6669f32],
    [0.1499f32, 0.7109f32, 0.6616f32],
    [0.1585f32, 0.7129f32, 0.6561f32],
    [0.1671f32, 0.7148f32, 0.6507f32],
    [0.1758f32, 0.7168f32, 0.6451f32],
    [0.1849f32, 0.7186f32, 0.6395f32],
    [0.1938f32, 0.7205f32, 0.6338f32],
    [0.2033f32, 0.7223f32, 0.6281f32],
    [0.2128f32, 0.7241f32, 0.6223f32],
    [0.2224f32, 0.7259f32, 0.6165f32],
    [0.2324f32, 0.7275f32, 0.6107f32],
    [0.2423f32, 0.7292f32, 0.6048f32],
    [0.2527f32, 0.7308f32, 0.5988f32],
    [0.2631f32, 0.7324f32, 0.5929f32],
    [0.2735f32, 0.7339f32, 0.5869f32],
    [0.2845f32, 0.7354f32, 0.5809f32],
    [0.2953f32, 0.7368f32, 0.5749f32],
    [0.3064f32, 0.7381f32, 0.5689f32],
    [0.3177f32, 0.7394f32, 0.5630f32],
    [0.3289f32, 0.7406f32, 0.5570f32],
    [0.3405f32, 0.7417f32, 0.5512f32],
    [0.3520f32, 0.7428f32, 0.5453f32],
    [0.3635f32, 0.7438f32, 0.5396f32],
    [0.3753f32, 0.7446f32, 0.5339f32],
    [0.3869f32, 0.7454f32, 0.5283f32],
    [0.3986f32, 0.7461f32, 0.5229f32],
    [0.4103f32, 0.7467f32, 0.5175f32],
    [0.4218f32, 0.7473f32, 0.5123f32],
    [0.4334f32, 0.7477f32, 0.5072f32],
    [0.4447f32, 0.7482f32, 0.5021f32],
    [0.4561f32, 0.7485f32, 0.4972f32],
    [0.4672f32, 0.7487f32, 0.4924f32],
    [0.4783f32, 0.7489f32, 0.4877f32],
    [0.4892f32, 0.7491f32, 0.4831f32],
    [0.5000f32, 0.7491f32, 0.4786f32],
    [0.5106f32, 0.7492f32, 0.4741f32],
    [0.5212f32, 0.7492f32, 0.4698f32],
    [0.5315f32, 0.7491f32, 0.4655f32],
    [0.5418f32, 0.7490f32, 0.4613f32],
    [0.5519f32, 0.7489f32, 0.4571f32],
    [0.5619f32, 0.7487f32, 0.4531f32],
    [0.5718f32, 0.7485f32, 0.4490f32],
    [0.5816f32, 0.7482f32, 0.4451f32],
    [0.5913f32, 0.7479f32, 0.4412f32],
    [0.6009f32, 0.7476f32, 0.4374f32],
    [0.6103f32, 0.7473f32, 0.4335f32],
    [0.6197f32, 0.7469f32, 0.4298f32],
    [0.6290f32, 0.7465f32, 0.4261f32],
    [0.6382f32, 0.7460f32, 0.4224f32],
    [0.6473f32, 0.7456f32, 0.4188f32],
    [0.6564f32, 0.7451f32, 0.4152f32],
    [0.6653f32, 0.7446f32, 0.4116f32],
    [0.6742f32, 0.7441f32, 0.4081f32],
    [0.6830f32, 0.7435f32, 0.4046f32],
    [0.6918f32, 0.7430f32, 0.4011f32],
    [0.7004f32, 0.7424f32, 0.3976f32],
    [0.7091f32, 0.7418f32, 0.3942f32],
    [0.7176f32, 0.7412f32, 0.3908f32],
    [0.7261f32, 0.7405f32, 0.3874f32],
    [0.7346f32, 0.7399f32, 0.3840f32],
    [0.7430f32, 0.7392f32, 0.3806f32],
    [0.7513f32, 0.7385f32, 0.3773f32],
    [0.7596f32, 0.7378f32, 0.3739f32],
    [0.7679f32, 0.7372f32, 0.3706f32],
    [0.7761f32, 0.7364f32, 0.3673f32],
    [0.7843f32, 0.7357f32, 0.3639f32],
    [0.7924f32, 0.7350f32, 0.3606f32],
    [0.8005f32, 0.7343f32, 0.3573f32],
    [0.8085f32, 0.7336f32, 0.3539f32],
    [0.8166f32, 0.7329f32, 0.3506f32],
    [0.8246f32, 0.7322f32, 0.3472f32],
    [0.8325f32, 0.7315f32, 0.3438f32],
    [0.8405f32, 0.7308f32, 0.3404f32],
    [0.8484f32, 0.7301f32, 0.3370f32],
    [0.8563f32, 0.7294f32, 0.3336f32],
    [0.8642f32, 0.7288f32, 0.3300f32],
    [0.8720f32, 0.7282f32, 0.3265f32],
    [0.8798f32, 0.7276f32, 0.3229f32],
    [0.8877f32, 0.7271f32, 0.3193f32],
    [0.8954f32, 0.7266f32, 0.3156f32],
    [0.9032f32, 0.7262f32, 0.3117f32],
    [0.9110f32, 0.7259f32, 0.3078f32],
    [0.9187f32, 0.7256f32, 0.3038f32],
    [0.9264f32, 0.7256f32, 0.2996f32],
    [0.9341f32, 0.7256f32, 0.2953f32],
    [0.9417f32, 0.7259f32, 0.2907f32],
    [0.9493f32, 0.7264f32, 0.2859f32],
    [0.9567f32, 0.7273f32, 0.2808f32],
    [0.9639f32, 0.7285f32, 0.2754f32],
    [0.9708f32, 0.7303f32, 0.2696f32],
    [0.9773f32, 0.7326f32, 0.2634f32],
    [0.9831f32, 0.7355f32, 0.2570f32],
    [0.9882f32, 0.7390f32, 0.2504f32],
    [0.9922f32, 0.7431f32, 0.2437f32],
    [0.9952f32, 0.7476f32, 0.2373f32],
    [0.9973f32, 0.7524f32, 0.2310f32],
    [0.9986f32, 0.7573f32, 0.2251f32],
    [0.9991f32, 0.7624f32, 0.2195f32],
    [0.9990f32, 0.7675f32, 0.2141f32],
    [0.9985f32, 0.7726f32, 0.2090f32],
    [0.9976f32, 0.7778f32, 0.2042f32],
    [0.9964f32, 0.7829f32, 0.1995f32],
    [0.9950f32, 0.7880f32, 0.1949f32],
    [0.9933f32, 0.7931f32, 0.1905f32],
    [0.9914f32, 0.7981f32, 0.1863f32],
    [0.9894f32, 0.8032f32, 0.1821f32],
    [0.9873f32, 0.8083f32, 0.1780f32],
    [0.9851f32, 0.8133f32, 0.1740f32],
    [0.9828f32, 0.8184f32, 0.1700f32],
    [0.9805f32, 0.8235f32, 0.1661f32],
    [0.9782f32, 0.8286f32, 0.1622f32],
    [0.9759f32, 0.8337f32, 0.1583f32],
    [0.9736f32, 0.8389f32, 0.1544f32],
    [0.9713f32, 0.8441f32, 0.1505f32],
    [0.9692f32, 0.8494f32, 0.1465f32],
    [0.9672f32, 0.8548f32, 0.1425f32],
    [0.9654f32, 0.8603f32, 0.1385f32],
    [0.9638f32, 0.8659f32, 0.1343f32],
    [0.9623f32, 0.8716f32, 0.1301f32],
    [0.9611f32, 0.8774f32, 0.1258f32],
    [0.9600f32, 0.8834f32, 0.1215f32],
    [0.9593f32, 0.8895f32, 0.1171f32],
    [0.9588f32, 0.8958f32, 0.1126f32],
    [0.9586f32, 0.9022f32, 0.1082f32],
    [0.9587f32, 0.9088f32, 0.1036f32],
    [0.9591f32, 0.9155f32, 0.0990f32],
    [0.9599f32, 0.9225f32, 0.0944f32],
    [0.9610f32, 0.9296f32, 0.0897f32],
    [0.9624f32, 0.9368f32, 0.0850f32],
    [0.9641f32, 0.9443f32, 0.0802f32],
    [0.9662f32, 0.9518f32, 0.0753f32],
    [0.9685f32, 0.9595f32, 0.0703f32],
    [0.9710f32, 0.9673f32, 0.0651f32],
    [0.9736f32, 0.9752f32, 0.0597f32],
    [0.9763f32, 0.9831f32, 0.0538f32],
];

fn map(val: u64, input_range: (u64, u64), output_range: Option<(u64, u64)>) -> u64 {
    let output = output_range.unwrap_or((0u64, 1u64));
    return (output.0 as f64
        + ((output.1 - output.0) as f64 / (input_range.1 - input_range.0) as f64)
            * (val - input_range.0) as f64) as u64;
}
fn cmap(val: u64, input_range: (u64, u64)) -> [u8; 3] {
    // print!("{} -> {}-{}", val, input_range.0, input_range.1);
    let fval = CMAP_VALUES[map(val, input_range, Some((0u64, 255u64))) as usize];
    return [
        (fval[0] * 255f32) as u8,
        (fval[1] * 255f32) as u8,
        (fval[2] * 255f32) as u8,
    ];
}

fn iterate(
    buffer: &mut std::vec::Vec<std::vec::Vec<u64>>,
    size: (u32, u32),
    scale: (f32, f32),
    v: &Complex32,
    max_iter: Option<u64>,
) {
    for x in 0..size.0 {
        for y in 0..size.1 {
            let mut i: u64 = 0;
            let mut z = Complex32::new(x as f32 * scale.0 - 1.5, y as f32 * scale.1 - 1.5);
            while i < max_iter.unwrap_or(1000) && z.norm() <= 2.0 {
                z = z * z + v;
                i += 1;
            }
            buffer[x as usize][y as usize] = i;
        }
    }
}

fn gen_buffer(
    size: (u32, u32),
    scale: (f32, f32),
    v: &Complex32,
    max_iter: Option<u64>,
) -> (std::vec::Vec<std::vec::Vec<u64>>, u64) {
    let mut buffer = vec![vec![0u64; size.1 as usize]; size.0 as usize];
    iterate(&mut buffer, size, scale, v, max_iter);
    let mut max_val = 0;
    for x in 0..size.0 {
        for y in 0..size.1 {
            max_val = std::cmp::max(max_val, buffer[x as usize][y as usize]);
        }
    }
    return (buffer, max_val);
}

fn render_sequence(
    points: std::vec::Vec<Complex32>,
    path: &Path,
    size: (u32, u32),
    scale: (f32, f32),
    max_iter: Option<u64>,
) {
    let bar1 = ProgressBar::new(points.len() as u64);
    bar1.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green.bold} [{elapsed_precise}] [{bar:40.cyan/blue.bold}] {pos:4}/{len:7} ({eta})",
            )
            .progress_chars("#>-"),
    );
    let bar2 = ProgressBar::new(points.len() as u64);
    bar2.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green.bold} [{elapsed_precise}] [{bar:40.cyan/blue.bold}] {pos:4}/{len:7} ({eta})",
            )
            .progress_chars("#>-"),
    );
    let buffers: Vec<(Vec<Vec<u64>>, u64)> = points
        .par_iter()
        .progress_with(bar1)
        .map(|v| gen_buffer(size, scale, v, max_iter))
        .collect();
    let mut max_steps: u64 = 0;
    buffers.iter().for_each(|(_buff, max_iters)| {
        max_steps = std::cmp::max(*max_iters, max_steps);
    });
    buffers
        .par_iter()
        .enumerate()
        .progress_with(bar2)
        .for_each(|(i, (buff, _max_step))| {
            let mut imgbuf = image::RgbImage::new(size.0, size.1);
            for x in 0..size.0 {
                for y in 0..size.1 {
                    imgbuf.put_pixel(
                        x,
                        y,
                        image::Rgb(cmap(buff[x as usize][y as usize], (0, max_steps))),
                    );
                }
            }
            let out = path
                .with_file_name(format!(
                    "{:03}-{}",
                    i,
                    path.file_stem()
                        .unwrap_or(OsStr::new("out"))
                        .to_string_lossy()
                ))
                .with_extension(path.extension().unwrap_or(OsStr::new("png")));
            imgbuf.save(out).unwrap();
        });
}

fn render_gif(
    points: Vec<Complex32>,
    path: &Path,
    size: (u32, u32),
    scale: (f32, f32),
    max_iter: Option<u64>,
) -> std::io::Result<()> {
    let bar1 = ProgressBar::new(points.len() as u64);
    bar1.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green.bold} [{elapsed_precise}] [{bar:40.cyan/blue.bold}] {pos:4}/{len:7} ({eta})",
            )
            .progress_chars("#>-"),
    );
    let bar2 = ProgressBar::new(points.len() as u64);
    bar2.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green.bold} [{elapsed_precise}] [{bar:40.cyan/blue.bold}] {pos:4}/{len:7} ({eta})",
            )
            .progress_chars("#>-"),
    );
    let buffers: Vec<(Vec<Vec<u64>>, u64)> = points
        .par_iter()
        .progress_with(bar1)
        .map(|v| gen_buffer(size, scale, v, max_iter))
        .collect();
    let mut max_steps: u64 = 0;
    buffers.iter().for_each(|(_buff, max_iters)| {
        max_steps = std::cmp::max(*max_iters, max_steps);
    });
    println!("WRITE FILE: {}", path.to_string_lossy());
    let mut file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut encoder = image::gif::Encoder::new(file_out);
    buffers.iter().for_each(|(buff, _max_iter)| {
        let mut imgbuf = image::RgbImage::new(size.0, size.1);
        for x in 0..size.0 {
            for y in 0..size.1 {
                imgbuf.put_pixel(
                    x,
                    y,
                    image::Rgb(cmap(buff[x as usize][y as usize], (0, max_steps))),
                );
            }
        }
        encoder
            .encode(&image::gif::Frame::from_rgb(
                size.0 as u16,
                size.1 as u16,
                &imgbuf.into_raw(),
            ))
            .unwrap();
    });
    Ok(())
}

fn main() {
    let opts = clap_app!(app =>
        (version: "1.0")
        (author: "Arden Rasmussen <ardenrasmussen@lclark.edu>")
        (about: "Fractal path marcher")
        (@arg imgx: -w --width +takes_value "Set image width")
        (@arg imgy: -h --height +takes_value "Set image height")
        (@arg output: -o --out --output +takes_value "Set output image file")
        (@arg samples: -s --samples +takes_value "Set the number of samples to take")
        (@arg iter: -i --iter --iterations +takes_value "Maximum number of iterations")
    )
    .get_matches();
    let scale = (
        3.0 / opts
            .value_of("imgx")
            .unwrap_or("1000")
            .parse::<f32>()
            .unwrap(),
        3.0 / opts
            .value_of("imgy")
            .unwrap_or("1000")
            .parse::<f32>()
            .unwrap(),
    );
    let samples = opts
        .value_of("samples")
        .unwrap_or("100")
        .parse::<u32>()
        .unwrap();
    let step = 2f32 * std::f32::consts::PI / (samples as f32);
    let size = (
        opts.value_of("imgx")
            .unwrap_or("1000")
            .parse::<u32>()
            .unwrap(),
        opts.value_of("imgy")
            .unwrap_or("1000")
            .parse::<u32>()
            .unwrap(),
    );
    let path = Path::new(opts.value_of("output").unwrap_or("imgs/out.png"));

    let points: Vec<Complex32> = (0..samples)
        .map(|s| Complex32::from_polar(&0.7885f32, &(step * (s as f32))))
        .collect();
    if path.extension().unwrap() == "gif" {
        match render_gif(
            points,
            path,
            size,
            scale,
            Some(
                opts.value_of("iter")
                    .unwrap_or("5000")
                    .parse::<u64>()
                    .unwrap(),
            ),
        ) {
            Ok(v) => println!("Saved FILE {:?}", v),
            Err(e) => println!("Error: {:?}", e),
        }
    } else {
        render_sequence(
            points,
            path,
            size,
            scale,
            Some(
                opts.value_of("iter")
                    .unwrap_or("5000")
                    .parse::<u64>()
                    .unwrap(),
            ),
        );
    }
}
