use crate::list_items::enums::{Priority, ToDoSelectionError};
use std::collections::HashMap;

/// Representation of a single to-do list item.
struct Item {
    /// Name of the item
    name: String,
    /// Description of the item
    description: String,
    /// Priority to the action (high/medium/low)
    priority: Priority,
    /// Date when the item was created
    creation_date: String,
    /// Optional due date for the item
    due_date: Option<String>,
    /// Flag to mark if an item was completed
    completed: bool,
}

impl Item {

    fn new(name: &str, description: &str, priority: &str) -> Self {
        Item { 
            name: name.to_string(), 
            description: description.to_string(), 
            priority: Priority::from_str(priority), 
            creation_date: "".to_string(), 
            due_date: None, 
            completed: false 
        }
    }
    /// Creates a reference to the `Item` name.
    /// 
    /// # Returns
    /// * `&str`: Item name
    fn get_name(&self) -> &str {
        &self.name
    }   

    /// Creates a reference to the `Item` description.
    /// 
    /// # Returns
    /// * `&str`: Item description    
    fn get_description(&self) -> &str {
        &self.description
    }       

    /// Creates a reference to the `Item` priority.
    /// 
    /// # Returns
    /// * `&Priority`: Item priority     
    fn get_priority(&self) -> &Priority {
        &self.priority
    }        

    fn get_creation_date(&self) -> &str {
        &self.creation_date
    }          

    fn get_due_date(&self) -> &str {
        if let Some(date) = &self.due_date {
            date
        } else {
            ""
        }
    }              

    /// Indicates whether the item has been completed yet.
    /// 
    /// # Returns
    /// * `bool`: Is true if the `Item` has been completed        
    fn is_completed(&self) -> bool {
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

    /// Mark an `Item` as completed.  
    fn complete_item(&mut self) {
        self.completed = true
    }

    /// Mark an `Item` as not completed. 
    fn open_item(&mut self) {
        self.completed = false
    }    

}

pub struct ToDoList {
    name: String,
    items: HashMap<String, Item>,
}

impl ToDoList {

    pub fn create_to_do_list(list_name: &str) -> Self {
        ToDoList { name: list_name.to_string(), items: HashMap::new() }
    }

    pub fn create_item(&mut self, name: &str, description: &str, priority: &str, replace: bool) -> Result<(), ToDoSelectionError> {
        if !self.list_contains_item(name) || replace {
            self.items.insert(name.to_string(), Item::new(name, description, priority));
            Ok(())
        } else {
            Err(ToDoSelectionError::ToDoAlreadyPresent)
        }
    }

    pub fn list_all_open_items (&self) -> Vec<&Item> {
        let mut output: Vec<&Item> = vec![];
        for item in &self.items {
            if !item.1.is_completed() {
                output.push(item.1);
            }
        }
        output
    }

    pub fn list_contains_item(&self, item_name: &str) -> bool {
        self.items.contains_key(item_name)
    }

    pub fn delete_item(&mut self, item_name: &str) -> Result<(), ToDoSelectionError> {
        if self.list_contains_item(item_name) {
            self.items.remove(item_name);
            Ok(())
        } else {
            Err(ToDoSelectionError::ToDoNotFound)
        }
    }

}
