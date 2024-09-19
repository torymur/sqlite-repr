/// Preloaded examples of databases to start UI with somethinh

use std::include_bytes;

pub const SIMPLE_DB: &str = "Simple";
pub const BIG_PAGE_DB: &str = "Max page size";
pub const TABLE_INDEX_LEAF_DB: &str = "Leaf nodes";
pub const OVERFLOW_PAGE_DB: &str = "Overflow pages";
pub const FREELIST_PAGE_DB: &str = "Freelist pages";
pub const TABLE_INDEX_INTERIOR_DB: &str = "Interior nodes";

#[allow(clippy::type_complexity)]
pub static INCLUDED_DB: &[(&str, (&[u8], &[&str]))] = &[
    (
        SIMPLE_DB,
        (
            include_bytes!("../included/simple"),
            &[
                "CREATE TABLE simple(int)",
                "INSERT INTO simple VALUES(1), (2), (3), (4)",
            ],
        ),
    ), 
    (
        BIG_PAGE_DB, 
        (
            include_bytes!("../included/big_page"),
            &[
                "PRAGMA page_size=65536",
                "CREATE TABLE big_page(int)",
                "INSERT INTO simple VALUES(1), (2), (3), (4)",
            ],
        ),
    ),
    (
       TABLE_INDEX_LEAF_DB,
       (
            include_bytes!("../included/table_index_leaf"),
            &[
                "CREATE TABLE stars(id INTEGER PRIMARY KEY, name TEXT, distance REAL, brightness REAL)",
                "INSERT INTO stars VALUES(100, 'Sirius', 8.6, -1.46), ... ",
                "CREATE INDEX idx_stars_name ON stars (name)",
                "CREATE TABLE spaceships(launched, name, operator)",
                "INSERT INTO spaceships VALUES(1977, 'Voyager 1', 'NASA'), ... ",
                "CREATE INDEX idx_spaceships_name ON spaceships(name)",
            ],
        ),
    ),
    (
       OVERFLOW_PAGE_DB,
       (
            include_bytes!("../included/overflow_page"),
            &[
                "PRAGMA page_size=1024",
                "CREATE TABLE mixed_overflow(text, longint, int, blob)",
                "CREATE TABLE blob_overflow(blob)",
                "INSERT INTO blob_overflow VALUES(fileio_read('dev/overflow.txt'))",
                "INSERT INTO mixed_overflow SELECT CAST(blob as TEXT), 234234235, 0, blob FROM blob_overflow",
                "INSERT INTO mixed_overflow SELECT CAST(blob as TEXT), 94542343, 1, blob FROM blob_overflow",
            ],
        ),
    ),
    (
        TABLE_INDEX_INTERIOR_DB,
        (
            include_bytes!("../included/table_index_interior"),
            &[
                "PRAGMA page_size=512",
                "CREATE TABLE macro_story(line)",
                "INSERT INTO macro_story SELECT VALUE FROM fileio_scan('dev/lines.txt')",
                "CREATE INDEX idx_macro_story_line ON macro_story(line)",
            ],
        ),
    ),
    (
       FREELIST_PAGE_DB,
       (
            include_bytes!("../included/freelist_page"),
            &[
                "PRAGMA page_size=1024",
                "CREATE TABLE mixed_overflow(text, blob)",
                "CREATE TABLE blob_overflow(blob)",
                "INSERT INTO blob_overflow VALUES(fileio_read('dev/overflow.txt'))",
                "INSERT INTO mixed_overflow SELECT CAST(blob as TEXT), blob FROM blob_overflow",
                "DELETE FROM mixed_overflow",
                "DROP TABLE blob_overflow",
            ],
        ),
    ),
];

