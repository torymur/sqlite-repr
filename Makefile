.PHONY: setup
setup:
	rm -rf examples/
	mkdir examples

	sqlite3 examples/simple \
		'drop table if exists simple' \
		'create table simple(int)' \
		'insert into simple values(1),(2),(3),(4)'
	
	sqlite3 examples/big_page \
		-cmd 'PRAGMA page_size=65536' \
		'drop table if exists big_page' \
		'create table big_page(int)' \
		'insert into big_page values(1),(2),(3),(4)'
