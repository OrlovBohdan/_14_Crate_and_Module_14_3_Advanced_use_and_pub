#[test]

/*
use std::fmt::Result;
use std::io::Result;

fn main() {}
*/
fn main() {
    // Тепер ви можете використовувати FmtResult і IoResult
    let _fmt_result: FmtResult = Ok(());
    let _io_result: IoResult<()> = Ok(());

    println!("Success!");
}
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;



/*
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn main() {
    // Тепер ви можете використовувати FmtResult і IoResult
    let fmt_result: FmtResult = Ok(());
    let io_result: IoResult<()> = Ok(());

    println!("Success!");
}

*/