// Match Option

#[derive(Debug)]

enum Balance {
    Small,
    Fish,
    Shark,
}
enum Coin {
    Solana,
    Ethereum,
    Bitcoin(Balance),
}

fn decimals(coin: Coin) -> u8 {
    match coin {
        Coin::Solana => {
            println!("Solana");
            1
        }
        Coin::Ethereum => 10,
        Coin::Bitcoin(bala) => {
            println!("I am a {:#?}", bala);
            30
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        Some (x) => Some (x+1),
        _=>None
    }
}

fn main() {
    // Match Option
    decimals(Coin::Solana);
    decimals(Coin::Bitcoin(Balance::Shark));

    let five = Some(5);
    let six = plus_one(five);
    println!("six = {:#?}",six);

    let none = plus_one(None);
    println!("none = {:#?}",none);
}
