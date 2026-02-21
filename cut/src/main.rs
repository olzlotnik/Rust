fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> (usize, usize) {
    // This is just a placeholder signature mentioned in the text
    unimplemented!()
}

fn first_word_better(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let slice = &s[..];

    let s_literal: &str = "Hello, world!";

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s, whether partial or whole.
    let word = first_word_better(&my_string[0..6]);
    let word = first_word_better(&my_string[..]);
    // first_word also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word_better(&my_string);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals, whether partial or whole.
    let word = first_word_better(&my_string_literal[0..6]);
    let word = first_word_better(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_better(my_string_literal);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
