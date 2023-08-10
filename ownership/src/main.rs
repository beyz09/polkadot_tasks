fn concatenate_strings(slice1: &str, slice2: &str) -> String {
    let mut result = String::new();
    result.push_str(slice1);
    result.push_str(slice2);
    result
}

fn main() {
    let string1 = "Hello, ";
    let string2 = "Beyz!";
    
    let concatenated_string = concatenate_strings(string1, string2);

    println!("{}", concatenated_string);
}