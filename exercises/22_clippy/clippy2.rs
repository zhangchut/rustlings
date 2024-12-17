fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    if let Some(val) = option {
        res += val;
    }

    println!("{res}");
}
