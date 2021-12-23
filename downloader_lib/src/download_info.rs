use crate::filetypes::{FileType, TypeOfFile};
use crate::to_bytes_format;
use log::Level;
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
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
    #[inline]
    pub fn get_url(&self) -> &str {
        &self.url
    }
}

#[generate_interface_doc]
#[derive(serde::Serialize, serde::Deserialize)]
/// Struct to hold information received from the server
pub struct RequestInfo {
    download_info: DownloadInfo,
    filename: String,
    ///File size in bytes
    file_size: Option<u64>,
    type_of_file: TypeOfFile,
    resumable: bool,
}

impl RequestInfo {
    #[generate_interface(constructor)]
    #[inline]
    pub fn new(
        download_info: DownloadInfo,
        filename: String,
        file_size: i64,
        category: TypeOfFile,
        resumable: bool,
    ) -> RequestInfo {
        RequestInfo {
            download_info,
            filename,
            file_size: if file_size == 0 {
                None
            } else {
                Some(file_size as u64)
            },
            type_of_file: category,
            resumable,
        }
    }

    #[generate_interface]
    pub fn get_url(&self) -> &str {
        self.download_info.get_url()
    }

    #[generate_interface]
    pub fn get_file_size(&self) -> Option<String> {
        self.file_size.map(|it| to_bytes_format!(it))
    }

    #[generate_interface]
    pub fn get_file_size_in_bytes(&self) -> u64 {
        self.file_size.unwrap_or_default()
    }

    #[generate_interface]
    pub fn is_resumable(&self) -> bool {
        self.resumable
    }

    #[generate_interface]
    pub fn type_of_file(&self) -> TypeOfFile {
        self.type_of_file
    }

    #[generate_interface]
    pub fn get_category(&self) -> FileCategory {
        FileType::get_category(self.type_of_file)
    }

    #[generate_interface]
    pub fn filename(&self) -> &str {
        &self.filename
    }

    #[generate_interface]
    pub fn to_string(&self) -> String {
        unsafe { serde_json::to_string(self).unwrap_unchecked() }
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
