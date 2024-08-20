.PHONY: setup
setup: included included/simple included/big_page included/two_tables
	
included:
	mkdir $@

included/simple:
	sqlite3 $@ \
		'create table simple(int)' \
		'insert into simple values(1),(2),(3),(4)'

included/big_page:
	sqlite3 $@ \
		-cmd 'PRAGMA page_size=65536' \
		'create table big_page(int)' \
		'insert into big_page values(1),(2),(3),(4)'

included/two_tables:
	sqlite3 $@ \
		'create table simple(int)' \
		'insert into simple values(1),(2),(3),(4)' \
		'create table two_cols(int,text)' \
		'insert into two_cols values(1,"one"),(2,"two"),(3,"three"),(4,"four")'

.PHONY: clean
clean:
	rm -rf included
