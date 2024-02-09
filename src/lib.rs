use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataModel {
    pub id: u32,
    pub content: String,
}

pub async fn process_data(data: &DataModel) -> Result<(), String> {
    // Simulate data processing
    println!("Processing data: {:?}", data);
    // Here, you would add real logic, such as validating data, transforming it,
    // and sending it to a database or another service.

    Ok(())
}
