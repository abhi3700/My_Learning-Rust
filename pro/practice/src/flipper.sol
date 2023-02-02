contract Flipper {
    bool public flipped;

    event Flipped(address indexed flipper, bool flipped);

    function flip() external {
        flipped = !flipped;
    }
}
