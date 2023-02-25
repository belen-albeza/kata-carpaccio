use std::io;

pub fn run<R>(reader: R) -> Result<String, std::num::ParseFloatError>
where
    R: io::BufRead,
{
    let raw_prices = read_line(reader);
    let prices = parse_prices(&raw_prices)?;
    let net_price = net_price_for_order(&prices);

    Ok(format!("{}", net_price))
}

fn read_line<R>(mut reader: R) -> String
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

fn net_price_for_order(prices: &[f64]) -> f64 {
    prices.iter().sum()
}
