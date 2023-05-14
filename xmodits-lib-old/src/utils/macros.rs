/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[macro_export]
macro_rules! word {
    ($i:expr) => {
        $i..$i + 2
    };
}

#[macro_export]
macro_rules! dword {
    ($i:expr ) => {
        $i..$i + 4
    };
}

#[macro_export]
macro_rules! long {
    ($i:expr) => {
        $i..$i + 8
    };
}

#[macro_export]
/// slice!(offset, length)
macro_rules! slice {
    ($i:expr, $e:expr) => {
        $i..$i + $e
    };
}
