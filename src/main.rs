mod cli_handler;
use cli_handler::MenuHandler;


fn main() {
    let options = [
        "Option 1".to_string(),
        "Option 2".to_string(),
        "Option 3".to_string(),
        "Option 4".to_string(),
    ];
    
    let menu = MenuHandler::new("Choose an option:".to_string(), &options);
    let selected = menu.run();
    println!("You selected: {}", selected);
}
