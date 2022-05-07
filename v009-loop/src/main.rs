fn main() {
  for x in 1..10 {
    if x % 2 == 0{
        println!("this is even number {}",x);
    } else {
        println!("this is odd number {}",x);
    }
  } 

  let mut x = 0;
   while x < 10{
      x+=1;
      println!("inside loop x value is {}",x);
   }
   println!("outside loop x value is {}",x);

   let mut x = 0;
   loop {
      x+=1;
      println!("x={}",x);

      if x==15 {
         break;
      }
   }
}
