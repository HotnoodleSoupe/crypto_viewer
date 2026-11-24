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


// Abgrage der Transaktinsdaten 
pub fn add_transaction(conn: &Connection) -> Result<(), Box<dyn std::error::Error>>{
    let date = read_input("Date (e.g. 2026-11-24): ")?;
    let tx_type = read_input("TX Type (BUY/SELL): ")?.to_uppercase();
    let exchange = read_input("Exchange (e.g. Kraken):")?;
    let amount = read_input("Amount (e.g. 0.05):")?.parse::<f64>()?;
    let invested = read_input("Invested in EUR (e.g. 1500.50):")?.parse::<f64>()?;
    let kyc_input = read_input("KYC (y/n):")?;
    let kyc = kyc_input.to_lowercase() == "j" || kyc_input.to_lowercase() == "y";
    let note = read_input("Note (optional):")?;


    // Schreiben in die SQL 

    conn.execute("INSERT INTO transactions (date, tx_type, exchange, amount, invested, kyc, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![date, tx_type, exchange, amount, invested, kyc, note],
)?;
Ok(())

}




