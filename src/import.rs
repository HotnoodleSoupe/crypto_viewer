use serde::Deserialize;

#[derive(Deserialize)]
pub struct DataImport {
    pub date: String,
    pub exchange: String,
    pub tx_type: String,
    pub invested: f64,
    pub amount: f64, 
    pub kyc: bool,
    pub note: Option<String>

}

pub fn load_transactons(path: &str) -> Result<Vec<DataImport>, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut transactions = Vec::new();

    for result in reader.deserialize() {
        let mut row: DataImport = result?;
        row.tx_type = row.tx_type.to_uppercase();
        transactions.push(row);  

    }
     Ok(transactions)
}