mod list_items;
mod utils;

// Section for unit tests
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::utils::functions::*;
    use crate::list_items::structs::*;

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

    #[test]
    fn it_loads_to_do_list() {
        let test_list = ToDoList::load_to_do_list("example");
        println!("All Items");
        test_list.display_all_items();
        println!("All open Items");
        test_list.display_all_open_items();
        println!("All Overdue Items");
        test_list.display_all_overdue_items();
    }

    #[test]    
    fn item_can_be_modified() {
        let mut test_list = ToDoList::load_to_do_list("example");
        // Original description value
        let item_ref_before = test_list.get_item_ref("test1").unwrap();
        assert_eq!(item_ref_before.get_description(), "First test Item");
        // Modify the description value
        test_list.update_item_description("test1", "Modified Description").unwrap();
        let item_ref_after = test_list.get_item_ref("test1").unwrap();
        assert_eq!(item_ref_after.get_description(), "Modified Description");
    }
}