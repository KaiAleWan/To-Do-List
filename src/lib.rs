mod list_items;
mod utils;
use std::path::Path;
use std::fs::read_dir;
use std::io;
use chrono::NaiveDate;
use crate::list_items::structs::ToDoList;

/// Retrieves user input from the terminal and stores it inside a String value.
/// 
/// # Returns
/// * `String`: The trimmed user input that was submitted via the terminal.
/// 
/// # Panics
/// The function panics if the io module failed to read the terminal input line
pub fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

/// Uses user input to create a tuple that can be used when a date field should be populated.
/// The function asks the user to input 3 integer values. The first represents a year,
/// the second a month, and the third a day. 
/// At the end, the function validates whether the submitted values can be used to create
/// a valid NaiveDate struct.
/// If not, the function will return to its start and loop again.
/// 
/// # Returns
/// * `(i32, u32, u32)`: A tuple that represents, year, month, and day
pub fn enter_date_value() -> (i32, u32, u32) {
    loop {
        let mut ymd: (i32, u32, u32) = (0,0,0);
        'year: loop {
            println!("Please enter a numeric year value");
            let input = get_user_input();
            let input: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
            };   
            ymd.0 = input;
            break 'year;
        }
        'month: loop {
            println!("Please enter a numeric month value");
            let input = get_user_input();
            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
            };   
            ymd.1 = input;
            break 'month;
        }    
        'day: loop {
            println!("Please enter a numeric day value");
            let input = get_user_input();
            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
            };   
            ymd.2 = input;
            break 'day;
        }
        // Check whether the input was valid
        if NaiveDate::from_ymd_opt(ymd.0, ymd.1, ymd.2).is_some() {
            return ymd;
        } else {
            println!("The submitted values could not be converted into a date. Please enter valid integers.");
        }
    }
}

/// Lists all files stored in the ./lists folder. 
/// The function assumes that only list structs are stored in this location.#
/// 
/// # Returns
/// * `Vec<String>`: A Vector containing the names of all files in the lists folder
fn summarize_list_files() -> Vec<String> {
    // The path is expected to lead to the "./lists" folder
    let path = Path::new("./lists");
    let mut file_list: Vec<String> = vec![];
    match read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(file) = entry {
                    file_list.push(file.file_name().into_string().expect("Could not convert OsString to String"));
                } else {
                    println!("A file could not be read: {:?}", entry);
                }
            }
        },
        Err(e) => println!("The directory could not be read: {}",e),
    }
    file_list
}

/// Displays the names of files located in the ./lists folder.
/// The names are directly printed to the standard output.
pub fn show_all_lists() {
    let file_list = summarize_list_files();
    if !file_list.is_empty() {
        println!("Known to-do lists:");
        for file_name in file_list {
            println!("\t- {}", file_name);
        }
    } else {
        println!("No to-do list was found in ./lists");
    }
}

/// Checks whether the ./lists folder contains a list with a specific name.
/// The function checks the list name with and without the .json extension.
/// 
/// # Arguments
/// * list_name : &str - Name of the ToDoList 
/// 
/// # Returns
/// * `bool`: Is `true` if a ToDoList with the same name exists in ./lists
fn list_file_exists(list_name: &str) -> bool {
    let list_name = list_name.to_string();
    let file_name = format!("{}.json", &list_name);
    summarize_list_files().contains(&list_name) || summarize_list_files().contains(&file_name)
}

/// Deserializes a list.json file and loads it into a ToDoList struct.
/// The function processes the name of the list with or withouth the 
/// .json extension.
/// 
/// # Arguments
/// * list_name: &str - Name of the list (file) to load
/// 
/// # Returns
/// * `ToDoList` - The deserialized version of the selected list
/// 
/// # Errors
/// * Returns an error message if the selected list file does not exist
pub fn open_to_do_list(list_name: &str) -> Result<ToDoList, String> {
    let file_name = if list_name.to_lowercase().contains(".") {
        list_name.to_string()
    } else {
        format!("{}.json", list_name)
    };   
    if list_file_exists(&file_name) {
        Ok(ToDoList::load_to_do_list(&file_name) )
    } else {
        Err(format!("No to-list with the submitted name {} was found", file_name))
    }
}

/// Creates a new ToDoList and store it as a .json file in the lists folder.
/// The function checks whether a list with the suggested name already exists
/// and will ask the user for confirmation if an existing one should be replaced.
pub fn create_to_do_list() {
    println!("Enter the name of the list");
    let list_name = get_user_input();
    println!("Enter the description of the list");
    let list_description = get_user_input();    
    if !list_file_exists(&list_name) {
        ToDoList::new(&list_name, &list_description).save_to_do_list();
    } else {
        println!("A list with the name {} already exists. Enter 'Y' to replace it. \nWarning: All items will be removed.", &list_name); 
        let user_choice = get_user_input();   
        if user_choice.to_lowercase().trim().eq("y") {
            ToDoList::new(&list_name, &list_description).save_to_do_list();
        }
    }
}

/// Attempts to create a new Item witin a ToDoList and saves it in the respective
/// .json file.
/// The function checks whether an Item with the same name already exists and will
/// ask for permission to overwrite it if so.
/// Note that the change is saved directly and cannot be reversed later.
/// 
/// # Arguments
/// * list :&mut ToDoList - Mutable reference to the ToDoList in which the Item will be created
fn create_new_item(list :&mut ToDoList) {
    println!("Enter the name of the item");
    let item_name = get_user_input();
    println!("Enter the description of the item");
    let item_description = get_user_input();
    println!("Define the priority of the item (Low, Medium, or High)");
    let item_priority = get_user_input();
    println!("Enter 'Y' if you would like to assign a due date");
    let item_due_date = if get_user_input().to_lowercase().trim().eq("y") {
        Some(enter_date_value())
    } else {
        None
    };
    let mut replace = false;
    if list.list_contains_item(&item_name) {
        println!("An item with the name {} already exists. Enter 'Y' to replace it.", item_name);
        if get_user_input().to_lowercase().trim().eq("y") {
            replace = true;
        }
    }
    if let Err(e) = list.create_item(&item_name, &item_description, &item_priority, item_due_date, replace) {
        println!("The item was not created: {}", e);
    } else {
        ToDoList::save_to_do_list(list);
    }
}

/// Uses user input to select and modify an Item in the open ToDoList.
/// The user can choose to set any of the fields in the selected Item and
/// is able to save the changes inside the respective .json file.
/// Note that without using the 'save' option, the changes will be reversed
/// as soon as the Item is closed.
fn select_and_modify_list(list :&mut ToDoList) {
    // Loop used to select a list Item
    'list_modification: loop {
        println!("Choose an Item to modify or submit 'cancel' to return");
        println!("Current list:\n{}", &list);
        list.display_all_items();
        let item_name = get_user_input();
        if !list.list_contains_item(&item_name) && !item_name.to_lowercase().trim().eq("cancel") {
            println!("The list does not contain an Item with name {}. Please submit another value.", &item_name);
            continue;
        }
        if item_name.to_lowercase().trim().eq("cancel") {
            break 'list_modification;
        }
        // Loop used to pick the desired modification in the selected Item
        'item_modification: loop {
            println!("Selected Item:\n{}", list.get_item_ref(&item_name).expect("The list Item does not exist"));
            println!("Choose a property to modify");
            println!("1: Description\n2: Due Date\n3: Priority\n4: Complete item\n5: Open item\n6: Save changes\n7: Cancel");    
            let input = get_user_input();
            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
            };     
            if input == 1 {
                println!("Enter the new description");
                let new_description = get_user_input();
                list.update_item_description(&item_name, &new_description).expect("The list Item does not exist");
                continue;
            }
            if input == 2 {
                println!("Enter the new due date as year, month, day");
                let new_due_date = enter_date_value();
                list.update_item_due_date(&item_name, new_due_date).expect("The list Item does not exist");
                continue;
            }
            if input == 3 {
                println!("Enter the new priority (Low, Medium, High)");
                let new_priority = get_user_input();
                list.update_item_priority(&item_name, &new_priority).expect("The list Item does not exist");
            }
            if input == 4 {
                // Marks the Item as completed
                list.close_list_item(&item_name).expect("The list Item does not exist");
            }
            if input == 5 {
                // Marks the Item as non-completed
                list.open_list_item(&item_name).expect("The list Item does not exist");
            }                
            if input == 6 {
                ToDoList::save_to_do_list(list);
            }
            if input == 7 {
                break 'item_modification;
            }                    
        }
    }
}

/// Permanently deletes a selected Item from the open ToDoList.
/// The function will ask for user input to select the Item and then asks 
/// for a final confirmation before the Item is deleted. 
/// Note that the change is saved directly and cannot be reversed later.
fn delete_list_item(list: &mut ToDoList) {
    'item_deletion: loop {
        println!("Current list:\n{}", &list);
        list.display_all_items();                
        println!("Select an item to delete or 'cancel' to abort.");
        let delete_selection = get_user_input();
        if delete_selection.to_lowercase().trim().eq("cancel") {
            break 'item_deletion;
        }
        if !list.list_contains_item(&delete_selection) {
            println!("The selected item does not exist");
            continue;
        }
        println!("Item {} will be deleted permanently. Enter 'Y' to confirm", &delete_selection);
        let delete_confirmation = get_user_input();
        if delete_confirmation.to_lowercase().trim().eq("y") {
            list.delete_item(&delete_selection).expect("The list Item does not exist");
            ToDoList::save_to_do_list(list);
            break 'item_deletion;
        }
    }
}

/// Opens the sub-menu to modify the selected ToDoList. 
/// The menu asks for user input to add, delete, or alter Items in the selected list. 
/// The changes are then saved to their respective .json file to make them permanent.
pub fn modify_to_do_list(mut list: ToDoList) {
    'main: loop {
        println!("Current list:\n{}", &list);
        list.display_all_items();
        println!("Choose an action:\n1: Create new Item\n2: Modify existing Item\n3: Delete item\n4: Cancel");
        let input = get_user_input();
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };  
        if input == 1 {
            create_new_item(&mut list);
        }
        if input == 2 {
            select_and_modify_list(&mut list);
        }
        if input == 3 {
            delete_list_item(&mut list);
        }
        if input == 4 {
            break 'main;
        }
    }
}

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