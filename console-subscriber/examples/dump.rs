use console_api::tasks::{tasks_client::TasksClient, TasksRequest};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    args.next(); // drop the first arg (the name of the binary)
    let target = args.next().unwrap_or_else(|| {
        eprintln!("using default address (http://127.0.0.1:6669)");
        String::from("http://127.0.0.1:6669")
    });

    eprintln!("CONNECTING: {}", target);
    let mut client = TasksClient::connect(target).await?;

    let request = tonic::Request::new(TasksRequest {});
    let mut stream = client.watch_tasks(request).await?.into_inner();

    let mut i: usize = 0;
    while let Some(update) = stream.next().await {
        match update {
            Ok(update) => {
                eprintln!("UPDATE {}: {:#?}\n", i, update);
                i += 1;
            }
            Err(e) => {
                eprintln!("update stream error: {}", e);
                return Err(e.into());
            }
        }
    }

    eprintln!("update stream terminated");
    Ok(())
}
