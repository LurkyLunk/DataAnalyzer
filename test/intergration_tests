use data_ingestion::DataModel;
use tokio;

#[tokio::test]
async fn test_data_processing() {
    let data = DataModel {
        id: 1,
        content: "Test data".to_string(),
    };

    // Assuming `process_data` is an async function you've defined
    // in your library to process or handle data
    let result = data_ingestion::process_data(&data).await;
    assert!(result.is_ok());
}
