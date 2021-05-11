# Test
run:
	cargo run -- data/test.xml

# Cf README re: fulltext column
schema:
	diesel print-schema > src/schema.rs

# Run after data load to set fulltext column
tsvec:
	psql ct < tsvec.sql
