use crate::list_items::enums::{Priority, ToDoSelectionError};
use crate::utils::functions::{sort_list};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::{write, File};
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

/// Representation of a single to-do list item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// Name of the item
    name: String,
    /// Description of the item
    description: String,
    /// Priority to the action (high/medium/low)
    priority: Priority,
    /// Date when the item was created
    creation_date: NaiveDate,
    /// Optional due date for the item
    due_date: Option<NaiveDate>,
    /// Flag to mark if an item was completed
    completed: bool,
}

impl Item {
    /// Constructor function for a new `Item`. Every Item will be created as non-completed.
    /// The creation date is always the day when the function was called.
    /// The due_date_ymd parameter is optional and can be used to assign a due date to the Item.
    /// A Some variant is expected to use a Tuple with 3 numeric values presenting year, month, day, in this order.
    /// If an invalid value is used, the function will ignore it and print a message in the log.
    /// 
    /// # Arguments
    /// * name : &str - Name of the Item
    /// * description : &str - Item description
    /// * priority : &str - Item priority
    /// * due_date_ymd : Option<(i32, u32, u32)> - Item due date (optional)
    /// 
    /// # Returns
    /// * `Item`: A new instance of an Item 
    fn new(name: &str, description: &str, priority: &str, due_date_ymd: Option<(i32, u32, u32)>) -> Self {
        // Process the optional due date parameter
        let mut due_date: Option<NaiveDate> = None;
        if let Some(ymd) = due_date_ymd {
            if let Some(assigned_due_date) = NaiveDate::from_ymd_opt(ymd.0, ymd.1, ymd.2) {
                due_date = Some(assigned_due_date);
            } else {
                println!("The submitted values for year {}, month {}, and day {} did not return a valid date", ymd.0, ymd.1, ymd.2);
            }
        }

        Item { 
            name: name.to_string(), 
            description: description.to_string(), 
            priority: Priority::from_str(priority), 
            creation_date: Local::now().date_naive(), 
            due_date, 
            completed: false 
        }
    }
    /// Creates a reference to the `Item` name.
    /// 
    /// # Returns
    /// * `&str`: Item name
    pub fn get_name(&self) -> &str {
        &self.name
    }   

    /// Creates a reference to the `Item` description.
    /// 
    /// # Returns
    /// * `&str`: Item description    
    pub fn get_description(&self) -> &str {
        &self.description
    }       

    /// Creates a reference to the `Item` priority.
    /// 
    /// # Returns
    /// * `&Priority`: Item priority     
    pub fn get_priority(&self) -> &Priority {
        &self.priority
    }        

    /// Creates a reference to the `Item` creation_date.
    /// 
    /// # Returns
    /// * `&NaiveDate`: Item creation date      
    pub fn get_creation_date(&self) -> &NaiveDate {
        &self.creation_date
    }          

    /// Creates a reference to the `Item` due_date.
    /// 
    /// # Returns
    /// * `&Option<NaiveDate>`: Item due date (when applicable)       
    pub fn get_due_date(&self) -> &Option<NaiveDate >{
        &self.due_date
    }           

    /// Checks whether the Item is overdue (i.e., the due date lies in the past).
    /// 
    /// # Returns
    /// * `bool`: Is `true` if the due date passed   
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            due_date < Local::now().date_naive()
        } else {
            false
        }
    }   

    /// Indicates whether the item has been completed yet.
    /// 
    /// # Returns
    /// * `bool`: Is true if the `Item` has been completed        
    pub fn is_completed(&self) -> bool {
        self.completed
    }      

    /// Change the `Item` description.
    /// 
    /// # Arguments
    /// * new_description : `&str` - New value for the description field
    fn update_description(&mut self, new_description: &str) {
        self.description = String::from(new_description);
    }

    /// Change the `Item` priority. The method accepts a `&str` and
    /// converts it into a `Priority`.
    /// 
    /// # Arguments
    /// * new_priority : `&str` - New value for the priority field    
    fn update_priority(&mut self, new_priority: &str) {
        self.priority = Priority::from_str(new_priority);
    }

    /// Change the `Item` due_date.
    /// If an invalid date is submitted, the method will not update the Item and print a message in the log.
    /// 
    /// # Arguments
    /// * ymd : (i32, u32, u32) - Updated due_date of the Item (year, month, day)    
    fn update_due_date(&mut self, ymd: (i32, u32, u32)) {
        if let Some(due_date) = NaiveDate::from_ymd_opt(ymd.0, ymd.1, ymd.2) {
            self.due_date = Some(due_date)
        } else {
            println!("The submitted values for year {}, month {}, and day {} did not return a valid date", ymd.0, ymd.1, ymd.2);
        }
    }

    /// Mark an `Item` as completed.  
    fn complete_item(&mut self) {
        self.completed = true
    }

    /// Mark an `Item` as not completed. 
    fn open_item(&mut self) {
        self.completed = false
    }    

}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(due_date) = self.due_date {
            write!(f, "Name: {}\tDescription: {}\tPriority: {}\tCreation Date:{}\tDue Date:{}", self.name, self.description, self.priority, self.creation_date, due_date)
        } else {
            write!(f, "Name: {}\tDescription: {}\tPriority: {}\tCreation Date:{}\tDue Date: NA", self.name, self.description, self.priority, self.creation_date)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Representation of a to-do list with multiple items.
pub struct ToDoList {
    /// Name of the to-do list
    name: String,
    /// Description of the to-do list
    description: String,
    /// Collection of all `Item` structs within the to-do list
    items: HashMap<String, Item>,
}

impl ToDoList {
    /// Constructor function for a new, empty `ToDoList`.
    /// The function assigns a name and a description to the new list.
    /// 
    /// # Arguments
    /// * name : &str - Name of the list
    /// * description : &str - List description
    /// 
    /// # Returns
    /// * `ToDoList`: A new instance of a to-do list   
    pub fn create_to_do_list(list_name: &str, list_description: &str) -> Self {
        ToDoList { name: list_name.to_string(), description: list_description.to_string(), items: HashMap::new() }
    }

    /// Creates a new `Item` and automatically stores it in the `ToDoList`.
    /// By default, the method will check whether the list already contains in Item with 
    /// the same name as the submitted one. If so, it will not create the new Item and instead
    /// return an error.
    /// Submitting the method with `replace` as `true` allows it to replace the existing version.
    /// 
    /// # Arguments
    /// * name : &str - Name of the Item
    /// * description : &str - Item description
    /// * priority : &str - Item priority
    /// * replace: bool - Set to true to replace an existing Item
    /// * due_date_ymd : Option<(i32, u32, u32)> - Item due date (optional)
    /// 
    /// # Errors
    /// * `ToDoSelectionError::ToDoAlreadyPresent`: An Item with the same name already exists in the ToDoList and replace was set to false.  
    pub fn create_item(&mut self, name: &str, description: &str, priority: &str, due_date_ymd: Option<(i32, u32, u32)>, replace: bool) -> Result<(), ToDoSelectionError> {
        if !self.list_contains_item(name) || replace {
            self.items.insert(name.to_string(), Item::new(name, description, priority, due_date_ymd));
            Ok(())
        } else {
            Err(ToDoSelectionError::ToDoAlreadyPresent)
        }
    }

    /// Creates a reference to the `ToDoList` name.
    /// 
    /// # Returns
    /// * `&str`: ToDoList name    
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Creates a reference to the `ToDoList` description.
    /// 
    /// # Returns
    /// * `&str`: ToDoList description      
    pub fn get_description(&self) -> &str {
        &self.description
    }    

    /// Checks whether the item HashMap contains an Item with the submitted name
    /// 
    /// # Arguments
    /// * item_name : &str - Name of the Item 
    /// 
    /// # Returns
    /// * `bool`: is `true` if the Item exists    
    fn list_contains_item(&self, item_name: &str) -> bool {
        self.items.contains_key(item_name)
    }

    /// Returns an immutable reference to an `Item` stored in the items field.
    /// 
    /// # Arguments
    /// * item_name : &str - Name of the Item 
    /// 
    /// # Returns
    /// * `&Item`: Reference to the Item
    /// 
    /// # Errors
    /// * `ToDoSelectionError::ToDoNotFound`: No Item with the submitted name exists in the `item` field. 
    pub fn get_item_ref(&self, item_name: &str) -> Result<&Item, ToDoSelectionError> {
        if self.list_contains_item(item_name) {
            Ok(self.items.get(item_name).unwrap())
        } else {
            Err(ToDoSelectionError::ToDoNotFound)
        }        
    }

    /// Permanently deletes an Item from the item HashMap if it exists. If not, the method returns an error instead.
    /// 
    /// # Arguments
    /// * item_name : &str - Name of the Item 
    /// 
    /// # Errors
    /// * `ToDoSelectionError::ToDoNotFound`: No Item with the submitted name exists in the `item` field.  
    pub fn delete_item(&mut self, item_name: &str) -> Result<(), ToDoSelectionError> {
        if self.list_contains_item(item_name) {
            self.items.remove(item_name);
            Ok(())
        } else {
            Err(ToDoSelectionError::ToDoNotFound)
        }
    }

    /// Change the description of an Item in the item HashMap if it exists. If not, the method returns an error instead.
    /// 
    /// # Arguments
    /// * item_name : &str - Name of the Item 
    /// * new_description : &str - Updated description of the Item
    /// 
    /// # Errors
    /// * `ToDoSelectionError::ToDoNotFound`: No Item with the submitted name exists in the `item` field.      
    pub fn update_item_description(&mut self, item_name: &str, new_description: &str) -> Result<(), ToDoSelectionError> {
        if let Some(item) = self.items.get_mut(item_name) {
            item.update_description(new_description);
            Ok(())
        } else {
            Err(ToDoSelectionError::ToDoNotFound)
        }
    }

    /// Change the priority of an Item in the item HashMap if it exists. If not, the method returns an error instead.
    /// 
    /// # Arguments
    /// * item_name : &str - Name of the Item 
    /// * new_priority : &str - Updated Priority of the Item
    /// 
    /// # Errors
    /// * `ToDoSelectionError::ToDoNotFound`: No Item with the submitted name exists in the `item` field.     
    pub fn update_item_priority(&mut self, item_name: &str, new_priority: &str) -> Result<(), ToDoSelectionError> {
        if let Some(item) = self.items.get_mut(item_name) {
            item.update_priority(new_priority);
            Ok(())
        } else {
            Err(ToDoSelectionError::ToDoNotFound)
        }
    }

    /// Change the due date of an Item in the item HashMap if it exists. If not, the method returns an error instead.
    /// If an invalid date is submitted, the method will not update the Item and print a message in the log.
    /// 
    /// # Arguments
    /// * item_name : &str - Name of the Item 
    /// * ymd : (i32, u32, u32) - Updated due_date of the Item (year, month, day)
    /// 
    /// # Errors
    /// * `ToDoSelectionError::ToDoNotFound`: No Item with the submitted name exists in the `item` field.     
    pub fn update_item_due_date(&mut self, item_name: &str, ymd: (i32, u32, u32)) -> Result<(), ToDoSelectionError> {
        if let Some(item) = self.items.get_mut(item_name) {
            item.update_due_date(ymd);
            Ok(())
        } else {
            Err(ToDoSelectionError::ToDoNotFound)
        }
    }    

    /// Mark a list Item as completed if it exists. If not, the method returns an error instead.
    /// 
    /// # Arguments
    /// * item_name : &str - Name of the Item 
    /// 
    /// # Errors
    /// * `ToDoSelectionError::ToDoNotFound`: No Item with the submitted name exists in the `item` field.    
    pub fn close_list_item(&mut self, item_name: &str) -> Result<(), ToDoSelectionError> {
        if let Some(item) = self.items.get_mut(item_name) {
            item.complete_item();
            Ok(())
        } else {
            Err(ToDoSelectionError::ToDoNotFound)
        }        
    }

    /// Mark a list Item as uncompleted if it exists. If not, the method returns an error instead.
    /// 
    /// # Arguments
    /// * item_name : &str - Name of the Item 
    /// 
    /// # Errors
    /// * `ToDoSelectionError::ToDoNotFound`: No Item with the submitted name exists in the `item` field.     
    pub fn open_list_item(&mut self, item_name: &str) -> Result<(), ToDoSelectionError> {
        if let Some(item) = self.items.get_mut(item_name) {
            item.open_item();
            Ok(())
        } else {
            Err(ToDoSelectionError::ToDoNotFound)
        }
    }

    /// Creates a new version of the Item list in which only
    /// open Items are being kept.
    /// 
    /// # Returns
    /// * `HashMap<String, Item>`: Filtered item list     
    pub fn filter_open_items(&self) -> HashMap<String, Item> {
        let mut output: HashMap<String, Item> = HashMap::new();
        for item in &self.items {
            if !item.1.is_completed() {
                output.insert(item.0.clone(), item.1.clone());
            }
        }        
        output
    }

    /// Creates a new version of the Item list in which only
    /// overdue and open Items are being kept.
    /// 
    /// # Returns
    /// * `HashMap<String, Item>`: Filtered item list
    pub fn filter_overdue_items(&self) -> HashMap<String, Item> {
        let mut output: HashMap<String, Item> = HashMap::new();
        for item in &self.items {
            if !item.1.is_completed() && item.1.is_overdue() {
                output.insert(item.0.clone(), item.1.clone());
            }
        }
        output
    }

    /// Converts an item HashMap into a Vector in which the original entries are
    /// stored in tuples. The items in the resulting vector are sorted alphabetically
    /// based on the Item names.
    /// 
    /// # Returns
    /// * `Vec<(&String, &Item)>`: Sorted Vector representing the inserted HashMap      
    pub fn list_all_items (hash_map: &HashMap<String, Item>) -> Vec<(&String, &Item)> {
        sort_list(hash_map)
    }         

    /// Prints every Item in the ToDoList to the console.
    pub fn display_all_items(&self) {
        let list = Self::list_all_items(&self.items);
        for item in list {
            println!("\n{}", item.1);
        }
    }

    /// Prints every non-completed Item in the ToDoList to the console.
    pub fn display_all_open_items(&self) {
        let filtered_list = self.filter_open_items();
        let list = Self::list_all_items(&filtered_list);
        for item in list {
            println!("\n{}", item.1);
        }
    }    

    /// Prints every overdue Item in the ToDoList to the console.
    pub fn display_all_overdue_items(&self) {
        let filtered_list = self.filter_overdue_items();
        let list = Self::list_all_items(&filtered_list);
        for item in list {
            println!("\n{}", item.1);
        }
    }

    /// Permanently save the `ToDoList` and all its Items to a JSON file. 
    /// The file will be generated in the ./lists folder.
    /// 
    /// # Panics
    /// The method will panic if the ToDoList cannot be converted to a JSON file or
    /// if the expected lists folder cannot be found.
    pub fn save_to_do_list(&self) {
        let json = serde_json::to_string_pretty(self).expect("JSON serialize error");
        let path = format!("./lists/{}.json", self.name);
        write(path, json).expect("Unable to write file");
    }

    /// Load an existing `ToDoList` and its Items from an JSON file. 
    /// The JSON file is expected to be present in the ./lists folder.
    /// 
    /// # Panics
    /// The function will panic if the ToDoList cannot be loaded from JSON file or
    /// if the expected lists folder cannot be found.    
    pub fn load_to_do_list(list_name: &str) -> Self {
        let path = if list_name.to_lowercase().contains(".") {
            format!("./lists/{}", list_name)
        } else {
            format!("./lists/{}.json", list_name)
        };
        let file = File::open(&path).expect("Could not open the file");
        serde_json::from_reader(file).expect("Could not process JSON file")
    }    

}

impl Display for ToDoList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Name: {}\tDescription: {}", self.name, self.description)
    }
}
