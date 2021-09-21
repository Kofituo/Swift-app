use flapigen::LanguageConfig::JavaConfig;
use rust_interface_file_generator::{Generator, Language, TypeCases};
use std::env;
use std::path::{Path, PathBuf};

//see https://github.com/Dushistov/flapigen-rs/issues/395

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let in_src = path_buf!("src", "java_glue.rs.in");
    Generator::new(TypeCases::CamelCase, Language::Java, path_buf!("src"))
        .generate_interface(&in_src);
    //panic!();
    //std::fs::write("/home/kofi/Desktop/t.txt", &out_dir).unwrap();
    let out_src = Path::new(&out_dir).join("java_glue.rs");
    //delete the lib folder then create it again to prevent obsolete files from staying
    let out_dir = path_buf!("C:\\Users\\taimoor\\Desktop\\Swift_final\\app\\src\\main\\java\\com\\example\\swift_final\\lib");
    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir).unwrap();
    }
    std::fs::create_dir(&out_dir).unwrap();
    let swig_gen = flapigen::Generator::new(JavaConfig(
        flapigen::JavaConfig::new(out_dir, "com.example.swift_final.lib".into())
            .use_null_annotation_from_package("androidx.annotation".into()),
    ))
    .rustfmt_bindings(true);
    swig_gen.expand("android bindings", &in_src, &out_src);
    println!("cargo:rerun-if-changed={}", "src");
}

///Creates a new [`PathBuf`] from the arguments
#[macro_export]
macro_rules! path_buf {
    ($($others:expr),+) => {{
        collect!(PathBuf,<[_]>::iter(&[$($others),+]))
    }};
}

#[macro_export]
macro_rules! collect {
    ($type:ty,$value:expr) => {
        $value.collect::<$type>()
    };
}
