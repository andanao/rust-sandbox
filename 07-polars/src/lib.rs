use polars::prelude::*;
use polars::df;

pub fn make_df() -> Result<DataFrame, PolarsError>{
    df![
	"28" => [1, 1, 1, 1],
	"29" => [1, 2, 3, 4],
	"time" => [1, 2, 3, 4],
    ]
}
