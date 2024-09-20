// use std::collections::HashMap;


fn main(){

    let val = vec![2,3,4,5];
    let val_iter = val.iter().filter(|x|*x%2==1).map(|x|x*2);
   for x in val_iter{
    println!("vals{} ",x);
   }
   
}



