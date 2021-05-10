# Clinical Trials XML load in Rust

```
$ cargo run -- data/test.xml
     ...
Processings 1 file...
     1: test.xml => NCT00000516 (251841)
```

## Altering Database

If you alter the database, you need to run `make schema` to recreate _src/schema.rs_.
The `study.fulltext` field needs to be removed from the created schema because the `TsVector` stuff does not play nicely in Diesel.
I will have to load a plain text column `study.fulltext_load` and then copy the data into the indexed column.

## Author

Ken Youens-Clark <kyclark@c-path.org>
