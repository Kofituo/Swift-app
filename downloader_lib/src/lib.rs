mod download_info;
mod java_glue;
//mod java_glue

use crate::download_info::*;
pub use crate::java_glue::*;
use rust_interface_file_generator::gen_attributes_interface_generator::*;

//constants
// 1 MB
const BYTES_TO_WRITE_TO_FILE: u32 = 1_048_576;
//128 KB
const MAX_BYTES_TO_GET_AT_A_TIME: usize = 128 * 1024;
static mut NUMBER_OF_THREADS: u8 = 8;

const DO_NOT_TOUCH: &[u8] =
    b"/** WARNING **/\n/** MACHINE GENERATED FILE **/\n/** DO NOT EDIT **/\n";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        panic!()
    }
}
