use std::io::Cursor;

use actix_http::StatusCode;
use actix_web::{ get, post, web, HttpResponse, Result };
use image::{ imageops::FilterType, DynamicImage, ImageFormat };
use qirust::{
    helper::{
        generate_frameqr_buffer,
        generate_image_buffer,
        generate_svg_string,
        hex_to_rgb,
        FrameStyle,
    },
    qrcode::{ QrCode, QrCodeEcc, Version },
};
use serde::{ de, Deserialize, Deserializer, Serialize };

#[derive(Deserialize)]
struct SVG {
    data: String,
}

#[derive(Serialize)]
struct ResponseData {
    qr: String,
}

#[derive(Deserialize, Clone)]
struct QR {
    data: String,
    #[serde(default, deserialize_with = "deserialize_size")]
    size: Option<(u32, u32)>,
    color: Option<String>,
    border: Option<u32>,
    inner_px: Option<u32>,
    frame_style: Option<String>,
    scale: Option<u32>,
    ecc: Option<String>,
    bg_color: Option<String>,
}

fn deserialize_size<'de, D>(deserializer: D) -> Result<Option<(u32, u32)>, D::Error>
    where D: Deserializer<'de>
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(val) => {
            let parts: Vec<&str> = val.split('x').collect();
            if parts.len() != 2 {
                return Err(de::Error::custom("Invalid size format: expected 'widthxheight'"));
            }
            let width = parts[0]
                .parse::<u32>()
                .map_err(|_| de::Error::custom("Invalid width in size"))?;
            let height = parts[1]
                .parse::<u32>()
                .map_err(|_| de::Error::custom("Invalid height in size"))?;

            Ok(Some((width, height)))
        }
        None => Ok(None),
    }
}

#[post("/qr")]
async fn get_svg(info: web::Json<SVG>) -> Result<HttpResponse> {
    let svg_string = generate_svg_string(&info.data);
    Ok(HttpResponse::Ok().content_type("application/json").json(ResponseData { qr: svg_string }))
}

#[get("/qr")]
async fn generate_qr(info: web::Query<QR>) -> Result<HttpResponse> {
    let qr_color = hex_to_rgb(&info.color.clone().unwrap_or("#000000".to_string())).unwrap_or([
        0, 0, 0,
    ]);
    let bg_color = hex_to_rgb(&info.bg_color.clone().unwrap_or("#FFFFFF".to_string())).unwrap_or([
        255, 255, 255,
    ]);
    let border_modules = info.border.unwrap_or(1).max(1);
    let size = info.size.unwrap_or((500, 500));
    let scale = info.scale.unwrap_or(4);

    let buffer = generate_image_buffer(
        &info.data,
        Some(border_modules),
        Some(qr_color),
        Some(bg_color),
        Some(scale)
    ).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let buffer_rgba = image::ImageBuffer::from_fn(buffer.width(), buffer.height(), |x, y| {
        let rgb = buffer.get_pixel(x, y);
        image::Rgba([rgb[0], rgb[1], rgb[2], 255])
    });

    let mut cursor = Cursor::new(Vec::new());
    let dyn_img = DynamicImage::ImageRgba8(buffer_rgba);
    let resized_img = dyn_img.resize(size.0, size.1, FilterType::Lanczos3);
    resized_img
        .write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let png_bytes = cursor.into_inner();

    Ok(HttpResponse::build(StatusCode::OK).content_type("image/png").body(png_bytes))
}

#[get("/frameqr")]
async fn generate_qrs(info: web::Query<QR>) -> Result<HttpResponse> {
    let qr_color = hex_to_rgb(&info.color.clone().unwrap_or("#000000".to_string())).unwrap_or([
        0, 0, 0,
    ]);
    let border_modules = info.border.unwrap_or(1).max(1);
    let inner_frame_px = info.inner_px.unwrap_or(0);
    let frame_style = match info.frame_style.as_deref() {
        Some("rounded") => FrameStyle::Rounded,
        Some("square") => FrameStyle::Square,
        _ => FrameStyle::None,
    };
    let size = info.size.unwrap_or((500, 500));
    let scale = info.scale.unwrap_or(16);
    let ecc = match info.ecc.as_deref() {
        Some("L") => QrCodeEcc::Low,
        Some("M") => QrCodeEcc::Medium,
        Some("H") => QrCodeEcc::High,
        Some("Q") => QrCodeEcc::Quartile,
        _ => QrCodeEcc::High,
    };

    let data_len = info.data.len().max(Version::MAX.buffer_len());
    let mut outbuffer = vec![0u8; data_len];
    let mut tempbuffer = vec![0u8; data_len];

    let qr = QrCode::encode_text(
        &info.data,
        &mut tempbuffer,
        &mut outbuffer,
        ecc,
        Version::MIN,
        Version::MAX,
        None,
        true
    ).unwrap();

    let buffer = generate_frameqr_buffer(
        qr,
        "src/static/images/gh.png",
        Some(scale),
        Some(qr_color),
        Some(border_modules),
        Some(inner_frame_px),
        Some(frame_style)
    );

    let buffer_rgba = image::ImageBuffer::from_fn(buffer.width(), buffer.height(), |x, y| {
        let rgb = buffer.get_pixel(x, y);
        image::Rgba([rgb[0], rgb[1], rgb[2], 255])
    });

    let mut cursor = Cursor::new(Vec::new());
    let dyn_img = DynamicImage::ImageRgba8(buffer_rgba);
    let resized_img = dyn_img.resize(size.0, size.1, FilterType::Lanczos3);
    resized_img
        .write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let png_bytes = cursor.into_inner();

    Ok(HttpResponse::build(StatusCode::OK).content_type("image/png").body(png_bytes))
}
