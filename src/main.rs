use to_do_list::{get_user_input, show_all_lists, open_to_do_list, modify_to_do_list, create_to_do_list};

fn main() {
    println!("Welcome to your To-Do Lists.");
    'main: loop {
        println!("\nPlease make a selection:\n1: Examine existing lists\n2: Create a new list\n3: View/Update an existing list\n4: Delete list\n5: Exit");
        let input = get_user_input();
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };       
        if input == 1 {
            show_all_lists();
        }
        if input == 2 {
            create_to_do_list();
        }
        if input == 3 {
            'list_selection: loop {
                println!("Please enter the name of the list you would like to open");
                println!("Or enter 'cancel' to return");
                show_all_lists();
                let input  = get_user_input();
                if input.to_lowercase().trim().eq("cancel") {
                    break 'list_selection;
                }
                if let Ok(selected_list) = open_to_do_list(input.trim()) {
                    modify_to_do_list(selected_list);           
                } else if let Err(e) = open_to_do_list(input.trim()) {
                    println!("{}", e);
                    continue;
                }                
            }
        }
        if input == 5 {
            break 'main;
        }
    }
    println!("The program ended.\nPress enter to close the terminal");
    let _ = get_user_input();     
}
