use std::collections::HashMap;
use std::io;

fn main() {
    // Given a list of integers, use a vector and return the median (when sorted, the value in the
    // middle position) and mode (the value that occurs most often; a hash map will be helpful here)
    // of the list.
    let mut numbers = vec![6, 3, 11, 34, 5, 77, 55, 55, 1];
    numbers.sort();

    println!("{:?}", numbers);

    let mut middle_pos = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        middle_pos -= 1;
    }

    println!("The median is: {}", numbers[middle_pos]);

    let mut map = HashMap::new();

    for number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode= -1;
    for (key, value) in map.iter() {
        if *value > max {
            mode = *key;
            max = *value;
        }
    }

    println!("{:?}", map);
    println!("The mode is: {}", mode);

    println!("---");

    // Convert strings to pig latin. The first consonant of each word is moved to the end of the
    // word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have
    // “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about
    // UTF-8 encoding!
    fn word_to_pig_latin(word: &str) -> String {
        let first = &word[0..1];
        let vowels = ["a", "e", "i", "o", "u"];
        for vowel in vowels {
            if first == vowel {
                return format!("{}-hay", word);
            }
        }

        return format!("{}-{}ay", &word[1..word.len()], first);
    }

    println!("apple: {}, first: {}", word_to_pig_latin("apple"), word_to_pig_latin("first"));

    println!("---");

    // Using a hash map and vectors, create a text interface to allow a user to add employee names
    // to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department or all people in the company
    // by department, sorted alphabetically.
    let mut people_by_department = HashMap::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let mut index = 0;
        let mut person = String::from("");
        let mut department = String::from("");
        for word in input.split_whitespace() {
            if index == 1 {
                person = word.to_string();
            } else if index == 3 {
                department = word.to_string();
            }
            index += 1;
        };
        let people = people_by_department.entry(department).or_insert(vec![]);
        people.push(person);
        people.sort();
        println!("{:?}", people_by_department);
    }
}
