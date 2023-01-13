fn main() {
    let partidge = "a partridge in a pair tree";
    let lines = [
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "fiiiiiivvvve gooolllddd riinnnggs", // necessary!
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "nineth",
        "tenth",
        "eleventh",
        "twelth",
    ];

    // Want both day name and its index value for slicing the lines.
    for (i, day) in days.iter().enumerate() {
        println!("On the {day} day of Christmas, my true love gave to me:");

        // If day one just print partridge
        if i == 0 {
            println!("\t{partidge}");
            continue;
        }

        // Print all but partidge line
        // I ideally want to work the array
        let day_lines = &lines[0..i]; // Slice of the vector
        for line in day_lines.iter().rev() {
            println!("\t{line},");
        };

        // Print "And" partridge line.
        println!("\tand {partidge}.");
    };
}
