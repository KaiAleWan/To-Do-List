use std::collections::HashMap;

pub fn sort_list <T> (hash_list: &HashMap<String,T>) -> Vec<(&String, &T)> {
        let mut output: Vec<_> = hash_list.iter().collect();
        output.sort_by(|x,y| x.0.cmp(&y.0));
        output
}