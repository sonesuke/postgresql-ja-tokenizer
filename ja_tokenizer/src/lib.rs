use pgx::*;
use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pg_module_magic!();

static TOKENIZER: Lazy<Mutex<Tokenizer>> = Lazy::new(|| Mutex::new(Tokenizer::new(Mode::Normal, "")));

#[pg_extern]
fn jat_tokenize(input: &str) -> impl std::iter::Iterator<Item=String> {
    let tokens = TOKENIZER.lock().unwrap().tokenize(input);
    let mut ret = Vec::<String>::new();
    for token in tokens {
        ret.push(String::from(token.text));
    }
    ret.into_iter()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_jat_tokenize() {
        let count = Spi::get_one::<i32>("SELECT COUNT(*) FROM jat_tokenize('人工知能は最近発展した。');")
            .expect("failed to get SPI result");
        assert_eq!(count, 8);
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
