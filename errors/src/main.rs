use std::fs::File;
use std::io::ErrorKind;


fn main() {
    let _f: Result<std::fs::File, std::io::Error> = File::open("hello.txt");

    let _f = match _f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }, 
            _ => {
                panic!("Problem opening the file")
            }
        }
    };


    // version using closures
    let _f = File::open("hello1.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("hello1.txt").unwrap_or_else(|err| {
                panic!("problem creating the file: {:?}", err);
            })
        } else {
            panic!("Probkem opening the file: {:?}", e);
        }
    });


    // unwrap() calls the panic! for us if Error.
    let _f = File::open("hello3.txt").unwrap();

    // same as above but allows the message to be passed in for panic!
    let _f = File::open("hello4.txt").expect("Failed to open hello4.txt");

}
