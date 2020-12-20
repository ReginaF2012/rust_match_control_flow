// Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into. In the same way, values go through each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution.

fn main() {
    #[derive(Debug)] // so we can inspect the state

    enum UsState {
        Alabama, 
        Alaska, 
        Arizona, 
        Arkansas, 
        California, 
        Colorado, 
        Connecticut, 
        Delaware, 
        Florida, 
        Georgia, 
        Hawaii, 
        Idaho, 
        Illinois, 
        Indiana, 
        Iowa, 
        Kansas, 
        Kentucky, 
        Louisiana, 
        Maine, 
        Maryland, 
        Massachusetts, 
        Michigan, 
        Minnesota, 
        Mississippi, 
        Missouri, 
        Montana, 
        Nebraska, 
        Nevada, 
        NewHampshire, 
        NewJersey, 
        NewMexico, 
        NewYork, 
        NorthCarolina, 
        NorthDakota, 
        Ohio, 
        Oklahoma, 
        Oregon, 
        Pennsylvania, 
        RhodeIsland, 
        SouthCarolina, 
        SouthDakota, 
        Tennessee, 
        Texas, 
        Utah, 
        Vermont, 
        Virginia, 
        Washington, 
        WestVirginia, 
        Wisconsin, 
        Wyoming,
        
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            // will still return 1, but also will execute the code to print "Lucky penny!"
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    // because matching is exhaustive, we have to cover every possible scenario when using match. So we use the _ placeholder to match to any value. Because it will match to any value it has to go at the end
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));
}
