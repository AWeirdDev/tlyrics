use anyhow::Result;
use tlyrics::get_by_id;

#[tokio::main]
async fn main() -> Result<()> {
    let track = get_by_id(5432440).await?;
    println!("{:#?}", track);

    Ok(())
}
