fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod main {

    #[test]
    fn it_works() {
        println!("Boo!");
    }
}