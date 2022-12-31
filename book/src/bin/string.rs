use unicode_segmentation::UnicodeSegmentation;

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
