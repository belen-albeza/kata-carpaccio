use std::io;

pub fn run<R>(reader: &mut R) -> Result<String, String>
where
    R: io::BufRead,
{
    let raw_prices = read_line(reader);
    let prices = parse_prices(&raw_prices).map_err(|_| "Error reading prices".to_string())?;

    let raw_units = read_line(reader);
    let _ = parse_units(&raw_units).map_err(|_| "Error reading units".to_string())?;

    let net_price = net_price_for_order(&prices);

    Ok(format!("{}", net_price))
}

fn read_line<R>(reader: &mut R) -> String
where
    R: io::BufRead,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer).expect("Could not read line");
    buffer.trim_end().to_string()
}

fn parse_prices(text: &str) -> Result<Vec<f64>, std::num::ParseFloatError> {
    text.split(" ").map(|x| x.parse::<f64>()).collect()
}

fn parse_units(text: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    text.split(" ").map(|x| x.parse::<u64>()).collect()
}

fn net_price_for_order(prices: &[f64]) -> f64 {
    prices.iter().sum()
}
