.PHONY: setup
setup: included included/simple included/big_page included/overflow_page included/two_tables
	
included:
	mkdir $@

included/simple:
	sqlite3 $@ \
		'create table simple(int)' \
		'insert into simple values(1),(2),(3),(4)'

included/overflow_page:
	sqlite3 $@ \
		-cmd '.load ./dev/fileio' \
		-cmd 'PRAGMA page_size=1024' \
		'create table mixed_overflow(text,longint,int,blob)' \
		'create table blob_overflow(blob)' \
		'insert into blob_overflow values(fileio_read("dev/overflow.txt"))' \
		'insert into mixed_overflow select cast(blob as TEXT), 234234235, 0, blob from blob_overflow'

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

.PHONY: mini
mini: 
	npx tailwindcss -o assets/tailwind.css --minify
