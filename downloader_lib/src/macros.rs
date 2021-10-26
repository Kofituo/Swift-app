#[macro_export]
///Creates a new [`PathBuf`] from the arguments
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

#[macro_export]
macro_rules! get_type_sc {
    ($self:expr, $($types:tt),+)=> {{
        $ (
            if $self.$types() {
                return casey::pascal!($types)
            }

        )+
        Other
    }};
}

#[macro_export]
macro_rules! match_mime_type {
    ($mime_type:expr,$($types:tt),+) => {{
        match $mime_type {
            $(
                casey::upper!($types) => TypeOfFile::$types,
            )+
            _=>TypeOfFile::default()
        }
    }};
}
