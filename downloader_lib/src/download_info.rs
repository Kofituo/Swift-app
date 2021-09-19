use rust_interface_file_generator::gen_attributes_interface_generator::*;
#[generate_interface_doc]
#[derive(Clone)]
pub struct Authorisation {
    username: String,
    password: Option<String>,
}

impl Authorisation {
    #[generate_interface(constructor)]
    pub fn new() -> Authorisation {
        panic!()
    }
}

pub struct DownloadInfo {
    url: String,
    auth: Option<Authorisation>,
}

impl DownloadInfo {
    #[generate_interface(constructor)]
    pub fn new(url: String, auth: Option<Authorisation>) -> DownloadInfo {
        panic!();
        //DownloadInfo { url, auth }
    }
}
