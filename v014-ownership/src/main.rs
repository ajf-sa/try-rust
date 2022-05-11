use std::vec;

fn main() {
 let v = vec![1, 2, 4];
 let v2 = v;
 display(v);
 //error ^ value used here after move
 println!("In main {:?}",v2);

}

fn display(v:Vec<i32>){
    println!("inside display {:?}",v);
}
