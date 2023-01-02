use unicode_segmentation::UnicodeSegmentation;
use std::iter::FromIterator;


#[allow(unused)]
fn main() {
    // strings are stored in utf8 encoded bytes
    let c = '💥';
    println!("{}", c.len_utf16());
    let str = "anas jaidi";
    let s1 = str.to_string();
    let mut s2 = String::new();
    let s3 = String::from("jaidi anas");
    let s4 = s1 + &s3; // takes ownership of s1
    let s5 = format!("{} {}", s3, s4); // jaidi anas anas jaidijaidi anas
    println!("{}", s5);
    s2.push('a');
    s2.push_str("nas");
    println!("{}", s2); // anas
                        /* --------------------------------------------------------------------------------------- */
    let h = "नमस्ते";

    // string in bytes [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    for c in h.bytes() {
        println!("{c}");
    }

    // chars ['न', 'म', 'स', '्', 'त', 'े']

    for c in h.chars() {
        println!("{}", c.len_utf8());
    }
    // graphism ["न", "म", "स्", "ते"]
    for c in h.graphemes(true) {
        println!("{}", c);
    }
    /* --------------------------------------------------------------------------------------- */
    {
        // len: Gets the length of the string (in bytes). Copy code
        let s = "hello";
        let len = s.len(); // len is 5
    }
    /* --------------------------------------------------------------------------------------- */
    {
        let s = "";
        let is_empty = s.is_empty(); // is_empty is true
    }
    /* --------------------------------------------------------------------------------------- */
    {
        let s = String::with_capacity(10);
        let capacity = s.capacity(); // capacity is 10
    }
    /* --------------------------------------------------------------------------------------- */
    {
        let s = "   hello   ".trim(); // s is "hello"
    }
    /* --------------------------------------------------------------------------------------- */
    {
        // split: Splits the string into a vector of substrings, using a delimiter.
        let s = "a,b,c";
        let v: Vec<&str> = s.split(',').collect(); // v is ["a", "b", "c"]
    }
    /* --------------------------------------------------------------------------------------- */
    {
        // replace: Replaces all occurrences of a string slice with another string slice. Copy code
        let s = "hello, world!";
        let s = s.replace("world", "Rust"); // s is "hello, Rust!"
    }
    /* --------------------------------------------------------------------------------------- */
    /* --------------------------------------------------------------------------------------- */
    {
        let mut s = String::new(); // create a new, empty String

        s.push('H'); // add a character to the end of the String
        s.push_str("ello"); // add a string to the end of the String

        assert_eq!(s.len(), 5); // check the length of the String

        s.reserve_exact(10); // increase the capacity of the String to at least 15 bytes

        assert!(s.capacity() == 15); // check the length of the String

        s.reserve_exact(20); // increase the capacity of the String to exactly 25 bytes
        assert_eq!(s.capacity(), 25);

        s.shrink_to_fit(); // reduce the capacity of the String to the minimum required to hold its contents

        assert_eq!(s.is_empty(), false); // check if the String is empty

        assert_eq!(s.pop(), Some('o')); // remove and return the last character of the String

        s.truncate(3); // remove all characters from the String after the third

        assert_eq!(s.get(0..1), Some("H")); // get a slice of the String

        s.clear(); // remove all characters from the String, making it empty

        assert_eq!(s.is_empty(), true);

        let w = "worldw";

        s.push_str(w); // add a string to the end of the String

        assert_eq!(s.find('w'), Some(0)); // find the index of the first occurrence of a character in the String

        assert_eq!(s.rfind('w'), Some(5)); // find the index of the last occurrence of a character in the String

        let v: Vec<&str> = s.split('r').collect(); // split the String into a Vec of substrings at each occurrence of a character

        s.replace_range(0..1, "A"); // replace a range of characters in the String with new characters

        let mut t = "   hello   ".to_string();

        let trimed = t.trim(); // remove leading and trailing whitespace from the String

        assert_eq!(trimed, "hello");

        let trimed = t.trim_start(); // remove leading whitespace from the String

        assert_eq!(trimed, "hello   ");

        let trimed = t.trim_end(); // remove trailing whitespace from the String

        assert_eq!(trimed, "   hello");

        let trimed = t.trim_matches(' '); // remove leading and trailing occurrences of a character from the String

        assert_eq!(trimed, "hello");

        let trimed = trimed.trim_start_matches('h'); // remove leading occurrences of a character from the String

        assert_eq!(trimed, "ello");

        let trimed = trimed.trim_end_matches('o'); // remove trailing occurrences of a character from the String

        assert_eq!(trimed, "ell");

        let num: Option<f32> = "42.9".parse().ok(); // parse the String into a value of a given type

        let mut chars: Vec<char> = trimed.chars().collect(); // get an iterator over the characters in the String

        assert_eq!(chars, ['e', 'l', 'l']);

        chars.reverse(); // reverse the order of the characters in the Vec

        let reversed = String::from_iter(&chars);

        assert_eq!(reversed, "lle");

        let mut bytes: Vec<u8> = trimed.bytes().collect(); // get an iterator over the bytes in the String

        assert_eq!(bytes, [101, 108, 108]);

        bytes.reverse(); // reverse

        let reversed: String = String::from_utf8(bytes).unwrap(); // create a new String from a byte array
        
        
        assert_eq!(reversed, "lle");

        let mut ind: Vec<(usize, char)> = reversed.char_indices().collect(); 

        assert_eq!(ind[0], (0, 'l'));

        ind.reverse();

        let mut s: String = ind.into_iter().map(|(_, c)| c).collect();

        println!("string: {:#?}",s);


    }
    /* --------------------------------------------------------------------------------------- */
    /* --------------------------------------------------------------------------------------- */
    /*

    new: Creates a new, empty string.

    from: Creates a new string from a &str slice.

    with_capacity: Creates a new string with a specified capacity (in bytes).

    into_bytes: Converts the string into a vector of bytes.

    as_bytes: Gets a slice of the string's bytes.

    as_mut_bytes: Gets a mutable slice of the string's bytes.

    push: Appends a character to the end of the string.

    push_str: Appends a string slice to the end of the string.

    pop: Removes the last character from the string and returns it, if the string is non-empty.

    truncate: Removes all characters from the string after a specified length.

    clear: Removes all characters from the string, making it empty.

    reserve: Increases the capacity of the string to a specified value.

    reserve_exact: Increases the capacity of the string to a specified value.

    shrink_to_fit: Shrinks the capacity of the string to the minimum required to hold its current contents.

    len: Gets the length of the string (in bytes).

    is_empty: Returns true if the string is empty, false otherwise.

    capacity: Gets the capacity of the string (in bytes).

    contains: Returns true if the string contains a specified string slice, false otherwise.

    find: Returns the position of the first occurrence of a specified string slice within the string, if found.

    rfind: Returns the position of the last occurrence of a specified string slice within the string, if found.

    starts_with: Returns true if the string starts with a specified string slice, false otherwise.

    ends_with: Returns true if the string ends with a specified string slice, false otherwise.

    replace: Replaces all occurrences of a string slice with another string slice.

    replacen: Replaces at most n occurrences of a string slice with another string slice.

    strip_prefix: Removes a specified prefix from the string, if present.

    strip_suffix: Removes a specified suffix from the string, if present.

    trim: Removes leading and trailing white space from the string.

    trim_start: Removes leading white space from the string.

    trim_end: Removes trailing white space
        */
    /* --------------------------------------------------------------------------------------- */
    /* --------------------------------------------------------------------------------------- */
    /* --------------------------------------------------------------------------------------- */
}
