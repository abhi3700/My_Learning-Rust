#[allow(dead_code)]
const MAX_LEN: usize = 5;

#[allow(dead_code)]
fn price_push(prices: &mut Vec<u64>, price: u64) {
    if prices.len() < MAX_LEN {
        prices.push(price);
    } else {
        prices[price as usize % MAX_LEN] = price;
    }
}
#[test]
fn test_push_to_empty_buffer() {
    let mut prices = Vec::new();
    let price = 10;
    price_push(&mut prices, price);
    assert_eq!(prices, vec![10]);
}

#[test]
fn test_push_to_full_buffer() {
    let mut prices = vec![1, 2, 3, 4, 5];
    let price = 10;
    price_push(&mut prices, price);
    assert_eq!(prices, vec![10, 2, 3, 4, 5]);
}

#[test]
fn test_overwrite_oldest_price() {
    let mut prices = vec![1, 2, 3, 4, 5];
    let price = 10;
    price_push(&mut prices, price);
    assert_eq!(prices, vec![10, 2, 3, 4, 5]);
    price_push(&mut prices, 20);
    assert_eq!(prices, vec![20, 2, 3, 4, 5]);
}

#[test]
fn test_overwrite_multiple_prices() {
    let mut prices = vec![1, 2, 3, 4, 5];
    let price = 10;
    price_push(&mut prices, price);
    price_push(&mut prices, 20);
    price_push(&mut prices, 30);
    price_push(&mut prices, 40);
    price_push(&mut prices, 50);
    price_push(&mut prices, 60);
    assert_eq!(prices, vec![60, 2, 3, 4, 5]);
}

fn main() {}
