struct Authorisation {
    username: String,
    password:Option<String>
}

struct DownloadInfo {
    url: String,
    auth:Option<Authorisation>,
}

impl DownloadInfo {
    //fn new(url:String,)
}