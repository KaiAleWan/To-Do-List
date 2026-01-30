mod list_items;
mod utils;

// Section for unit tests
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::utils::functions::*;

    #[test]
    fn it_sorts_hashmap() {
        let mut map : HashMap<String, u32> = HashMap::new();
        map.insert("d".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.insert("a".to_string(), 1);

        let vec = sort_list(&map);

        assert_eq!(vec[0].0, "a");
        assert_eq!(vec[1].0, "b");
        assert_eq!(vec[2].0, "d");
    }
}