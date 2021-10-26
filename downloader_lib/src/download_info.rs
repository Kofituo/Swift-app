use crate::filetypes::TypeOfFile;
use derive_new::new;
use rifgen::rifgen_attr::*;

#[generate_interface_doc]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Authentication {
    pub username: String,
    pub password: Option<String>,
}

impl Authentication {
    #[generate_interface(constructor)]
    pub fn new(username: String, password: Option<&str>) -> Authentication {
        Authentication {
            username,
            password: password.map(|it| it.to_string()),
        }
    }
}

pub struct DownloadInfo {
    url: String,
    pub auth: Option<Authentication>,
}

impl DownloadInfo {
    #[generate_interface(constructor)]
    pub fn constructor(url: String, auth: Option<Authentication>) -> DownloadInfo {
        DownloadInfo { url, auth }
    }

    #[generate_interface]
    pub fn get_url(&self) -> &str {
        &self.url
    }
}

///Holds information about the download when it's paused
#[derive(Debug, serde::Serialize, serde::Deserialize, new)]
struct SaveInfo {
    ///Total length of the file if specified by the site
    total_length: Option<u64>,
    ///Amount of bytes downloaded
    downloaded_length: u64,
    ///Authentication if any
    auth: Option<Authentication>,
    ///Chosen category
    category: String,
}

#[generate_interface_doc]
/// Struct to hold information received from the server
pub struct RequestInfo {
    download_info: DownloadInfo,
    filename: String,
    ///File size in bytes
    file_size: Option<u64>,
    category: TypeOfFile,
}

impl RequestInfo {
    #[generate_interface(constructor)]
    #[inline]
    pub fn new(
        download_info: DownloadInfo,
        filename: String,
        file_size: Option<i64>,
        category: TypeOfFile,
    ) -> RequestInfo {
        RequestInfo {
            download_info,
            filename,
            file_size: file_size.map(|i| i as u64),
            category,
        }
    }
}
#[generate_interface]
pub enum FileCategory {
    Video,
    Document,
    Image,
    Compressed,
    Audio,
    Application,
    Other,
}
