use std::os::raw::{c_char, c_int};

use rusqlite::ffi;
use rusqlite::vtab::{
    eponymous_only_module, sqlite3_vtab, sqlite3_vtab_cursor, Context, IndexInfo, VTab,
    VTabConnection, VTabCursor, Values,
};
use rusqlite::{to_sqlite_error, Connection, Result};

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn sqlite3_extension_init(
    db: *mut ffi::sqlite3,
    pz_err_msg: *mut *mut c_char,
    p_api: *mut ffi::sqlite3_api_routines,
) -> c_int {
    // SQLITE_EXTENSION_INIT2 equivalent
    unsafe {
        ffi::sqlite3_api = p_api;
    }
    let res = dummy_init(db);
    if let Err(err) = res {
        return unsafe { to_sqlite_error(&err, pz_err_msg) };
    }

    ffi::SQLITE_OK
}

fn dummy_init(db: *mut ffi::sqlite3) -> Result<()> {
    let conn = unsafe { Connection::from_handle(db)? };

    // Dummy virtual table
    let module = eponymous_only_module::<DummyTab>(1);

    #[repr(C)]
    struct DummyTab {
        /// Base class. Must be first
        base: sqlite3_vtab,
    }

    impl VTab for DummyTab {
        type Aux = ();
        type Cursor = DummyTabCursor;

        fn connect(
            _: &mut VTabConnection,
            _aux: Option<&()>,
            _args: &[&[u8]],
        ) -> Result<(String, DummyTab)> {
            let vtab = DummyTab {
                base: sqlite3_vtab::default(),
            };
            Ok(("CREATE TABLE x(value)".to_owned(), vtab))
        }

        fn best_index(&self, info: &mut IndexInfo) -> Result<()> {
            info.set_estimated_cost(1.);
            Ok(())
        }

        fn open(&self) -> Result<DummyTabCursor> {
            Ok(DummyTabCursor::default())
        }
    }

    #[derive(Default)]
    #[repr(C)]
    struct DummyTabCursor {
        /// Base class. Must be first
        base: sqlite3_vtab_cursor,
        /// The rowid
        row_id: i64,
    }

    impl VTabCursor for DummyTabCursor {
        fn filter(
            &mut self,
            _idx_num: c_int,
            _idx_str: Option<&str>,
            _args: &Values<'_>,
        ) -> Result<()> {
            self.row_id = 1;
            Ok(())
        }

        fn next(&mut self) -> Result<()> {
            self.row_id += 1;
            Ok(())
        }

        fn eof(&self) -> bool {
            self.row_id > 1
        }

        fn column(&self, ctx: &mut Context, _: c_int) -> Result<()> {
            ctx.set_result(&self.row_id)
        }

        fn rowid(&self) -> Result<i64> {
            Ok(self.row_id)
        }
    }

    conn.create_module::<DummyTab>("dummy", &module, None)
}
