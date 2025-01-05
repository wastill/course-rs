use std::fs::File;

mod tests {
    use super::*;

    #[test]
    fn test_file() {
        let f = File::open("hello.txt");
        println!("{:?}", f);
    }
}
