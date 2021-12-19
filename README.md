# postgresql-ja-tokenizer

This is a postgresql extension to tokenize sentences in Japanese.
morphological tokenizer.

# Usage

This is simple usage.

```
ja_tokenizer=# CREATE EXTENSION ja_tokenizer;
CREATE EXTENSION
ja_tokenizer=# SELECT jat_tokenize('PostgreSQLで形態要素解析をする');
 jat_tokenize
--------------
 PostgreSQL
 で
 形態
 要素
 解析
 を
 する
(7 rows)
```

This is by json.

```
ja_tokenizer=# select * from  jat_tokenize_to_json('人工知能は最近発展した。');
                   jat_tokenize_to_json
-----------------------------------------------------------
 {"conjugation":"一般","feature":"名詞","text":"人工"}
 {"conjugation":"一般","feature":"名詞","text":"知能"}
 {"conjugation":"係助詞","feature":"助詞","text":"は"}
 {"conjugation":"副詞可能","feature":"名詞","text":"最近"}
 {"conjugation":"サ変接続","feature":"名詞","text":"発展"}
 {"conjugation":"自立","feature":"動詞","text":"し"}
 {"conjugation":"*","feature":"助動詞","text":"た"}
 {"conjugation":"句点","feature":"記号","text":"。"}
(8 rows)
```

For using user defined dictionary, use `jat_config`.

```
ja_tokenizer=# select jat_config('/app/sample/user_dic.csv');
 jat_config
------------
 /app/sample/user_dic.csv
(1 row)

ja_tokenizer=# select jat_tokenize('東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です');
       jat_tokenize
--------------------------
 東京スカイツリー
 の
 最寄り駅
 は
 とうきょうスカイツリー駅
 です
(6 rows)
```



# Install



# For contributors


## Unit tests

Test for all version of postgresql.

```
cargo pgx test
```

Test for postgresql 14 only.

```
cargo pgx test pg14
```

