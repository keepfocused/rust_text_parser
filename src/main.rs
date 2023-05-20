
use std::thread;
use regex::Regex;
use std::fs;
mod idioms_struct;
mod string_to_regex_converter;
const NONE: Option<Regex> = None;
use std::sync::Mutex;
use std::sync::Arc;

// Сделать паблик интерфейс 
// тесты 
// скомпоновать модуль 

fn main() {
let idiom_search_patterns = create_regex_idioms();
//let extractedIdioms = extract_idiom(idiom_search_patterns);
// let extractedIdioms = extractedIdiomsAsync(idiom_search_patterns);

let mybook = read_file_to_string();

println!("my rust ruuuuuun");

let my_values = extract_values_async(idiom_search_patterns, mybook);

//let vec = extract_idiom_async(idiom_search_patterns, mybook);

//let vec = extract_values_async(idiom_search_patterns, mybook);
}

fn extract_values_async(values_regex: [Option<Regex>; 8337], input_str: String) -> Vec<String> {

 let array_of_ranges:[[usize; 2]; 8] = [[0, 1042],[1043, 2084],[2085, 3126],[3127, 4168],[4169, 5210],[5211,6252],[6253, 7294],[7295, 8336]];
 let mut async_protected_accum = Arc::new(Mutex::new(Vec::<String>::new()));
 let accum = async_protected_accum.clone();
                                                  
   thread::scope(|s| {

    for (index, ranges) in array_of_ranges.iter().enumerate() {
        s.spawn(|| {
            let start_index = ranges[0];
            let end_index = ranges[1];
            let result_partition = do_part_work(start_index,end_index,&mut values_regex.clone(),&mut input_str.clone());
            accum.lock().unwrap().extend(result_partition);

        });
    }
    println!("hello from the main thread");
});


print!("external to scoup scope ");

println!("total array count = {}", accum.lock().unwrap().len());

    

        //заглушка
    let return_value: Vec<String> = Vec::new();

   return return_value;
}

fn do_part_work(start_index:usize, end_index:usize, values_regex: &mut [Option<Regex>; 8337], input_str: &mut String ) -> Vec<String> {

   let mut result_array: Vec<String> = Vec::new();

    for i in start_index..end_index {
        let srch_regex = values_regex[i].clone();
        let captures = srch_regex.clone().unwrap().captures(input_str.as_str());
        if let Some(result) = captures {
        let str_value = result.get(0).unwrap().as_str().to_string();
        result_array.push(str_value);
    } }
    println!("task job finished"); 
    return  result_array;
}


fn extract_idiom(idioms_regex: [Option<Regex>; 8337]) -> Vec<String> {

    let mybook = read_file_to_string();
    let mybook_str = mybook.as_str();
    let mut my_vec = Vec::new();

     for (index, srch_regex) in idioms_regex.iter().enumerate() {
      let captures = srch_regex.clone().unwrap().captures(&mybook_str);

        if let Some(result) = captures {
             let str_value = result.get(0).unwrap().as_str();
                my_vec.push(str_value.to_string());
        }           
    }

    println!("captured idioms count: {}", my_vec.len());
    println!("Job finished");
    return my_vec;
}

fn create_regex_idioms() -> [Option<Regex>; 8337] {

    let idioms = idioms_struct::get_idioms();
    let mut regex_array = [NONE; 8337];

       for (index, idiom) in idioms.iter().enumerate() {
        let regex_value = string_to_regex_converter::build_regex(idiom);
         regex_array[index] = Some(regex_value.unwrap());
    }
    return regex_array;
}

fn read_file_to_string() -> String {

let book = fs::read_to_string("/Users/danil/Downloads/Developer/RustExpririmentsFolder/rust_hello_world_proj/src/dune1-6.txt");
return book.unwrap()
}

