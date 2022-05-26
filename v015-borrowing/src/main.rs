fn main(){
  let s1 = String::from("hello"); 
  print_string(&s1);
  println!("{}", s1);

 }
 fn print_string(x:&String){
    println!("{}",x);
 }