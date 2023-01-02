#[allow(unused)]
fn main() {
    let arr = [1, 2, 3]; // stack allocated because of fixed size
    let mut vec: Vec<i32> = Vec::new(); // heap allocated no fixed size
                                        /*_____________________________________________________________________________________*/
    {
        let mut v: Vec<i32> = Vec::with_capacity(10); // create a new Vec<i32> with a capacity of 10 elements
        assert_eq!(v.len(), 0); // check the length of the Vec
        assert_eq!(v.capacity(), 10); // check the capacity of the Vec

        v.reserve(20); // increase the capacity of the Vec to at least 20 elements
        assert_eq!(v.capacity(), 20);

        v.reserve_exact(30); // increase the capacity of the Vec to exactly 30 elements
        assert_eq!(v.capacity(), 30);
        v.shrink_to_fit(); // reduce the capacity of the Vec to the minimum required to hold its elements
        for i in 1..6 {
            v.push(i); // add elements to the end of the Vec
        }
        assert_eq!(v.len(), 5); // check the length of the Vec
        let w: Vec<i32> = (6..21).collect(); // create a new Vec<i32> from an iterator
        v.extend(w as Vec<i32>); // extend the Vec with the elements in another Vec
        assert_eq!(v.len(), 20);

        v.truncate(10); // The truncate method on a Vec in Rust can be used to shorten the vector by removing elements from the end. It takes a single argument, which is the new length of the vector.

        v.insert(0, 0); // insert a new element at an index, shifting the existing elements to the right
        v.remove(0); // remove an element at an index, shifting the remaining elements to the left

        v.retain(|x: &i32| x % 2 == 0); // remove all elements from the Vec that do not match a given predicate
        v.insert(0, 2); // insert a new element at an index, shifting the existing elements to the right
        v.dedup(); // remove consecutive duplicate elements from the Vec
        v.clear(); // remove all elements from the Vec, making it empty

        println!("{:#?}", v);

        let mut y: Vec<i32> = (1..6).collect();

        let z: Vec<i32> = y.iter().cloned().collect(); // create a new Vec from an iterator

        let a: Vec<i32> = y.iter_mut().map(|x| *x * 2).collect(); // create a new Vec from a mutable iterator

        let mut b: Vec<i32> = y.into_iter().filter(|x| x % 2 == 0).collect(); // create a new Vec by consuming the original Vec and filtering the elements

        let slice: &[i32] = b.as_slice(); // get a slice of the Vec

        let mut_slice: &mut [i32] = b.as_mut_slice(); // get a mutable slice of the Vec
        let mut new: Vec<u8> = (1..10).collect();
        let (left, right) = new.split_at(3); // split the Vec into two at a given index, returning the elements before the index as a new
        let (left, right) = new.split_at_mut(3); // split the Vec into two at a given index, returning the elements before the index as a new Vec and the elements after the index as a mutable slice
        println!("left: {:#?}", left);
        println!("right: {:#?}", right);
    }
    /*_____________________________________________________________________________________*/
}
