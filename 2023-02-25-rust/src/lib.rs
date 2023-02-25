use std::io;

pub fn run<R>(reader: &mut R) -> Result<String, String>
where
    R: io::BufRead,
{
    let raw_prices = read_line(reader);
    let prices = parse_prices(&raw_prices).map_err(|_| "Error reading prices".to_string())?;

    let raw_units = read_line(reader);
    let units = parse_units(&raw_units).map_err(|_| "Error reading units".to_string())?;

    let items: Vec<(f64, u64)> = std::iter::zip(prices, units).collect();

    let net_price = net_price_for_order(&items);
    let gross_price = add_tax(net_price);

    Ok(format!("{}", gross_price))
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

fn net_price_for_order(items: &[(f64, u64)]) -> f64 {
    items
        .iter()
        .map(|(price, amount)| price * *amount as f64)
        .sum()
}

fn add_tax(net_price: f64) -> f64 {
    net_price * 1.21
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_price_for_order() {
        let items = vec![(19.95, 1), (10.00, 2)];
        assert_eq!(net_price_for_order(&items), 39.95);
    }

    #[test]
    fn test_add_tax() {
        assert_eq!(add_tax(50.0), 50.0 * 1.21);
    }
}
