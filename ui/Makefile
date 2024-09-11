.PHONY: setup
setup: included included/simple included/big_page included/overflow_page included/table_index_leaves included/interior_table
	
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
		'insert into mixed_overflow select cast(blob as TEXT), 234234235, 0, blob from blob_overflow' \
		'insert into mixed_overflow select cast(blob as TEXT), 94542343, 1, blob from blob_overflow'

included/interior_table:
	sqlite3 $@ \
		-cmd '.load ./dev/fileio' \
		-cmd 'PRAGMA page_size=512' \
		'create table macro_story(line)' \
		'insert into macro_story select value from fileio_scan("dev/lines.txt")'\
		'create table same_macro_story(line)' \
		'insert into same_macro_story select value from fileio_scan("dev/lines.txt")'

included/big_page:
	sqlite3 $@ \
		-cmd 'PRAGMA page_size=65536' \
		'create table big_page(int)' \
		'insert into big_page values(1),(2),(3),(4)'


included/table_index_leaves:
	sqlite3 $@ \
		'create table stars(id INTEGER PRIMARY KEY, name TEXT, distance REAL, brightness REAL)' \
		'insert into stars values(100,"Sirius",8.6,-1.46),(200,"Altair",16.7,0.77),(300,"Vega",25,0.03),(400,"Polaris",323,2.02)' \
		'create index idx_stars_name on stars (name)'\
		'create table spaceships(launched,name,operator)' \
		'insert into spaceships values(1977,"Voyager 1","NASA"),(1984,"Space Shuttle Discovery","NASA"),(2020,"SpaceX Crew Dragon","SpaceX")'\
		'create index idx_spaceships_name on spaceships(name)'\

.PHONY: clean
clean:
	rm -rf included

.PHONY: mini
mini: 
	npx tailwindcss -i ./input.css -o assets/tailwind.css --minify
