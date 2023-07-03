mod order_book {

    #[derive(Debug, PartialEq)]
    pub struct LimitOrder {
        limit_price: u64,
        amount: u64,
    }

    #[derive(Debug, PartialEq)]
    pub struct OrderBook {
        // asks (orders to sell BTC in this case) sorted ascending by price
        asks: Vec<LimitOrder>,
    }

    impl OrderBook {
        pub fn new() -> Self {
            OrderBook {
                asks: Vec::from([]),
            }
        }

        // create ask without matching
        pub fn create_ask(&mut self, limit_price: u64, amount_to_sell: u64) {
            let ask_new = LimitOrder {
                limit_price: limit_price,
                amount: amount_to_sell,
            };
            // assume asks are always inserted in ascending order
            self.asks.push(ask_new);
        }

        // create bid with matching
        pub fn create_bid(&mut self, limit_price: u64, amount_to_buy: u64) {
            let bid_new = LimitOrder {
                limit_price: limit_price,
                amount: amount_to_buy,
            };
            self.match_order(bid_new);
        }

        // TODO
        fn match_order(&mut self, order: LimitOrder) {
            let mut to_be_removed = Vec::new();
            let mut tot_amount = 0;
            for (idx, a) in self.asks.iter_mut().enumerate() {
                if a.limit_price <= order.limit_price && tot_amount < order.amount {
                    if a.amount < order.amount {
                        to_be_removed.push(idx);
                        tot_amount += a.amount;
                    } else if a.amount == order.amount {
                        a.amount = tot_amount;
                        tot_amount += a.amount;
                    }
                }
            }
            for idx in to_be_removed.iter().rev() {
                self.asks.remove(*idx);
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    //for tests, orderbook trackes BTC/USDC pair. Hence the limit price denotes the BTC/USDC price
    use crate::order_book::OrderBook;

    #[test]
    fn orderbook_matches() {
        let mut order_book = OrderBook::new();

        // add orders to sell btc
        order_book.create_ask(20000, 1);
        order_book.create_ask(20000, 5);
        order_book.create_ask(20001, 10);

        // create order to buy btc
        order_book.create_bid(20001, 10);

        // compare end state of order book to desired state
        let mut order_book_test = OrderBook::new();
        order_book_test.create_ask(20001, 6);
        assert_eq!(order_book, order_book_test);
    }
}
