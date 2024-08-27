use polars::prelude::*;


fn main() -> Result<(), PolarsError> {
    // Create a new DataFrame
    // let df = df![
	// "28" => [1, 1, 1, 1],
	// "29" => [1, 2, 3, 4],
	// "time" => [1, 2, 3, 4],
    // ]?;

    let df = polars_testing::make_df()?;
    println!("{}", df);

    Ok(())
}
