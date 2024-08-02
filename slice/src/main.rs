
fn main() {
   // let mut s =String::from("Hello world!");
   // let wordIndex = first_word(&s);
   // s.clear();
   // println!("{}",wordIndex);
   let mut v=vec![1,2,3];
   let first=2;
   v[0] = 5;
   let first=&v[0];
   println!("{}",first);
   println!("{:?}",v.get(0));
   

}
// fn first_word(s:&String) -> &str {
//    let bytes=s.as_bytes();
//    for(i,&item) in bytes.iter().enumerate(){
//     if item==b' '{
//         return i;
//     }
//    }
//    s.len()
// }
