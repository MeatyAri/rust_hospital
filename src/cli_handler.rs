use std::io;

pub struct MenuHandler<'a> {
    pub query: String,
    pub options: &'a [String],
}

impl<'a> MenuHandler<'a> {
    pub fn new(query: String, options: &'a [String]) -> MenuHandler<'a> {
        MenuHandler { query, options }
    }

    fn get_selected_option(&self) -> String {
        loop {
            println!("{}", self.query);
            for (i, option) in self.options.iter().enumerate() {
                println!("{}: {}", i + 1, option);
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().parse::<usize>();
            match input {
                Ok(n) if n > 0 && n <= self.options.len() => {
                    return self.options[n - 1].clone();
                }
                _ => println!("Invalid input"),
            }
        }
    }

    pub fn get_input_string(query: String) -> String {
        println!("{}", query);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn run(&self) -> String {
        if self.options.is_empty() {
            Self::get_input_string(self.query.clone())
        } else {
            self.get_selected_option()
        }
    }
}

pub fn main_menu() -> String {
    let options = ["Login".to_string(), "Sign Up".to_string(), "Exit".to_string()];
    let menu = MenuHandler::new("What would you like to do?".to_string(), &options);
    let selected = menu.run();
    selected
}


