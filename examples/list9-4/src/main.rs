use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(f) => f,
        Err(err) => {
            panic!("Problem open the file: {:?}", err); // thread 'main' panicked at 'Problem open the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }'
        },
    };
    println!("Get file: {:#?}", f);
}
