use std::io;

pub struct MenuHandler<'a> {
    pub query: String,
    pub options: &'a [String],
}

impl<'a> MenuHandler<'a> {
    pub fn new(query: String, options: &'a [String]) -> MenuHandler<'a> {
        MenuHandler { query, options }
    }

    pub fn run(&self) -> String {
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
}
