fn main(){
  let mut s1 = String::from("hello"); 
  
  s1.push_str("string");

  print_string(&s1);

  println!("{}", s1);

 }
 fn print_string(x:&String){
    println!("{}",x);
 }