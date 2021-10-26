use crate::download_info::FileCategory;
use crate::filetypes::TypeOfFile::*;
use crate::get_type_sc;
use rifgen::rifgen_attr::*;

#[generate_interface]
pub enum TypeOfFile {
    Word,
    Excel,
    PowerPoint,
    Jpg,
    Mkv,
    Png,
    Html,
    MpFour,
    Pdf,
    Other,
    Application,
    Audio,
    MpThree,
    Gif,
    Video,
    Zip,
    Image,
    Iso,
    ThreeGp,
    Flv,
    Document,
    Compressed,
}

impl Default for TypeOfFile {
    fn default() -> Self {
        TypeOfFile::Other
    }
}

pub struct FileType {
    file_extension: String,
}

impl FileType {
    #[generate_interface(constructor)]
    pub fn new(string: &str) -> FileType {
        FileType {
            file_extension: string.into(),
        }
    }

    fn word(&self) -> bool {
        matches!(
            self.file_extension.as_str(),
            "doc" | "docx" | "wbk" | "docm" | "dotx" | "docb"
        )
    }

    fn excel(&self) -> bool {
        matches!(
            self.file_extension.as_str(),
            "xls" | "xlt" | "xlm" | "xlsx" | "xlsm" | "xltx" | "xltm"
        )
    }

    fn power_point(&self) -> bool {
        matches!(
            self.file_extension.as_str(),
            "ppt"
                | "pot"
                | "pps"
                | "pptx"
                | "pptm"
                | "potx"
                | "potm"
                | "ppsx"
                | "ppam"
                | "sldx"
                | "sldm"
                | "ppsm"
        )
    }

    fn jpg(&self) -> bool {
        matches!(
            self.file_extension.as_str(),
            "jpg" | "jpeg" | "jpe" | "jif" | "jfif" | "jfi"
        )
    }

    fn png(&self) -> bool {
        matches!(self.file_extension.as_str(), "png")
    }

    fn html(&self) -> bool {
        matches!(self.file_extension.as_str(), "html" | "htm")
    }

    fn mkv(&self) -> bool {
        matches!(self.file_extension.as_str(), "mkv" | "mk3d" | "mka" | "mks")
    }

    fn mp_four(&self) -> bool {
        matches!(
            self.file_extension.as_str(),
            "mp4" | "m4a" | "m4p" | "m4b" | "m4r" | "m4v" | "mov"
        )
    }

    fn pdf(&self) -> bool {
        matches!(self.file_extension.as_str(), "pdf")
    }

    fn mp_three(&self) -> bool {
        matches!(self.file_extension.as_str(), "mp3")
    }

    fn gif(&self) -> bool {
        self.file_extension.as_str() == "gif"
    }

    fn zip(&self) -> bool {
        matches!(
            self.file_extension.as_str(),
            "bz2"
                | "gz"
                | "lz"
                | "lz4"
                | "lzma"
                | "lzo"
                | "rz"
                | "sfark"
                | "sz"
                | "xz"
                | "z"
                | "Z"
                | "zst"
                | "7z"
                | "s7z"
                | "ace"
                | "dmg"
                | "jar"
                | "kgb"
                | "lzx"
                | "rar"
                | "sda"
                | "tbz2"
                | "tlz"
                | "uc"
                | "war"
                | "zipx"
                | "zip"
                | "zz"
        )
    }

    fn iso(&self) -> bool {
        matches!(self.file_extension.as_str(), "iso")
    }

    fn three_gp(&self) -> bool {
        matches!(
            self.file_extension.as_str(),
            "3gp" | "3gp2" | "3g2" | "3gpp" | "3gpp2"
        )
    }

    fn flv(&self) -> bool {
        self.file_extension == "flv"
    }

    #[generate_interface]
    pub fn get_type(&self) -> TypeOfFile {
        get_type_sc!(
            self,
            excel,
            word,
            power_point,
            jpg,
            mkv,
            png,
            html,
            mp_four,
            pdf,
            mp_three,
            gif,
            zip,
            iso,
            three_gp,
            flv
        )
    }

    #[generate_interface]
    pub fn get_category(type_of_file: TypeOfFile) -> FileCategory {
        match type_of_file {
            TypeOfFile::Mkv
            | TypeOfFile::MpFour
            | TypeOfFile::ThreeGp
            | TypeOfFile::Flv
            | TypeOfFile::Video => FileCategory::Video,

            TypeOfFile::Excel
            | TypeOfFile::PowerPoint
            | TypeOfFile::Pdf
            | TypeOfFile::Html
            | TypeOfFile::Word
            | TypeOfFile::Document => FileCategory::Document,
            TypeOfFile::Jpg | TypeOfFile::Png | TypeOfFile::Gif | TypeOfFile::Image => {
                FileCategory::Image
            }
            TypeOfFile::Iso | TypeOfFile::Zip | TypeOfFile::Compressed => FileCategory::Compressed,
            TypeOfFile::MpThree | TypeOfFile::Audio => FileCategory::Audio,
            TypeOfFile::Application => FileCategory::Application,
            TypeOfFile::Other => FileCategory::Other,
        }
    }
}
