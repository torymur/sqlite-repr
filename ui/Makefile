.PHONY: setup
setup: included included/simple included/big_page
	
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

.PHONY: clean
clean:
	rm -rf included
