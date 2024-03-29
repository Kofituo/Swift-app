#![feature(option_result_unwrap_unchecked)]

mod download_callback;
mod download_info;
mod errors;
mod filetypes;
mod java_glue;
mod logger;
mod macros;

use crate::download_info::*;
use crate::filetypes::*;
pub use crate::java_glue::*;
use crate::logger::*;
use percent_encoding::percent_decode_str;
use reqwest::blocking::Response;
use reqwest::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_TYPE};

use crate::download_callback::DownloadCallback;
use crate::errors::ResponseErrors;
use crate::filetypes::TypeOfFile;
use log::Level;
use mime::Mime;
use rifgen::rifgen_attr::*;
use std::path::PathBuf;
use std::str::FromStr;

//constants
// 1 MB
const BYTES_TO_WRITE_TO_FILE: u32 = 1_048_576;
//128 KB
const MAX_BYTES_TO_GET_AT_A_TIME: usize = 128 * 1024;
static mut NUMBER_OF_THREADS: u8 = 8;

const DO_NOT_TOUCH: &[u8] =
    b"/** WARNING **/\n/** MACHINE GENERATED FILE **/\n/** DO NOT EDIT **/\n";
const TIMEOUT: u8 = 8;

///Struct to download data
struct Downloader {
    request_info: RequestInfo,
}

impl Downloader {
    #[generate_interface]
    fn get_request_info(
        download_info: &DownloadInfo,
        download_callback: &Box<dyn DownloadCallback>,
    ) -> Option<RequestInfo> {
        let mut error = None;
        for _ in 0..TIMEOUT {
            //should only execute if it's active
            if !download_callback.is_active() {
                log::log!(Level::Error, "not active");
                return None;
            }
            let mut builder = reqwest::blocking::Client::new().get(download_info.get_url());
            if let Some(val) = &download_info.auth {
                builder = builder.basic_auth(&val.username, val.password.as_ref());
            }

            match builder.send() {
                Ok(val) => {
                    //success
                    let (type_of_file, filename) = Downloader::filename_and_type(&val);
                    return RequestInfo::new(
                        download_info.clone(),
                        filename,
                        val.content_length().unwrap_or_default() as i64,
                        type_of_file,
                        val.headers().contains_key(ACCEPT_RANGES),
                    )
                    .into();
                }
                Err(err) => error = Some(err),
            }
        }
        Downloader::propagate_error(error.unwrap(), download_callback.as_ref());
        None
    }

    fn filename_and_type(response: &Response) -> (TypeOfFile, String) {
        let disposition = response.headers().get(CONTENT_DISPOSITION);
        let name_from_url = //first check the header then the title then the url
            disposition.map(|it| it.to_str())
                .and_then(|it| it.ok())
                .and_then(|mut it| {
                    let index = it.find("filename=");
                    if let Some(index) = index {
                        it = &it[index + "filename=".len()..];
                        //it could be filename = "name" or filename = name
                        if let Some(val) = it.get(0..1) {
                            if val == r#"""# { it = &it[1..it.len() - 1] }
                        }
                        Some(it.to_string())
                    } else { None }
                })
                .unwrap_or_else(|| response.url().query_pairs().find(|(key, _)| {
                    key == "title"
                }).map(|(_, val)| String::from(val))
                    .unwrap_or_else(|| {
                        response
                            .url()
                            .path_segments()
                            .and_then(|segments| segments.last())
                            .and_then(|name| if name.is_empty() { None } else {
                                let decoded_name = percent_decode_str(name).decode_utf8();
                                Some(if let Ok(name) = decoded_name { name.into() } else { name.into() })
                            })
                            .unwrap_or_else(|| String::from("tmp.bin"))
                    })
                );
        let mut name_path: PathBuf = path_buf!(&name_from_url);
        match name_path.extension() {
            None => {
                let ext = match response.headers().get(CONTENT_TYPE) {
                    None => String::new(),
                    Some(content_type) => {
                        let content_type =
                            Mime::from_str(content_type.to_str().unwrap_or_default())
                                .unwrap_or(mime::TEXT_PLAIN);
                        match content_type.subtype() {
                            mime::PLAIN => "txt".to_string(),
                            it => it.to_string(),
                        }
                    }
                };
                name_path.set_extension(&ext);
                (
                    //get_type isn't exhaustive so we fall back to mime type
                    match FileType::new(&ext).get_type() {
                        TypeOfFile::Other => Downloader::get_mime_type(response),
                        ty => ty,
                    },
                    format!("{}", name_path.to_string_lossy()),
                )
            }
            Some(ext) => (
                match FileType::new(ext.to_string_lossy().as_ref()).get_type() {
                    TypeOfFile::Other => Downloader::get_mime_type(response),
                    ty => ty,
                },
                name_from_url,
            ),
        }
    }

    fn get_mime_type(response: &Response) -> TypeOfFile {
        match response.headers().get(CONTENT_TYPE) {
            None => TypeOfFile::Other,
            Some(content_type) => {
                let mut ret = TypeOfFile::default();
                if let Ok(string) = content_type.to_str() {
                    if let Ok(mime) = Mime::from_str(string) {
                        //
                        ret = match mime.type_() {
                            mime::APPLICATION => TypeOfFile::Application,
                            mime::AUDIO => TypeOfFile::Audio,
                            mime::VIDEO => TypeOfFile::Video,
                            mime::IMAGE => TypeOfFile::Image,
                            _ => TypeOfFile::default(),
                        };
                    }
                }
                ret
            }
        }
    }

    fn propagate_error(error: reqwest::Error, download_callback: &dyn DownloadCallback) {
        if error.is_body() | error.is_request() {
            download_callback.response_error(ResponseErrors::ErrorParsingRequest)
        } else if error.is_decode() {
            download_callback.response_error(ResponseErrors::UnableToDecodeRequest)
        } else if error.is_redirect() {
            download_callback.response_error(ResponseErrors::RedirectedManyTimes)
        } else if error.is_timeout() {
            download_callback.response_error(ResponseErrors::ConnectionTimeout)
        } else if error.is_status() {
            if let Some(stat) = error.status() {
                download_callback.status_error(stat.as_u16(), stat.canonical_reason().unwrap_or(""))
            } else {
                download_callback.response_error(ResponseErrors::ErrorParsingRequest)
            }
        } else {
            download_callback.response_error(ResponseErrors::UnknownError)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        panic!()
    }
}
