use std::collections::HashMap;

/// Converts a HashMap into a Vector. The Key-Value pair will be stored as a tuple.
/// In addition, the vector will be sorted alphabetically by the key values.
/// The function expects the HashMap to use a String as key. The value may be any object
/// that is not a reference.
/// 
/// # Arguments
/// * hash_list: &HashMap<String,T> - HashMap to be sorted and converted
/// 
/// # Returns
/// * `Vec<(&String, &T)>`: The sorted Vector representation of the input HashMap
pub fn sort_list <T> (hash_list: &HashMap<String,T>) -> Vec<(&String, &T)> {
    let mut output: Vec<_> = hash_list.iter().collect();
    output.sort_by(|x,y| x.0.cmp(y.0));
    output
}