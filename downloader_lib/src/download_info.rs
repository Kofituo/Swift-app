use derive_new::new;
use rust_interface_file_generator::gen_attributes_interface_generator::*;

#[generate_interface_doc]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Authentication {
    username: String,
    password: Option<String>,
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
    auth: Option<Authentication>,
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
struct SaveInfo<'auth, 'cat> {
    ///Total length of the file if specified by the site
    total_length: Option<u64>,
    ///Amount of bytes downloaded
    downloaded_length: u64,
    ///Authentication if any
    auth: Option<&'auth Authentication>,
    ///Chosen category
    category: &'cat String,
}
