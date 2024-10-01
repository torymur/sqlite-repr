### SQLite3 Representation

[SQLite](https://www.sqlite.org/) is a C-language library that implements a small, fast, self-contained, high-reliability, full-featured, SQL database engine. SQLite is the most used database engine in the world, built into all mobile phones and most computers and comes bundled inside countless other applications that people use every day.

The SQLite file format is stable, cross-platform and backwards compatible, the developers pledge to keep it that way through the year 2050.

All that makes it interesting to peek into their on-disk [database file format](https://www.sqlite.org/fileformat2.html) to understand it for software development objective and troubleshooting reasons, as well as to study format of SQLite databases for academic purposes or regular self-education.

### Visual
Available at https://torymur.github.io/sqlite-repr/

### Map 🗺️ 

#### Parser
- [x] Table Interior Btree page
- [x] Table Leaf Btree page
- [x] Index Interior Btree page
- [x] Index Leaf Btree page
- [x] Freelist pages
- [x] Overflow pages
  - [x] Spilled record values
  - [ ] Spilled record headers (rare)
- [ ] ~~Pointer map pages~~
- [ ] ~~Lock-byte page~~
- [ ] Freeblock & Fragmented bytes

#### UI
- [x] Hybrid, Hex, Text field repr
- [x] Preloaded example databases, details 
- [x] Page View
- [x] Tree View
- [ ] Reserved space
- [ ] Add yours
- [ ] Console  
