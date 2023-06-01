use datafusion::prelude::*;

#[tokio::main]
pub async fn main() -> datafusion::error::Result<()> {
    //Register the table
    let ctx = SessionContext::new();

    ctx.register_csv("example", "./data/example.csv", CsvReadOptions::new()).await?;

    let df = ctx.sql("SELECT a, MIN(b) FROM example WHERE a <= b GROUP BY a LIMIT 100").await?;

    df.show().await?;

    let ctx = SessionContext::new();
    let df = ctx.read_csv("./data/example.csv", CsvReadOptions::new()).await?;
    let df = df.filter(col("a").lt_eq(col("b")))?
            .aggregate(vec![col("a")], vec![min(col("b"))])?
            .limit(0, Some(100))?;
    // Print results
    df.show();
    Ok(())

}



