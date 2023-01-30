use std:io:Read;

/*
server notes the variable values it's using
as well as the resulting outcomes

that info is then digested by this program
this program then notes down its suggested adjustments to the var vals

the server then digests it

it will be like a mailbox system.
probably 3 files:
2 for variable values (is, should)
1 for outcome values
*/

fn main(){
  println!("levi wuz here");
}

//fn set_input_source(txt:String){
//fn set_output_source(txt:String){

/*
 * file_loc = the path to the file location
 * input = whether this is the var input or the value outcome output file
 * ofc true = it's the var input, and false = it's the outcome output
*/
fn set_source_file(file_loc:String, input:bool){
  eprintln!("Read file location as: {file_loc}");
}

// https://www.tutorialspoint.com/rust/rust_file_input_output.htm
// https://stackoverflow.com/questions/59164456/how-do-i-return-an-array-from-a-rust-function
fn read_source(file_loc:String) -> Vec<u32>{
  let mut file = std::fs::File::open(file_loc).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  eprintln("{contents}");
}
