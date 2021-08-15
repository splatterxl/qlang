use qlang::File;

fn main() {

    let file_data = String::from("echo (\"Hello, World!\");");

    let mut file = File {
        data: file_data,
        filename: String::from("aaaaa.q"),
        parsed: None
    };

    file.parse();

    dbg!(&file);

}
