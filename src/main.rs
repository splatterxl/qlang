fn main() {
    qlang::file(
        &std::env::args()
            .nth(1)
            .expect("file argument must be passed"),
    );
}
