#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Item { index: usize, value: i32 }

fn request() -> String {
 let mut value: String = String::new();

 std::io::stdin().read_line(&mut value).expect("Input failed");

 value = value.trim().to_string();
 value = value.replace("\n", "") ;
 value = value.replace("\r", "") ;

 value
}//fn request() -> String {

fn main() {
 loop {
  println!("\r\n\r\nvector:");

  let mut input: String = request();
 
  if &input[..] == "exit" {
   break;   

  } else {//if &input[..] == "exit" {
   let mut equal  : bool                = false                                                  ;
   let     integer: Vec<i32>            = serde_json::from_str(&input[..]).expect("Wrong format");
   let mut items  : std::vec::Vec<Item> = std::vec::Vec::new()                                   ;
   let mut minimum: i32                 = 0                                                      ;
   let     size   : usize               = integer.len()                                          ;

   println!("\r\ntarget:");

   input = request();

   if &input[..] == "exit" {
    break;   

   } else {//if &input[..] == "exit" {
    let mut left_current : usize = 0                                                 ;
    let mut left_nearest : usize = 0                                                 ;
    let     last         : usize = size - 1                                          ;      
    let mut right_current: usize = last                                              ;
    let mut right_nearest: usize = 0                                                 ;
    let     target       : i32   = (&input[..]).trim().parse().expect("Wrong format");

    if size > 1usize {
     for i in 0..size {
      items.push(Item{index: i, value: integer[i]});

     }//for i in 0..size {

     items.sort_by(|a, b| a.value.cmp(&b.value));

     while left_current < right_current {
      let sum: i32 = items[left_current].value + items[right_current].value;

      if sum == target {
       equal = true;
 
       break;

      } else {//if sum == target {
       let difference: i32;

       if sum < target {
        difference = target - sum;

       } else {//if sum < target {
        difference = sum - target;

       }//} else {//if sum < target {

       if left_current == 0 && right_current == last {
        minimum       = difference;
        left_nearest  = 0         ;
        right_nearest = last      ;

       } else {//if left_current == 0 && right_current == last {
        if difference < minimum {
         left_nearest  = left_current ;
         minimum       = difference   ;
         right_nearest = right_current;
        }//if difference < minimum {
       }//} else {//if left_current == 0 && right_current == last {

       if sum < target {
        left_current += 1;

       } else {//if sum < target {
        right_current -= 1;

       }//} else {//if sum < target {
      }//} else {//if sum == target {
     }//while left_current < right_current {
    }//if size > 1usize {

    if equal {
     if items[left_current].index < items[right_current].index {
      println!("\r\ntarget:\r\n{:?}" , vec![items[left_current].index, items[right_current].index]);

     } else {//if items[left_current].index < items[right_current].index {
      println!("\r\ntarget:\r\n{:?}", vec![items[right_current].index, items[left_current].index]);

     }//} else {//if items[left_current].index < items[right_current].index {

    } else {//if equal {
     if items[left_nearest].index < items[right_nearest].index {
      println!("\r\nnearest:\r\n{:?}", vec![items[left_nearest].index, items[right_nearest].index]);

     } else {//if items[left_nearest].index < items[right_nearest].index {
      println!("\r\nnearest:\r\n{:?}", vec![items[right_nearest].index, items[left_nearest].index]);

     }//} else {//if items[left_nearest].index < items[right_nearest].index {
    }//if equal {//} else {//if equal {
   }//} else {//if &input[..] == "exit" {
  }//} else {//if &input[..] == "exit" {
 }//loop {
}//fn main() {
