use ab_glyph::{FontRef, PxScale};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use exif::{In, Reader as ExifReader, Tag, Value};
use image::{imageops, DynamicImage, ImageBuffer, ImageFormat, Rgba};
use imageproc::drawing::draw_text_mut;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{BufWriter, Cursor, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportRequest {
    image_data: String,
    output_path: String,
    output_format: String,
    canvas_width: u32,
    canvas_height: u32,
    background: String,
    text_color: String,
    border_enabled: bool,
    border_color: String,
    border_width: u32,
    photo_area_x: u32,
    photo_area_y: u32,
    photo_area_width: u32,
    photo_area_height: u32,
    info_x: u32,
    info_y: u32,
    info_width: u32,
    font_size: u32,
    line_height: u32,
    lines: Vec<String>,
    font_family: String,
    fit_mode: String,
    crop_focus_x: f32,
    crop_focus_y: f32,
    zoom: f32,
    offset_x: f32,
    offset_y: f32,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct PhotoMetadata {
    camera: String,
    lens: String,
    focal: String,
    aperture: String,
    shutter: String,
    iso: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NativePhoto {
    name: String,
    data_url: String,
    width: u32,
    height: u32,
    size: u64,
    metadata: PhotoMetadata,
    exif_available: bool,
}

fn rational(value: &Value) -> Option<(u32, u32)> {
    match value {
        Value::Rational(values) => values.first().map(|v| (v.num, v.denom)),
        Value::SRational(values) => values
            .first()
            .map(|v| (v.num.max(0) as u32, v.denom.max(1) as u32)),
        _ => None,
    }
}
fn format_focal(value: &Value) -> String {
    rational(value)
        .map(|(num, den)| {
            if den > 0 {
                format!("{}mm", (num as f64 / den as f64).round())
            } else {
                String::new()
            }
        })
        .unwrap_or_default()
}
fn format_aperture(value: &Value) -> String {
    rational(value)
        .map(|(num, den)| {
            if den > 0 {
                format!("ƒ{:.1}", num as f64 / den as f64).replace('.', ",")
            } else {
                String::new()
            }
        })
        .unwrap_or_default()
}
fn format_shutter(value: &Value) -> String {
    rational(value)
        .map(|(num, den)| {
            if num == 0 || den == 0 {
                return String::new();
            }
            if num < den {
                format!("1/{}s", (den as f64 / num as f64).round())
            } else {
                format!("{}s", (num as f64 / den as f64).round())
            }
        })
        .unwrap_or_default()
}
fn format_iso(value: &Value) -> String {
    match value {
        Value::Short(values) => values
            .first()
            .map(|v| format!("ISO{}", v))
            .unwrap_or_default(),
        Value::Long(values) => values
            .first()
            .map(|v| format!("ISO{}", v))
            .unwrap_or_default(),
        _ => String::new(),
    }
}
fn read_exif(bytes: &[u8]) -> (PhotoMetadata, bool) {
    let mut cursor = Cursor::new(bytes);
    let Ok(exif) = ExifReader::new().read_from_container(&mut cursor) else {
        return (PhotoMetadata::default(), false);
    };
    let text = |tag: Tag| {
        exif.get_field(tag, In::PRIMARY)
            .map(|field| {
                field
                    .display_value()
                    .with_unit(&exif)
                    .to_string()
                    .trim_matches('\"')
                    .to_string()
            })
            .unwrap_or_default()
    };
    let camera = text(Tag::Model);
    let lens = text(Tag::LensModel);
    let focal = exif
        .get_field(Tag::FocalLength, In::PRIMARY)
        .map(|field| format_focal(&field.value))
        .unwrap_or_default();
    let aperture = exif
        .get_field(Tag::FNumber, In::PRIMARY)
        .map(|field| format_aperture(&field.value))
        .unwrap_or_default();
    let shutter = exif
        .get_field(Tag::ExposureTime, In::PRIMARY)
        .map(|field| format_shutter(&field.value))
        .unwrap_or_default();
    let iso = exif
        .get_field(Tag::PhotographicSensitivity, In::PRIMARY)
        .map(|field| format_iso(&field.value))
        .unwrap_or_default();
    (
        PhotoMetadata {
            camera,
            lens,
            focal,
            aperture,
            shutter,
            iso,
        },
        true,
    )
}
fn image_mime(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "png" => "image/png",
        "webp" => "image/webp",
        "tif" | "tiff" => "image/tiff",
        _ => "image/jpeg",
    }
}
#[tauri::command]
fn read_image_files(paths: Vec<String>) -> Result<Vec<NativePhoto>, String> {
    paths
        .into_iter()
        .map(|raw| {
            let path = PathBuf::from(&raw);
            let bytes =
                fs::read(&path).map_err(|e| format!("{} okunamadı: {e}", path.display()))?;
            let image = image::load_from_memory(&bytes)
                .map_err(|e| format!("{} görüntü olarak açılamadı: {e}", path.display()))?;
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("fotoğraf")
                .to_string();
            let (metadata, exif_available) = read_exif(&bytes);
            Ok(NativePhoto {
                name,
                data_url: format!(
                    "data:{};base64,{}",
                    image_mime(&path),
                    STANDARD.encode(&bytes)
                ),
                width: image.width(),
                height: image.height(),
                size: bytes.len() as u64,
                metadata,
                exif_available,
            })
        })
        .collect()
}
fn font_bytes(id: &str) -> &'static [u8] {
    match id {
        "courier" => include_bytes!("../assets/CourierNew.ttf"),
        "din" => include_bytes!("../assets/DINAlternateBold.ttf"),
        "arial" => include_bytes!("../assets/Arial.ttf"),
        "times" => include_bytes!("../assets/TimesNewRoman.ttf"),
        "din-condensed" => include_bytes!("../assets/DINCondensedBold.ttf"),
        "arial-narrow" => include_bytes!("../assets/ArialNarrow.ttf"),
        "verdana" => include_bytes!("../assets/Verdana.ttf"),
        "trebuchet" => include_bytes!("../assets/TrebuchetMS.ttf"),
        "georgia" => include_bytes!("../assets/Georgia.ttf"),
        "stix" => include_bytes!("../assets/STIXGeneral.otf"),
        "chalkduster" => include_bytes!("../assets/Chalkduster.ttf"),
        _ => include_bytes!("../assets/SFNSMono.ttf"),
    }
}

fn parse_hex(value: &str) -> Rgba<u8> {
    let value = value.trim().trim_start_matches('#');
    let expanded = if value.len() == 3 {
        value.chars().map(|c| format!("{c}{c}")).collect::<String>()
    } else {
        value.to_string()
    };
    if expanded.len() == 6 {
        let r = u8::from_str_radix(&expanded[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&expanded[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&expanded[4..6], 16).unwrap_or(255);
        Rgba([r, g, b, 255])
    } else {
        Rgba([255, 255, 255, 255])
    }
}

fn decode_data_url(value: &str) -> Result<Vec<u8>, String> {
    value
        .split_once(',')
        .map(|(_, encoded)| STANDARD.decode(encoded).map_err(|e| e.to_string()))
        .unwrap_or_else(|| STANDARD.decode(value).map_err(|e| e.to_string()))
}

fn safe_path(value: &str) -> PathBuf {
    let path = PathBuf::from(value);
    if path.is_absolute() {
        path
    } else {
        PathBuf::from(value.trim())
    }
}

fn wrap_for_width(lines: &[String], info_width: u32, font_size: u32) -> Vec<String> {
    let chars_per_line =
        ((info_width as f32 / (font_size.max(8) as f32 * 0.64)).floor() as usize).max(8);
    let mut result = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let mut current = String::new();
        for word in line.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_string()
            } else {
                format!("{current} {word}")
            };
            if candidate.chars().count() > chars_per_line && !current.is_empty() {
                result.push(current);
                current = word.to_string();
            } else {
                current = candidate;
            }
        }
        if !current.is_empty() {
            result.push(current);
        }
    }
    result
}

fn draw_border(canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, color: Rgba<u8>, width: u32) {
    let w = canvas.width();
    let h = canvas.height();
    let thickness = width.max(1).min(w.min(h) / 2).max(1);
    for offset in 0..thickness {
        let right = w.saturating_sub(offset + 1);
        let bottom = h.saturating_sub(offset + 1);
        for x in offset..=right {
            canvas.put_pixel(x, offset, color);
            canvas.put_pixel(x, bottom, color);
        }
        for y in offset..=bottom {
            canvas.put_pixel(offset, y, color);
            canvas.put_pixel(right, y, color);
        }
    }
}

#[tauri::command]
fn get_default_export_dir() -> Result<String, String> {
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    let dir = PathBuf::from(home)
        .join("Downloads")
        .join("IG-Kamera-Bilgisi");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command]
fn export_composite(request: ExportRequest) -> Result<String, String> {
    if request.canvas_width == 0 || request.canvas_height == 0 {
        return Err("Canvas ölçüsü geçersiz.".into());
    }
    let source_bytes = decode_data_url(&request.image_data)?;
    let source =
        image::load_from_memory(&source_bytes).map_err(|e| format!("Fotoğraf okunamadı: {e}"))?;
    let source_rgba = source.to_rgba8();
    let source_w = source_rgba.width().max(1);
    let source_h = source_rgba.height().max(1);
    let mut canvas = ImageBuffer::from_pixel(
        request.canvas_width,
        request.canvas_height,
        parse_hex(&request.background),
    );

    let area_w = request.photo_area_width.max(1);
    let area_h = request.photo_area_height.max(1);
    let base_scale = if request.fit_mode == "cover" {
        (area_w as f32 / source_w as f32).max(area_h as f32 / source_h as f32)
    } else {
        (area_w as f32 / source_w as f32).min(area_h as f32 / source_h as f32)
    };
    let scale = (base_scale * request.zoom.max(0.1)).max(0.0001);
    let placed_w = ((source_w as f32 * scale).round() as u32).max(1);
    let placed_h = ((source_h as f32 * scale).round() as u32).max(1);
    let resized = imageops::resize(
        &source_rgba,
        placed_w,
        placed_h,
        imageops::FilterType::Lanczos3,
    );

    let (final_image, dest_x, dest_y) = if request.fit_mode == "cover" {
        let max_x = placed_w.saturating_sub(area_w);
        let crop_x = ((max_x as f32) * request.crop_focus_x.clamp(0.0, 1.0)).round() as u32;
        let max_y = placed_h.saturating_sub(area_h);
        let crop_y = ((max_y as f32) * request.crop_focus_y.clamp(0.0, 1.0)).round() as u32;
        let cropped = imageops::crop_imm(
            &resized,
            crop_x,
            crop_y,
            area_w.min(placed_w),
            area_h.min(placed_h),
        )
        .to_image();
        (
            cropped,
            request.photo_area_x as i64,
            request.photo_area_y as i64,
        )
    } else {
        let x = request.photo_area_x as f32
            + (area_w as f32 - placed_w as f32) / 2.0
            + request.offset_x;
        let y = request.photo_area_y as f32
            + (area_h as f32 - placed_h as f32) / 2.0
            + request.offset_y;
        (resized, x.round() as i64, y.round() as i64)
    };
    imageops::overlay(&mut canvas, &final_image, dest_x, dest_y);

    if request.border_enabled {
        draw_border(
            &mut canvas,
            parse_hex(&request.border_color),
            request.border_width,
        );
    }

    let font = FontRef::try_from_slice(font_bytes(&request.font_family))
        .map_err(|_| "Teknik yazı tipi yüklenemedi.".to_string())?;
    let lines = wrap_for_width(&request.lines, request.info_width, request.font_size);
    let scale = PxScale::from(request.font_size.max(8) as f32);
    let text_color = parse_hex(&request.text_color);
    for (index, line) in lines.iter().enumerate() {
        let y = request.info_y as i32
            + (index as u32 * request.line_height.max(request.font_size + 6)) as i32;
        draw_text_mut(
            &mut canvas,
            text_color,
            request.info_x as i32,
            y,
            scale,
            &font,
            line,
        );
    }

    let output_path = safe_path(&request.output_path);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let output_format = request.output_format.to_lowercase();
    let format = match output_format.as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "tiff" | "tif" => ImageFormat::Tiff,
        _ => ImageFormat::Png,
    };
    match format {
        ImageFormat::Jpeg => {
            let rgb = DynamicImage::ImageRgba8(canvas).to_rgb8();
            let file =
                fs::File::create(&output_path).map_err(|e| format!("Export başarısız: {e}"))?;
            let mut writer = BufWriter::new(file);
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut writer, 100);
            encoder
                .encode_image(&DynamicImage::ImageRgb8(rgb))
                .map_err(|e| format!("Export başarısız: {e}"))?;
            writer
                .flush()
                .map_err(|e| format!("Export başarısız: {e}"))?;
        }
        _ => DynamicImage::ImageRgba8(canvas)
            .save_with_format(&output_path, format)
            .map_err(|e| format!("Export başarısız: {e}"))?,
    };
    Ok(output_path.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_default_export_dir,
            read_image_files,
            export_composite
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn request(
        source: DynamicImage,
        path: PathBuf,
        width: u32,
        height: u32,
        mode: &str,
        lines: Vec<String>,
    ) -> ExportRequest {
        let mut bytes = Cursor::new(Vec::new());
        source
            .write_to(&mut bytes, ImageFormat::Png)
            .expect("source png");
        ExportRequest {
            image_data: format!(
                "data:image/png;base64,{}",
                STANDARD.encode(bytes.into_inner())
            ),
            output_path: path.to_string_lossy().to_string(),
            output_format: "png".into(),
            canvas_width: width,
            canvas_height: height,
            background: "ffffff".into(),
            text_color: "111111".into(),
            border_enabled: true,
            border_color: "dddddd".into(),
            border_width: 2,
            photo_area_x: 80,
            photo_area_y: 60,
            photo_area_width: width - 160,
            photo_area_height: height.saturating_sub(330),
            info_x: 80,
            info_y: height.saturating_sub(220),
            info_width: width - 160,
            font_size: 24,
            line_height: 34,
            lines,
            font_family: "sf-mono".into(),
            fit_mode: mode.into(),
            crop_focus_x: 0.5,
            crop_focus_y: 0.5,
            zoom: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    #[test]
    fn required_aspect_ratio_exports_preserve_canvas_dimensions() {
        let cases = [
            (90, 160, 1080, 1350),
            (160, 90, 1080, 1350),
            (100, 100, 1080, 1350),
            (150, 100, 1080, 1440),
            (150, 100, 1080, 228),
        ];
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        for (source_w, source_h, canvas_w, canvas_h) in cases {
            let source = DynamicImage::ImageRgba8(ImageBuffer::from_pixel(
                source_w,
                source_h,
                Rgba([210, 140, 70, 255]),
            ));
            let path = std::env::temp_dir().join(format!(
                "ig-kamera-accept-{stamp}-{source_w}x{source_h}.png"
            ));
            let output = export_composite(request(
                source,
                path.clone(),
                canvas_w,
                canvas_h,
                "contain",
                vec![
                    "TEST CAMERA".into(),
                    "TEST LENS".into(),
                    "60mm f6.3 1/125s ISO400".into(),
                ],
            ))
            .expect("export");
            let rendered = image::open(&output).expect("rendered image");
            assert_eq!((rendered.width(), rendered.height()), (canvas_w, canvas_h));
            assert!(path.exists());
            fs::remove_file(path).ok();
        }
    }

    #[test]
    fn export_supports_jpeg_and_tiff_code_paths() {
        let source =
            DynamicImage::ImageRgba8(ImageBuffer::from_pixel(120, 80, Rgba([30, 80, 120, 255])));
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        for (extension, format) in [("jpg", "jpg"), ("tiff", "tiff")] {
            let path = std::env::temp_dir().join(format!("ig-kamera-accept-{stamp}.{extension}"));
            let mut req = request(
                source.clone(),
                path.clone(),
                320,
                400,
                "contain",
                vec!["CAMERA".into()],
            );
            req.output_format = format.into();
            export_composite(req).expect("format export");
            assert!(image::open(&path).is_ok());
            fs::remove_file(path).ok();
        }
    }
}

#[cfg(test)]
mod exif_tests {
    use super::*;

    #[test]
    fn reads_exif_from_local_sony_fixture_when_available() {
        let path = Path::new("/Users/akb/Desktop/ajksfvda/test photo 2.jpg");
        if !path.exists() {
            return;
        }
        let bytes = fs::read(path).expect("fixture bytes");
        let (metadata, available) = read_exif(&bytes);
        assert!(available);
        assert!(!metadata.camera.is_empty());
        assert!(!metadata.focal.is_empty());
        assert!(!metadata.aperture.is_empty());
        assert!(!metadata.shutter.is_empty());
        assert!(!metadata.iso.is_empty());
        println!("EXIF fixture: {:?}", metadata);
    }
}

#[cfg(test)]
mod import_tests {
    use super::*;

    #[test]
    fn native_import_returns_data_url_and_exif_metadata_when_fixture_exists() {
        let path = Path::new("/Users/akb/Desktop/ajksfvda/test photo 2.jpg");
        if !path.exists() {
            return;
        }
        let photos =
            read_image_files(vec![path.to_string_lossy().to_string()]).expect("native import");
        assert_eq!(photos.len(), 1);
        assert!(photos[0].data_url.starts_with("data:image/jpeg;base64,"));
        assert!(photos[0].width > 0 && photos[0].height > 0);
        assert!(photos[0].exif_available);
        assert_eq!(photos[0].metadata.shutter, "1/250s");
    }
}

#[cfg(test)]
mod font_tests {
    use super::*;

    #[test]
    fn bundled_typefaces_are_loadable() {
        for id in [
            "sf-mono",
            "courier",
            "din",
            "din-condensed",
            "arial",
            "arial-narrow",
            "verdana",
            "trebuchet",
            "georgia",
            "times",
            "stix",
            "chalkduster",
        ] {
            assert!(
                FontRef::try_from_slice(font_bytes(id)).is_ok(),
                "font failed: {id}"
            );
        }
    }
}
