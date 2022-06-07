use instrmnt::instrument_macro;
fn main() {
    println!("Hello, world!");
}

#[instrument_macro]
macro_rules! test_expr_lang {
    (add($e:expr, $f:expr)) => {
        $e + $f;
    };
    (sub($e:expr, $f:expr)) => {
        $e - $f;
    };
}