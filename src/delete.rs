use rusqlite::{Connection, params};
use std::io::{self, Write};

// Standart Weg für eine Eingabe im CLI 
fn read_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}



pub fn delete_transaction(conn: &Connection) -> Result<(), Box<dyn std::error::Error>>{

    let id_input = read_input("ID to delete: ")?;
    let id = id_input.parse::<i64>()?;
    
    conn.execute("DELETE FROM transactions WHERE id = ?1", params![id])?;
Ok(())

}



