mod args;

use std::{env, io};
use std::collections::VecDeque;


pub fn grep(args : &mut VecDeque<String>) -> Result<String, Err> {
    // get list of operations to do on file, return struct of those operations in a stack
    args::parse_args(args);
    // if recursively search through every directory...
    // generate a collection of files to search from in the directory
    todo!()
}

pub(crate) fn init() {
    grep( env::args().collect() as &mut VecDeque<String>);
}