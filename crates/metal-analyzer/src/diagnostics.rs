use salsa::Accumulator;

use crate::database::Db;

/// An accumulated value used for internal analyzer debugging.
#[salsa::accumulator]
pub struct DebugDiagnostic(pub String);

impl DebugDiagnostic {
    pub fn push(db: &dyn Db, location: String, content: String) {
        DebugDiagnostic(format!("(DEBUG) IN ({0}): {1}", location, content)).accumulate(db);
    }
}
