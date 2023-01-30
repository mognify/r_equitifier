use std::io::Read;
use std::io::Write;

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

// https://stackoverflow.com/questions/72526999/rust-equivalent-of-java-hashmap-initialization
//let mut ds_map: HashMap<String, Vec<u32>> = HashMap::new();

// https://www.tutorialspoint.com/rust/rust_collections.htm
let mut got_vars = Vec::new();
let mut set_vars = Vec::new();
let mut results = Vec::new();
let mut file_loc = String::from("get:none results:none set:none");

fn main(){
  println!("levi wuz here");
}

//fn set_input_source(txt:String){
//fn set_output_source(txt:String){

/*
 * file_loc = the path to the file location
 * dataset = varGet, results, varSet
*/
fn set_source(dataset:String, file_loc:String, ){
  eprintln!("set_source(dataset:{dataset}, file_loc:{file_loc})");
  
  file_loc = file_loc.replace("{dataset}:"
}

// https://www.tutorialspoint.com/rust/rust_file_input_output.htm
// https://stackoverflow.com/questions/59164456/how-do-i-return-an-array-from-a-rust-function
fn read_source(file_loc:String) -> Vec<u32>{
  eprintln!("read_source(file_loc:{file_loc})");
  
  let mut file = std::fs::File::open(file_loc).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  eprintln("{contents}");
}

// https://www.tutorialspoint.com/rust/rust_file_input_output.htm
fn write_adjustments(){
  let mut file = std::fs::File::create(
}
