/*  artifact: the requirements tracking tool made for developers
 * Copyright (C) 2017  Garrett Berg <@vitiral, vitiral@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the Lesser GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the Lesser GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 * */

// stdlib
pub use std::process::exit;

// crates
pub use ansi_term::Style;
pub use ansi_term::Colour::{Red, Blue, Green, Yellow};
pub use clap::{Arg, App, SubCommand, ArgMatches, AppSettings as AS, Result as ClapResult};

// module types
pub use ui;
pub use ui::{FmtSettings, FmtArtifact, PercentSearch, SearchSettings};

pub const COLOR: AS = AS::ColorAuto;

#[cfg(windows)]
pub const COLOR_IF_POSSIBLE: bool = false;

#[cfg(not(windows))]
pub const COLOR_IF_POSSIBLE: bool = true;
