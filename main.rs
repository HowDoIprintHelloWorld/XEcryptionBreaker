use std::env;
use std::process;
use itertools::Itertools;


// Checks parameters for 'help' and 'keygen' and calls the functions
// when they are requested
fn check_params() {
  let args: Vec<String> = env::args().collect();
  if args.len() >= 2 {
    let first_param = String::from(args[1].clone());
    if first_param == "key-gen" {
      if args.len() == 3 {
        let second_param = String::from(args[2].clone()).parse::<i32>().unwrap();
        auto_gen_pwd(second_param)
      } else {
        print!("[F]  Two parameters expected (See help!)\n")
      }
      process::exit(0);
    } else if first_param == "help" {
      help();
      process::exit(0);
    }
  } else {
    print!("[F]  No arguments given. Use help!");
    process::exit(1);
  }
}


// Displays the help page
fn help() {
  let explanation = "
  USAGE:
  XEcryptionBreaker [operation] (key)


  >>   Decryption (usage: 'XEcryptionBreaker [file name]')
  Put the to-be-decrypted message into a file and pass the file's location on to this program.
  
  This program currently has two limitations with its decryption:
    a) It can only decrypt messages with spaces in them
    b) It can not automatically detect the correct encrypted message

  After it is done decrypting, it will output all possible decrypted messages along with \ntheir respective key. Look through all of the messages until you find the correctly decypted one \n(This shouldn't be too difficult since the rest will be nonsense!)



  >>   Key Generation (usage: 'XEcryptionBreaker key-gen [key-value]')
  If you want to modify the encrypted message and re-encrypt it using the same key, you can use \nthis handy feature. XEcryption does not need the same key to en/decrypt a message,\nit only needs the same sum of the ascii values of all the characters in the key to match up.

  >> Help (usage: 'XEcryptionBreaker help')
  Displays this page.

  Credits:
    - Understanding of algorithm and approach to breaking it:
      https://mvddvm.blogspot.com/2007/04/breaking-xecryption.html
  ";
  print!("{explanation}\n");
}

// Takes an input vector and combines three following elements into its own vector
fn split_in_threes(vec: Vec<i16>) -> Vec<Vec<i16>> {
  let mut split_vec = Vec::new();
  let mut tmp_vec = Vec::new();
  let mut counter = 0;
  for element in vec {
    if counter == 3 {
      split_vec.push(tmp_vec);
      tmp_vec = Vec::new();
      counter = 0;
    }
    tmp_vec.push(element);
    counter += 1;
  }
  split_vec
}

// Filters out empty vector components, makes sure it can be split up
// in groups of threes and turns strings to ints
fn filter_vec(vec: Vec<String>) -> Vec<Vec<i16>> {
  let mut filtered_vec = Vec::new();
  for element in vec {
    if element.len() > 0 {
      filtered_vec.push(element.parse::<i16>().unwrap());
    }
  }
  if filtered_vec.len() % 3 != 0 {
    print!("[F]   Number of 'parts' must be a multiple of three!");
    process::exit(1);
  }
  split_in_threes(filtered_vec)
}

// Gets file name from first argument if more than two arguments are given
fn get_file() -> String {
  let args: Vec<String> = env::args().collect();
  if args.len() == 2 {
    String::from(args[1].clone())
  } else {
    process::exit(1);
  }
}

// Reads contents of file
fn read_file(file_path: String) -> String {
  let mut file_contents = std::fs::read_to_string(file_path).expect("Error reading file!");
  file_contents = file_contents.split_whitespace().collect();
  file_contents
}

// Turns the files contents into a vector
fn format_contents(file_contents: String) -> Vec<String> {
  let split = file_contents.split(".").clone();
  let mut formatted_vec = Vec::new(); 
  for element in split {
    formatted_vec.push(element.to_string().clone())
  }
  formatted_vec
}

// Adds each group of three together and subtracts the unicode for space from each one of them. Assuming one character is space, the result should be the key. Also returns a vector with all the (encrypted) characters sets of threes added together
fn generate_decoders(split_up_vec: Vec<Vec<i16>>, char_val: i16) -> (Vec<i16>, Vec<i16>)  {
  let mut decoders = Vec::new();
  let mut added_vec = Vec::new();
  for sub_vec in split_up_vec {
    let mut sum = 0;
    for num in sub_vec {
      sum += num;
    }
    added_vec.push(sum);
    decoders.push(sum - char_val);
  }
  (decoders, added_vec)
}


// Takes all the keys from the decoders and tries each one. If any of the
// resulting characters in the entire message is not valid UTF8, it 
// will move on to the next key. The remaning messages are displayed 
// with the keys used to decode them
fn decode(added_vec: Vec<i16>, decoders: Vec<i16>) {
  // print!("{:?}", added_vec);
  'decoders: for decoder in decoders {
    let mut message = String::from("");
    for enc_char in added_vec.clone() {
      if enc_char - decoder < 0 || enc_char - decoder > 127 { continue 'decoders; }
      let dec_char = &[(enc_char - decoder) as u8];
      message += std::str::from_utf8(dec_char).expect("[F]   Can't convert '{}' to string!");
    }
    print!("[!]    Possibility found using key with combined ascii value '{}':\n\n{message}\n\n\n\n\n", decoder);
  }
}

// (Works, but theres a cleaner approach!) 
// Takes an input integer, which is all of the UTF8 characters of all 
// of the characters of the key added together. It then generates a 
// key with an equal UTF8-character sum and displays it. Due to how the
// XEcryption algorithm works, only the crosssum of the characters in the 
// must be equal, not the key itself!
fn auto_gen_pwd(pwd_utf8: i32) {
  let mut pwd_utf8 = pwd_utf8.clone();
  let mut utf8_vec = Vec::new();
  let (min, max, mut to_be_sub) = (33, 122, 0);

  while pwd_utf8 != 0 {
    let new_pwd = pwd_utf8 - (max - to_be_sub);
    if new_pwd == 0 || new_pwd > min {
      pwd_utf8 = new_pwd;
      utf8_vec.push((max - to_be_sub) as u8);
      to_be_sub = 0;
    } else {
      to_be_sub += 1;
      if to_be_sub < -10 {
        process::exit(1);
      }
    }
  }
  let result = String::from_utf8(utf8_vec).unwrap();
  print!("[R]  Result: {}\n", result);
}

// Calls all of the functions in order
fn main() {
  let char_val = 32; // Subtract this from the sums to get a possible key!

  check_params();
  let file_path = get_file();
  let file_contents = read_file(file_path);
  let formatted_vec = format_contents(file_contents);
  let (decoders, added_vec) = generate_decoders(filter_vec(formatted_vec), char_val);
  let decoders: Vec<_> = decoders.into_iter().unique().collect();
  decode(added_vec, decoders.clone());
}
