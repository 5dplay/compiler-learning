use std::io::{self, Read};

use compiler_tools::form_blocks::{blocks_print, linear_print};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    linear_print(&buffer);
    blocks_print(&buffer);
}
