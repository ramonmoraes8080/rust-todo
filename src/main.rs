use std::io;
use std::io::Write;
use std::vec::Vec;

fn read_todo(todos: &mut Vec::<String>) -> io::Result<()> {
    let mut buffer = String::new();

    print!("new todo > ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer)?;
    todos.push(buffer.clone().trim().to_string());

    Ok(())
}

fn show_todo(todos: &mut Vec::<String>) {
    println!("");
    for (i, item) in todos.iter().enumerate() {
        println!("\t{i} - {item}");
    }
    println!("");
}

fn main() -> io::Result<()> {
    let mut todos = Vec::<String>::new();
    let mut buffer = String::new();

    loop {
        // Collecting View Selection
        println!("1 - Print List\n2 - Add new entry\n3 - quit");
        io::stdin().read_line(&mut buffer)?;
        let menu_opt = buffer.as_str().trim();
        match menu_opt {
            "1" => show_todo(&mut todos),
            "2" => read_todo(&mut todos)?,
            "3" => {
                return Ok(())
            },
            _ => println!("no option binded to {menu_opt:?}")
        }
        buffer.clear();
    }

    // Ok(())

}
