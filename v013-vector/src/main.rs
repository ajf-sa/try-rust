fn main() {
    let list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let o: Vec<i32> = list.into_iter().filter(|x| x % 2 == 0).collect();
    println!("{:?}", o);
}
