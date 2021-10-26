use crate::errors::ResponseErrors;
use rifgen::rifgen_attr::*;

#[generate_interface]
pub trait DownloadCallback {
    fn response_error(&self, error: ResponseErrors) {}

    fn status_error(&self, error_code: u16, reason: &str) {}
}
