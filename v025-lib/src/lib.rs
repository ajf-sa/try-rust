struct MyIterWrapper<'a,T>{
    slice:&'a[T]
}

impl<'a,T> Iterator for MyIterWrapper <'a,T>{

    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>{
   let elemnt = self.slice.get(0) ;
   todo!()
    }
}
#[cfg(test)]
mod tests {
    use crate::MyIterWrapper;

    #[test]
    fn it_works() {
     let mut collection = vec![1,2,3,4,5,6,7] ;
        let wrapper = MyIterWrapper{
            slice: &collection[..],
        };
     for c in wrapper {

     
     }
    }
}
