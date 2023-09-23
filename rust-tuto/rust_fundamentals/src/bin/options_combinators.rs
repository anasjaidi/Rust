fn main() {
    let a = Some(1);

    let is_none = a.is_none();
    dbg!(is_none);
    let is_some = a.is_some();
    dbg!(is_some);
    let mapped_a = a.map(|a| a + 1);
    dbg!(mapped_a);
    let filtred_a = a.filter(|a| a == &2);
    dbg!(filtred_a);
    let a_or_else = a.or_else(|| Some(-1));
    dbg!(a_or_else);
    let unwrapped_a_or_else = a.unwrap_or_else(|| -1);
    dbg!(unwrapped_a_or_else);
}
