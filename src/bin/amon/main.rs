//! Main entry point for Amon

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use amon::application::APP;

/// Boot Amon
fn main() {
    abscissa_core::boot(&APP);
}
