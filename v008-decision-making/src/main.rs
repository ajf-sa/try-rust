fn main() {
  let age :i32 = 25;
  if age == 25 {
      println!("your age is {}",age)
  }

  let sallary = 10_000;
  if sallary > 5_000 {
      println!("you can get loan !")
  } else {
      println!("we are very sorry!")
  }

  let num = 2 ;
  if num > 0 {
     println!("{} is positive",num);
  } else if num < 0 {
     println!("{} is negative",num);
  } else {
     println!("{} is neither positive nor negative",num) ;
  }

  // match 
  let currency_code = "SAR";
  let code = match currency_code {
      "SAR" => {"Saudi Rayils"},
      "USD" => {"usa dollar"},
      _ => {"unknown!"}   
  };
  println!("use currancy {}",code);
}
