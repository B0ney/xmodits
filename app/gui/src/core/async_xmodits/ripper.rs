/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::{Path, PathBuf};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::fs::{File, self, };

use xmodits_lib::wav::Wav;
use xmodits_lib::{load_module, TrackerModule, Error, SampleNamerFunc, XmoditsError};

