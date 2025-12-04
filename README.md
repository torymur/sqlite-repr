### SQLite3 Representation

[SQLite](https://www.sqlite.org/) is a C-language library that implements a small, fast, self-contained, high-reliability, full-featured, SQL database engine. SQLite is the most used database engine in the world, built into all mobile phones and most computers and comes bundled inside countless other applications that people use every day.

The SQLite file format is stable, cross-platform and backwards compatible, the developers pledge to keep it that way through the year 2050.

All that makes it interesting to peek into their on-disk [database file format](https://www.sqlite.org/fileformat2.html) to understand it for software development objective and troubleshooting reasons, as well as to study format of SQLite databases for academic purposes or regular self-education.

### Objective

This project provides the visual tool available at https://torymur.github.io/sqlite-repr/ to explore SQLite’s on-disk database file format.

It has some already included database examples to show off main type of SQLite pages like table/index interior/leaf btree pages, freelist and overflow and some other SQLite features.

It also allows you to load and parse your own database file. However, parsing may still fail even for valid database files, as the parser does not yet support all SQLite features.

If that happens, please consider opening an issue and including a minimal example that reproduces the sequence of database queries.

### Development

To run UI locally, please follow the instructions [here](ui/)

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
- [x] Add yours
- [ ] Console  
