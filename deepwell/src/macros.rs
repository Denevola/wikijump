/*
 * macros.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2022 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

/// Like `std::write!()`, except it asserts the writing succeeded.
///
/// This is done because the only failure mode for writing to a `String`
/// would be insufficient memory, which would cause an abort anyways.
///
/// # See also
/// * [`str_writeln!`](macro.str_writeln.html)
macro_rules! str_write {
    ($dest:expr, $($arg:tt)*) => {{
        use std::fmt::Write;

        write!($dest, $($arg)*).expect("Writing to string failed");
    }};
}

/// Like `std::writeln!()`, except it asserts the writing succeeded.
///
/// This is done because the only failure mode for writing to a `String`
/// would be insufficient memory, which would cause an abort anyways.
///
///
/// # See also
/// * [`str_write!`](macro.str_write.html)
macro_rules! str_writeln {
    ($dest:expr, $($arg:tt)*) => {{
        use std::fmt::Write;

        writeln!($dest, $($arg)*).expect("Writing to string failed");
    }};
}

/// Shortcut for creating a `Cow::Borrowed` from this object.
macro_rules! cow {
    ($s:expr) => {
        Cow::Borrowed($s.as_ref())
    };
}

/// Creates an `Option<Cow::Borrowed>` from this `Option<T>`.
macro_rules! cow_opt {
    ($s:expr) => {
        $s.ref_map(|s| cow!(s))
    };
}
