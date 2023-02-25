# kata-carpaccio

## Backlog

- [x] As a user, I can **run the program**.

  - Setup Rust binary crate.
  - Output `0` in stdout.

- [x] As a user, I can **input text** into the program.

  - User inputs a string and the program will print it back.

- [x] As a user, I can **input a list of prices**.

  - Parse the input so it gets converted to a list of numbers.
  - The program will panic if parsing error.

- [x] As a user, I can **get the sum of prices**.

  - Sum the prices in the list and print it in stdout.

- [x] As a user, I can **input a list of units**.

  - User inputs another string that will get parsed to a list of number of units.
  - The program will panic if parsing error.

- [x] As a user, I can **net price for the order**.

  - Instead of printing just the sum of prices, take into account the amount of units per item as well.

- [x] As a user, I can get the **gross price (with VAT applied) for Spain (21% VAT)**.

  - Print the gross price instead of the net price.
  - The gross price is calculated by adding 21% to the net price.

- [ ] As a user, I can input **how much VAT I want to apply**.

  - User inputs another string that will get parsed as the % of tax to apply (i.e. `21` -> 21% VAT).
  - The program will panic if parsing error.

- [ ] As a user, I can input the **country code instead of the VAT %** so I don't need to remember it.

  - User inputs another string that will get parsed into a country code (at the moment, just `es` or `ES` for Spain)
  - The program will panic if parsing error.

- [ ] As a user, I can **input any supported country code** (Spain, Portugal, France)

  - The gross price will be calculated depending on the country code.
  - The program will panic if parsing error.

- [ ] As a user, I can **apply a small discount to orders** above a threshold.

  - Orders above €1,000 (net price) will get a 3% discount, before tax

- [ ] As a user, I can have a **list of available discounts to be automatically applied**.

  - Orders above €5,000 will get a 5% discount.
  - Orders above €7,000 will get a 7% discount.
  - Orders above €10,000 will get a 10% discount.
  - Orders above €50,000 will get a 15% discount.

- [ ] As a user I can see the **final gross price nicely formatted**.

  - Apply a human-friendly format to the raw number that gets printed (i.e. `1221.2` -> `€1,221.20`)

- [ ] As a user, I can get a **detailed receipt**.
  - Per each item, print the price and the amount (i.e. `€19.95 × 2`) in a separate line.
  - Print the net price (before tax)
  - Print any discount applied
  - Print gross price, along with the VAT % and the country code
