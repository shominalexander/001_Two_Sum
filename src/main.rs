fn request(query: &str) -> String {
 let mut text = String::new();

 println!("{}", query);

 match std::io::stdin().read_line(&mut text) {
  Ok(result) => {
   if result > 0 {
    text = text.trim().to_string();
    text = text.replace("\n", "") ;
    text = text.replace("\r", "") ;
   }//if result > 0 {
  }//Ok(result) => {

  Err(error) => {
   println!("error: {:?}", error);

  }//Err(error) => { 
 }//match std::io::stdin().read_line(&mut text) {

 text
}//fn request(text: &str) -> String {

fn main() {
 'vector: loop {
  let mut text: String = request("\n\nVec<i32>:");
 
  match &text[..] {
   "back" | "exit" => { break; } 
   _ => {
    match serde_json::from_str::<Vec<i32>>(&text[..]) {
     Ok(vector) => {
      loop {
       text = request("\ni32:");

       match &text[..] {
        "back" => { break        ; } 
        "exit" => { break 'vector; } 
        _ => {
         match serde_json::from_str::<i32>(&text[..]) {
          Ok(sum) => {
           let mut index: usize                            ;
           let mut items = std::collections::HashMap::new();

           index = 0usize;
           for value in &vector {
            items.insert(value, index);

            index += 1;
           }//for value in &vector {

           index = 0usize;
           for value in &vector {
            if let Some(answer) = items.get(&(sum - value)) {
             if *answer != index {
              println!("answer: {:?}", vec![index, *answer]);

              break;
             }//if *answer != index {
            }//if let Some(answer) = items.get(&(sum - value)) {

            index += 1;
           }//for value in &vector {
          }//Ok(sum) => {

          Err(error) => {
           println!("error: {:?}", error);

          }//Err(error) => { 
         }//match serde_json::from_str::<Vec<i32>>(&text[..]) {
        }//_ => {
       }//match &text[..] {
      }//loop {
     }//Ok(vector) => {

     Err(error) => {
      println!("error: {:?}", error);

     }//Err(error) => { 
    }//match serde_json::from_str::<Vec<i32>>(&text[..]) {
   }//_ => {
  }//match &text[..] {
 }//'vector: loop {
}//fn main() {
