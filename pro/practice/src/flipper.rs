mod flipper {
    #[event]
    enum Flipped {
        accountId,
        value,
    }

    #[storage]
    struct Flipper {
        flipped: bool,
    }

    #[message]
    pub fn flip() {
        flipped = !flipped;
    }

    pub fn xyz() {}
}
