use lindera::tokenizer::{Tokenizer, TokenizerConfig};
use lindera_core::viterbi::Mode;
use once_cell::sync::OnceCell;
use pgx::*;
use serde_json::*;
use std::path::PathBuf;
use std::sync::Mutex;

pg_module_magic!();

static TOKENIZER: OnceCell<Mutex<Tokenizer>> = OnceCell::new();

#[pg_extern]
fn jat_tokenize(input: &str) -> impl std::iter::Iterator<Item = String> {
    let t = TOKENIZER.get_or_init(|| Mutex::new(Tokenizer::new().unwrap()));
    let result = t.lock().unwrap().tokenize(input);
    let mut ret = Vec::<String>::new();
    match result {
        Err(why) => panic!("{:?}", why),
        Ok(tokens) => {
            for token in tokens {
                ret.push(String::from(token.text));
            }
        }
    }
    ret.into_iter()
}

#[pg_extern]
fn jat_tokenize_to_json(input: &str) -> impl std::iter::Iterator<Item = Json> {
    let t = TOKENIZER.get_or_init(|| Mutex::new(Tokenizer::new().unwrap()));
    let result = t.lock().unwrap().tokenize(input);
    let mut ret = Vec::<Json>::new();
    match result {
        Err(why) => panic!("{:?}", why),
        Ok(tokens) => {
            for token in tokens {
                ret.push(Json(json! { { "text": token.text, "feature": token.detail[0], "conjugation": token.detail[1]} }));
            }
        }
    }
    ret.into_iter()
}

#[pg_extern]
fn jat_config(input: &str) -> &str {
    let path = PathBuf::from(input);
    let config = TokenizerConfig {
        user_dict_path: Some(path.as_path()),
        mode: Mode::Normal,
        ..TokenizerConfig::default()
    };
    let mut t = TOKENIZER
        .get_or_init(|| Mutex::new(Tokenizer::new().unwrap()))
        .lock()
        .unwrap();
    *t = Tokenizer::with_config(config).unwrap();
    input
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_jat_tokenize() {
        let count =
            Spi::get_one::<i32>("SELECT COUNT(*) FROM jat_tokenize('人工知能は最近発展した。');")
                .expect("failed to get SPI result");
        assert_eq!(count, 8);
    }

    #[pg_test]
    fn test_jat_tokenize_to_json() {
        let count = Spi::get_one::<i32>(
            "SELECT COUNT(*) FROM jat_tokenize_to_json('人工知能は最近発展した。');",
        )
        .expect("failed to get SPI result");
        assert_eq!(count, 8);
    }

    #[pg_test]
    fn test_jat_config() {
        let config = Spi::get_one::<&str>("SELECT jat_config('/app/sample/user_dic.csv');")
            .expect("failed to get SPI result");
        assert_eq!(config, "/app/sample/user_dic.csv");

        let count = Spi::get_one::<i32>("SELECT COUNT(*) FROM jat_tokenize('東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です');")
            .expect("failed to get SPI result");
        assert_eq!(count, 6);
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
