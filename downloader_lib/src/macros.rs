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
macro_rules! to_bytes_format {
    ($int:expr) => {
        byte_unit::Byte::from($int)
            .get_appropriate_unit(true)
            .format(2)
    };
}
