use anyhow::Result;
use tlyrics::search;

#[tokio::main]
async fn main() -> Result<()> {
    let res = search("slow dancing in the dark").await?;
    println!("{:#?}", res);
    Ok(())
}
