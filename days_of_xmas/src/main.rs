// constant string array voodoo from
// https://stackoverflow.com/questions/27459640/how-to-create-a-static-array-of-strings
// :)
const DAYS: & [& str] = &["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
const GIFTS: & [& str] = &["And a partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds", "Five gold rings", "Six geese a laying", "Seven swans a swimming", "Eight maids a milking", "Nine ladies dancing", "Ten lords a leaping", "Eleven pipers piping", "Twelve drummers drumming"];

fn main() {
    println!("Twelve Days of Christmas");

    for day in 0..12 {
        println!("");
        print_on_this_day(day);
        if day == 0 {
            println!("A partridge in a pear tree");
        } else {
            for day in (0..day+1).rev() {
                println!("{}", GIFTS[day]);
            }
        }
    }
}

fn print_on_this_day(day: usize) {
    println!("On the {} day of Christmas", DAYS[day]);
    println!("My true love gave to me");
}
