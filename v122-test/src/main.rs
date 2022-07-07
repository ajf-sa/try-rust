fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests{
    #[test]
    fn work(){
        let resutl = 2+2;
        assert_eq!(resutl,4);
    }
}