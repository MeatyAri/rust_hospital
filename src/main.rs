mod cli_handler;
mod auth;
mod db_handler;
mod data_structures;

use db_handler::Database;
use auth::Auth;

fn main() {
    let mut db = Database::load_from_file("users.db").unwrap_or(Database::new());
    let mut auth = Auth::new(&mut db);
    
    let selected = cli_handler::main_menu();

    auth.authenticate(selected);
    println!("Logged in as: {:?}", auth.user);
}
