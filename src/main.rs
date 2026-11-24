use std::fs;
use rusqlite::{Connection, params};
use clap::Parser;
mod api;
mod import;
mod header;
mod new_entry;
mod delete;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    help_template = "{name} {version}\nAuthor: {author}\nLicense: MIT\n{about}\n\n{usage-heading} {usage}\n\n{all-args}"
)]

struct Args {
    /// Show short info about investments
    #[arg(short = 'i', long)]
    info: bool,

    /// Export function (coming soon)
    #[arg(short = 'e', long)]
    export: Option<String>,

    /// Import transactions from CSV
    #[arg(short = 'I', long)]
    import: Option<String>,

    /// Enter a new transaction
    #[arg(short = 'n')]
    new: bool,

    /// Delete a transaction entry
    #[arg(short = 'd')]
    delete: bool,
}

#[derive(Debug)]
struct Transaction {
    id: i32,
    date: String,
    tx_type: String,
    exchange: Option<String>,
    amount: String,
    invested: String,
    kyc: bool,
    note: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Erstellt einen Ordner im Home-Verzeichnis, absolut!
    let home = std::env::var("HOME")?;
    let db_dir = format!("{}/.crypto_viewer", home);
    fs::create_dir_all(&db_dir)?;
    let db_path = format!("{}/crypto.db", db_dir);

    header::print_header(); // Zeigt den Programm-Header

    // fragt die APIs ab
    let data_geko = match api::geko_api().await {
        Ok(data_geko) => data_geko,
        Err(e) => {
            println!("API-Fehler: {e}");
            return Ok(());
        }
    };
    // Zeigt Blocktime
    let block_time = match api::mempool_api().await {
        Ok(block_time) => block_time,
        Err(e) => {
            println!("API-Fehler: {e}");
            return Ok(());
        }
    };

    let conn = Connection::open(&db_path)?; // Öffnet die SQL-Datenbank

    conn.execute(
        // Erstellt eine Tabelle
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY,
            date TEXT NOT NULL,
            tx_type TEXT NOT NULL,
            exchange TEXT,
            amount TEXT NOT NULL,
            invested TEXT NOT NULL,
            kyc INTEGER NOT NULL,
            note TEXT
        )",
        (),
    )?;

    let args = Args::parse();

    if args.delete {
        delete::delete_transaction(&conn)?;

    } else if let Some(path) = args.import {

        let date = import::load_transactons(&path)?;

        for enter in date {
            conn.execute(
                "INSERT INTO transactions (date, tx_type, exchange, amount, invested, kyc, note)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    enter.date,
                    enter.tx_type,
                    enter.exchange,
                    enter.amount,
                    enter.invested,
                    enter.kyc,
                    enter.note,
                ],
            )?;
        }

        return Ok(());
    } else if let Some(path) = args.export {
        println!("Your entry {path}");
        println!("coming soon.");
    } else if args.new {
        new_entry::add_transaction(&conn)?;
    } else if args.info {
        println!("coming soon.")
    }


    let mut stmt = conn.prepare(
        "SELECT id, date, tx_type, exchange, amount, invested, kyc, note FROM transactions",
    )?;

    let tx_iter = stmt.query_map([], |row| {
        // Liest die Tabelle Zeile für Zeile aus und speichert es in tx_iter ab
        Ok(Transaction {
            id: row.get(0)?,
            date: row.get(1)?,
            tx_type: row.get(2)?,
            exchange: row.get(3)?,
            amount: row.get(4)?,
            invested: row.get(5)?,
            kyc: row.get(6)?,
            note: row.get(7)?,
        })
    })?;

    println!(
        // anzeige von Spaltennamen
        "{:<4} | {:<10} | {:<5} | {:<12} | {:>10} | {:>10} | {:<4} | {}",
        "ID", "Datum", "Typ", "Source", "Menge", "Preis EUR", "KYC", "Notiz"
    );
    println!("{}", "-".repeat(100)); // Erstellt die Trennlinie

    let mut sum_euro = 0.0;
    let mut sum_menge = 0.0;


    // Gibt den Inhalt der Tabelle aus
    for tx in tx_iter {
        
        let tx = tx?;
        println!(
            "{:<4} | {:<10} | {:<4} | {:<12} | {:<12} | {:>10} | {:<5} | {}",
            tx.id,
            tx.date,
            tx.tx_type,
            tx.exchange.unwrap_or_default(),
            tx.amount,
            tx.invested,
            tx.kyc,
            tx.note.unwrap_or_default()
        );

        if tx.tx_type == "BUY" {
            sum_euro += tx.invested.parse::<f64>().unwrap_or(0.0);
            sum_menge += tx.amount.parse::<f64>().unwrap_or(0.0);
        }
        else if tx.tx_type == "SELL" {
            sum_euro -= tx.invested.parse::<f64>().unwrap_or(0.0);
            sum_menge -= tx.amount.parse::<f64>().unwrap_or(0.0);

        }

    }

    let sum_sats = (sum_menge * 100_000_000.0).round() as u64; // Rechnet BTC in Satoshis um
    let current_value_eu = sum_menge * data_geko.bitcoin.eur; // Rechnet den Portfoliowert in Euro um
    let current_value_usd = sum_menge * data_geko.bitcoin.usd; // Rechnet den Portfoliowert in USD um
    let pln = current_value_eu - sum_euro; // Rechnet den Pln
    let break_even_eur = if sum_menge > 0.0 { // Berechnet break even
                        sum_euro / sum_menge}
                    else {0.0};

    let break_even_usd = break_even_eur * (data_geko.bitcoin.usd / data_geko.bitcoin.eur); // berechnet den breakeve in usd anhand vom ermittelten wechslekurd durch btc eu und usd

    println!("{}", "-".repeat(100)); // Erstellt die Trennlinie
    println!("Block Time: {}", block_time);
    println!("Your Coins {sum_menge:.8} BTC / {sum_sats} sat");
    println!("Your Invest: {sum_euro:.2} Euro ");
    println!("Your portfolio value: {current_value_eu:.2} Euro / {current_value_usd:.2} USD");
    println!("Break Even: {break_even_eur:.2} Euro / {break_even_usd:.2} USD");
    println!("PnL: {pln:.2} Euro");
    println!("{}", "-".repeat(100)); // Erstellt die Trennlinie

    

    // println!("der btc Kurs:{current_value_eu:.2} Euro");
    // println!("der btc Kurs:{current_value_usd:.2} US Dollar");

    Ok(())
}


