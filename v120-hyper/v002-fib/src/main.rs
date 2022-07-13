mod fib;
use fib::ib::App;


 fn main() {
    let mut app = App::new("abdulaziz");

    app.get("/", |s| {
        println!("this is get  {}",s);
        Ok(())
    });
    
    app.post("/", |s| {
        println!("this is post {}",s);
        Ok(())
    });
    println!("count : {}", app.count)

}
