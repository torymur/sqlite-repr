//! DBHeader UI representation and description
use std::rc::Rc;

use parser::header::DBHeader;

use crate::{Field, Part, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct DBHeaderPart {
    pub header: Rc<DBHeader>,
}

impl Part for DBHeaderPart {
    fn label(&self) -> String {
        "Database Header".to_string()
    }

    fn desc(&self) -> &'static str {
        "The first 100 bytes of the database file comprise the database file header. All multibyte fields in the database file header are stored with the most significant byte first (big-endian)."
    }

    fn color(&self) -> String {
        "orange".to_string()
    }

    fn fields(&self) -> Vec<Field> {
        vec![
            Field::new(
                "Magic header string, which corresponds to the UTF-8 string: 'SQLite format 3\\000. Every valid SQLite database file begins with these 16 bytes (in hex): 53 51 4c 69 74 65 20 66 6f 72 6d 61 74 20 33 00.",
                0,
                16,
                Value::Text(self.header.header.clone()),
                ""
            ),
            Field::new(
                "Page size of the database, interpreted as a big-endian integer and must be a power of two between 512 and 32786, inclusive. Starting from version 3.7.1 page size of 65536 bytes is supported, but since it won't fit in a two-byte integer, big-endian magic number 1 is used to represent it: 0x00 0x01.",
                16,
                2,
                Value::PageSize(self.header.page_size),
                ""
            ),
            Field::new(
                "File format write version, 1 for legacy, 2 for WAL. Intended to allow for enhancements of the file format in future versions of SQLite. If read version is 1 or 2, but the write version is greater than 2, then the database file must be treated as read-only. If read version is greater than 2, then database cannot be read or written.",
                18,
                1,
                Value::U8(self.header.write_version),
                ""
            ),
            Field::new(
                "File format read version, 1 for legacy, 2 for WAL. Intended to allow for enhancements of the file format in future versions of SQLite. If read version is 1 or 2, but the write version is greater than 2, then the database file must be treated as read-only. If read version is greater than 2, then database cannot be read or written.",
                19,
                1,
                Value::U8(self.header.read_version),
                ""
            ),
            Field::new(
                "Number of bytes to define unused (reserved) space at the end of each page, usually 0. These bytes are used by extensions, for example, by the SQLite Encryption Extension to store a nonce and/or cryptographic checksum associated with each page. The 'usable size' of a database page is: Page size - Reserved space. It could be an odd number, but it's not allowed to be less than 480, which means that in this case reserved space size won't exceed 32.",
                20,
                1,
                Value::U8(self.header.reserved_page_space),
                ""
            ),
            Field::new(
                "Maximum embedded payload fraction, must be 64. Intended to be tunable parameters that could be used to modify the storage format of the b-tree algorithm. However, that functionality is not supported and there are no current plans to add support in the future, thus these bytes are fixed at the specified values.",
                21,
                1,
                Value::U8(self.header.max_embedded_payload_fraction),
                ""
            ),
            Field::new(
                "Minimum embedded payload fraction, must be 32. Intended to be tunable parameters that could be used to modify the storage format of the b-tree algorithm. However, that functionality is not supported and there are no current plans to add support in the future, thus these bytes are fixed at the specified values.",
                22,
                1,
                Value::U8(self.header.min_embedded_payload_fraction),
                ""
            ),
            Field::new(
                "Leaf payload fraction, must be 32. Intended to be tunable parameters that could be used to modify the storage format of the b-tree algorithm. However, that functionality is not supported and there are no current plans to add support in the future, thus these bytes are fixed at the specified values.",
                23,
                1,
                Value::U8(self.header.leaf_payload_fraction),
                ""
            ),
            Field::new(
                "File change counter, which is incremented whenever the database file is unlocked after having been modified. When two or more processes are reading the same database file, each process can detect database changes from the other processes by monitoring it. In that case a process will normally want to flush its database page cache, since the cache has become stale. In WAL mode, changes to the database are detected using the wal-index and so the change counter is not needed. Hence, the change counter might not be incremented on each transaction in WAL mode.",
                24,
                4,
                Value::U32(self.header.file_change_counter),
                ""
            ),
            Field::new(
                "Size of the database file in pages. If it's not valid, then the database size is computed by looking at the actual size of the database file, as did older versions of SQLite. New versions use it if it's available, but fallback to the actual file size. This number is only considered valid if it's non-zero and file change counter (offset 24) matches version valid for number (offset 92). Hence, invalid in-header database sizes can be detected (and ignored) by observing when the change-counter does not match the version-valid-for number.",
                28,
                4,
                Value::U32(self.header.db_size),
                ""
            ),
            Field::new(
                "Page number of the first freelist trunk page. Unused pages in the database file are stored on a freelist or zero if the freelist is empty.",
                32,
                4,
                Value::U32(self.header.first_free_page_num),
                ""
            ),
            Field::new(
                "Total number of freelist pages.",
                36,
                4,
                Value::U32(self.header.freelist_total),
                ""
            ),
            Field::new(
                "The schema cookie, which is incremented whenever the database schema changes. A prepared statement is compiled against a specific version of the database schema. When the database schema changes, the statement must be reprepared. When a prepared statement runs, it first checks the schema cookie to ensure the value is the same as when the statement was prepared and if the schema cookie has changed, the statement either automatically reprepares and reruns or it aborts with an SQLite schema error.",
                40,
                4,
                Value::U32(self.header.schema_cookie),
                ""
            ),
            Field::new(
                "The schema format number, which is similar to the file format read and write version numbers, except that the schema format number refers to the high-level SQL formatting, rather than the low-level b-tree formatting. Supported schema formats are 1, 2, 3 and 4. Format 1: understood by all versions back to 3.0.0. Format 2: adds the ability of rows within the same table to have a varying number of columns. Format 3: adds ability of extra columns to have non-NULL default values. Format 4: causes SQLite to respect the DESC keyword on index declarations, also adds two new boolean record type values, default format. Legacy_file_format pragma can be used to change it or via SQLITE_DEFAULT_FILE_FORMAT at a compile-time.",
                44,
                4,
                Value::U32(self.header.schema_format_num),
                ""
            ),
            Field::new(
                "Suggested default page cache size. This value is the suggestion only and SQLite is under no obligation to honor it. Suggested cache size can be set using the default_cache_size pragma.",
                48,
                4,
                Value::U32(self.header.default_page_cache_size),
                ""
            ),
            Field::new(
                "The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise. If it's zero then pointer-map pages are omitted from the database file and neither auto_vacuum nor incremental_vacuum are supported. If the integer is non-zero then it is the page number of the largest root page in the database file, the database file will contain ptrmap pages, and the mode must be either auto_vacuum or incremental_vacuum. In this latter case, the integer at offset 64 is true for incremental_vacuum and false for auto_vacuum. If the integer at offset 52 is zero then the integer at offset 64 must also be zero.",
                52,
                4,
                Value::U32(self.header.largest_root),
                ""
            ),
            Field::new(
                "The database text encoding. A value of 1 means UTF-8, 2: UTF-16le, 3: UTF-16be. No other values are allowed.",
                56,
                4,
                Value::Encoding(self.header.text_encoding),
                ""
            ),
            Field::new(
                "The 'user version' as read and set by the user_version pragma. The user version is not used by SQLite.",
                60,
                4,
                Value::U32(self.header.user_version),
                ""
            ),
            Field::new(
                "True (non-zero) for incremental-vacuum mode. False (zero) otherwise. If the integer at offset 52 is zero then pointer-map pages are omitted from the database file and neither auto_vacuum nor incremental_vacuum are supported. If the integer at the offset 52 is non-zero then it is the page number of the largest root page in the database file, the database file will contain ptrmap pages, and the mode must be either auto_vacuum or incremental_vacuum. In this latter case, the integer at offset 64 is true for incremental_vacuum and false for auto_vacuum. If the integer at offset 52 is zero then the integer at offset 64 must also be zero.",
                64,
                4,
                Value::Bool(self.header.inc_vacuum_mode),
                ""
            ),
            Field::new(
                "The 'Application ID' set by pragma application_id command in order to identify the database as belonging to or associated with a particular application. The application ID is intended for database files used as an application file-format. The application ID can be used by utilities such as file to determine the specific file type rather than just reporting 'SQLite3 Database'. A list of assigned application IDs can be seen by consulting the magic.txt file in the SQLite source repository.",
                68,
                4,
                Value::U32(self.header.application_id),
                ""
            ),
            Field::new(
                "Reserved for future expansion, must be set to zero.",
                72,
                20,
                Value::Array(Box::new(self.header.reserved_for_expansion)),
                ""
            ),
            Field::new(
                "The version-valid-for number is the value of the change counter when the version number was stored, indicates which transaction the version number is valid for.",
                92,
                4,
                Value::U32(self.header.version_valid_for_number),
                ""
            ),
            Field::new(
                "SQLite version number, that most recently modified the database file. The format is 'X.Y.Z', where X is the major version number (always 3 for SQLite3), Y is the minor version number, Z is the release number. The SQLITE_Version_NUMBER C preprocessor macro resolves to an integer with the value: X*1000000 + Y*1000 + Z.",
                96,
                4,
                Value::Version(self.header.version),
                ""
            ),
        ]
    }
}
