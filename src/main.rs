fn main() {
    let farenheit: i32 = 4;

    println!("{} farenheit is {} celsius!",farenheit, farenheit_to_celsius(farenheit));
    fibonacci(5);
    christmas_carol();
}

fn farenheit_to_celsius(x: i32) -> i32 {
    (x - 32) * 5/9
}

fn fibonacci(mut n: i32){
    let mut x = 0;
    let mut y = 0;
    while n > 0{
        println!("{}",x);
        if x == 0{
            x = x + 1;
        }else{
            x = x + y;
            y = x - y;
        }
        n = n-1;
    }
}

fn christmas_carol(){
    let mut day = 1;
    let gifts =["A partridge in a pear tree","turtle doves", "french hens", "four calling birds",
        "five gold rings", "six geese a-laying","seven swans a-swimming","eight maids a-milking",
        "nine ladies dancing", "ten lords a-leaping","eleven pipers piping","twelve drummers drummings"];
    

    while day < 13{
        println!("For the day {} of Chirstmas my true love sent to me", day);
        for gift in 0..day{
            if gift == 0 {
                println!("{}\n",gifts[gift]);
            }else{
                println!("{} {}\n",gift+1, gifts[gift]);
            }
           
        }
        day = day + 1;
    }
}