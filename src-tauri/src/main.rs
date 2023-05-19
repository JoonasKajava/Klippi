// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use modules::config::user_settings::UserSettings;
use modules::ffmpeg::get_version;
use modules::tauri_commands::get_latest_videos;
use modules::tauri_commands::install_dependencies;
use modules::tauri_commands::verify_dependencies;
use tauri::http::ResponseBuilder;
use url::Position;
use url::Url;

use std::io::{Write};
use tauri::http::{
    header::{ACCEPT_RANGES, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE, RANGE},
    status::StatusCode,
    MimeType,
};

use crate::modules::config::app_config::AppConfig;
use crate::modules::config::Init;
use crate::modules::environment::add_to_env_variable;
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            UserSettings::init(&app.config())?;
            AppConfig::init(&app.config())?;
            println!("Version: {:?}", get_version("ffprobe"));
            println!("Version: {:?}", get_version("ffmpeg"));

            add_to_env_variable("Path", "test")?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_latest_videos,
            verify_dependencies,
            install_dependencies
        ])
        // TODO: THIS IS FIX FOR https://github.com/tauri-apps/tauri/issues/6375 remove in future
        .register_uri_scheme_protocol("stream", move |_app, request| {
            let parsed_path = Url::parse(request.uri())?;
            let filtered_path = &parsed_path[..Position::AfterPath];
            let path = filtered_path
                .strip_prefix("stream://localhost/")
                // the `strip_prefix` only returns None when a request is made to `https://tauri.$P` on Windows
                // where `$P` is not `localhost/*`
                .unwrap_or("");
            let path = percent_encoding::percent_decode(path.as_bytes())
                .decode_utf8_lossy()
                .to_string();

            let mut file = std::fs::File::open(&path)?;

            // get current position
            let old_pos = file.seek(SeekFrom::Current(0))?;
            // get file len
            let len = file.seek(SeekFrom::End(0))?;
            // reset position
            file.seek(SeekFrom::Start(old_pos))?;

            // get the mime type
            let mut prelude: [u8; 256] = [0; 256];
            file.read(&mut prelude)?;
            let mime_type = MimeType::parse(&prelude, &path);

            // reset position
            file.seek(SeekFrom::Start(0))?;

            let mut resp = ResponseBuilder::new().header(CONTENT_TYPE, &mime_type);

            let response = if let Some(x) = request.headers().get(RANGE) {
                let not_satisfiable = || {
                    ResponseBuilder::new()
                        .status(StatusCode::RANGE_NOT_SATISFIABLE)
                        .header(CONTENT_RANGE, format!("bytes */{len}"))
                        .body(vec![])
                };

                resp = resp.header(ACCEPT_RANGES, "bytes");

                let ranges = http_range_header::parse_range_header(x.to_str()?)?;
                let ranges = ranges.validate(len);
                let ranges: Vec<_> = if let Ok(x) = ranges {
                    x.iter().map(|r| (*r.start(), *r.end())).collect()
                } else {
                    return not_satisfiable();
                };

                const MAX_LEN: u64 = 1000 * 1024;

                if ranges.len() == 1 {
                    let &(start, mut end) = ranges.first().unwrap();

                    if start >= len || end >= len || end < start {
                        return not_satisfiable();
                    }

                    // adjust end byte for MAX_LEN
                    end = start + (end - start).min(len - start).min(MAX_LEN - 1);

                    // calculate number of bytes needed to be read
                    let bytes_to_read = end + 1 - start;

                    // allocate a buf with a suitable capacity
                    let mut buf = Vec::with_capacity(bytes_to_read as usize);
                    // seek the file to the starting byte
                    file.seek(SeekFrom::Start(start))?;
                    // read the needed bytes
                    file.take(bytes_to_read).read_to_end(&mut buf)?;

                    resp = resp.header(CONTENT_RANGE, format!("bytes {start}-{end}/{len}"));
                    resp = resp.header(CONTENT_LENGTH, end + 1 - start);
                    resp = resp.status(StatusCode::PARTIAL_CONTENT);
                    resp.body(buf)
                } else {
                    let mut buf = Vec::new();
                    let ranges = ranges
                        .iter()
                        .filter_map(|&(start, mut end)| {
                            if start >= len || end >= len || end < start {
                                None
                            } else {
                                end = start + (end - start).min(len - start).min(MAX_LEN - 1);
                                Some((start, end))
                            }
                        })
                        .collect::<Vec<_>>();

                    let boundary = random_boundary();
                    let boundary_sep = format!("\r\n--{boundary}\r\n");
                    let boundary_closer = format!("\r\n--{boundary}\r\n");

                    resp = resp.header(
                        CONTENT_TYPE,
                        format!("multipart/byteranges; boundary={boundary}"),
                    );

                    for (end, start) in ranges {
                        // a new range is being written, write the range boundary
                        buf.write_all(boundary_sep.as_bytes())?;

                        // write the needed headers `Content-Type` and `Content-Range`
                        buf.write_all(format!("{CONTENT_TYPE}: {mime_type}\r\n").as_bytes())?;
                        buf.write_all(
                            format!("{CONTENT_RANGE}: bytes {start}-{end}/{len}\r\n").as_bytes(),
                        )?;

                        // write the separator to indicate the start of the range body
                        buf.write_all("\r\n".as_bytes())?;

                        // calculate number of bytes needed to be read
                        let bytes_to_read = end + 1 - start;

                        let mut local_buf = vec![0_u8; bytes_to_read as usize];
                        file.seek(SeekFrom::Start(start))?;
                        file.read_exact(&mut local_buf)?;
                        buf.extend_from_slice(&local_buf);
                    }
                    // all ranges have been written, write the closing boundary
                    buf.write_all(boundary_closer.as_bytes())?;

                    resp.body(buf)
                }
            } else {
                let mut buf = Vec::with_capacity(len as usize);
                file.read_to_end(&mut buf)?;
                resp = resp.header(CONTENT_LENGTH, len);
                resp.body(buf)
            };

            response
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn random_boundary() -> String {
    use rand::RngCore;

    let mut x = [0 as u8; 30];
    rand::thread_rng().fill_bytes(&mut x);
    (&x[..])
        .iter()
        .map(|&x| format!("{:x}", x))
        .fold(String::new(), |mut a, x| {
            a.push_str(x.as_str());
            a
        })
}
