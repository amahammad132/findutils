// Copyright 2017 Google Inc.
//
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use walkdir::DirEntry;
use super::{Matcher, MatcherIO};

use lscolors::{Indicator, LsColors};

pub enum PrintDelimiter {
    Newline,
    Null,
}

impl std::fmt::Display for PrintDelimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintDelimiter::Newline => writeln!(f),
            PrintDelimiter::Null => write!(f, "\0"),
        }
    }
}

/// This matcher just prints the name of the file to stdout.
pub struct Printer {
    delimiter: PrintDelimiter,
    colors: Option<LsColors>,
}

impl Printer {
    pub fn new(delimiter: PrintDelimiter) -> Self {
        Self { delimiter, colors: None }
    }

    pub fn new_with_colors(delimiter: PrintDelimiter, colors: Option<LsColors>) -> Self {
        Self { delimiter, colors }
    }
}

impl Matcher for Printer {
    fn matches(&self, file_info: &DirEntry, matcher_io: &mut MatcherIO) -> bool {
        const END_ANSI: &str = "\u{1b}[0m";

        let mut stdout = matcher_io.deps.get_output().borrow_mut();
        let path = file_info.path();

        let parent = path.parent();
        let parent_str_orig = parent.unwrap().to_string_lossy();

        let fname = path.file_name().unwrap_or_default().to_string_lossy();
        let ls_colors = self.colors.as_ref();
        
        let mut loc = [0; 1];
        let path_sep: &str = std::path::MAIN_SEPARATOR.encode_utf8(&mut loc);

        let (parent_str, sep) = match (parent_str_orig.is_empty(), fname.is_empty()) {
            (true , true )  => (path.to_string_lossy(),       ""),
            (false, false)  => (parent_str_orig,        path_sep),
            (_    , _    )  => (parent_str_orig,              "")
        };


        let (color_code_parent, color_code_path) = if let Some(ref c) = ls_colors {
            let style_d = c.style_for_indicator(Indicator::Directory);
            let ansi_style1 = style_d.unwrap().to_ansi_term_style();

            let style_f = c.style_for_path(path);
            let right = if let Some(ansi_style2) = style_f {
                ansi_style2.to_ansi_term_style().prefix().to_string()
            } else {
                END_ANSI.to_string()
            };

            (ansi_style1.prefix().to_string(), right)
        } else {
            ("".to_string(), "".to_string())
        };
        
        write!(
            &mut stdout,
            "{}{}{}{}{}{}{}",
            color_code_parent,
            parent_str,
            sep,
            color_code_path,
            fname,
            if color_code_path != END_ANSI { END_ANSI } else { "" },
            self.delimiter
        )
        .unwrap();
        true
    }

    fn has_side_effects(&self) -> bool {
        true
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::find::matchers::tests::get_dir_entry_for;
    use crate::find::matchers::Matcher;
    use crate::find::tests::fix_up_slashes;
    use crate::find::tests::FakeDependencies;

    #[test]
    fn prints_newline() {
        let abbbc = get_dir_entry_for("./test_data/simple", "abbbc");

        let matcher = Printer::new(PrintDelimiter::Newline);
        let deps = FakeDependencies::new();
        assert!(matcher.matches(&abbbc, &mut deps.new_matcher_io()));
        assert_eq!(
            fix_up_slashes("./test_data/simple/abbbc\n"),
            deps.get_output_as_string()
        );
    }

    #[test]
    fn prints_null() {
        let abbbc = get_dir_entry_for("./test_data/simple", "abbbc");

        let matcher = Printer::new(PrintDelimiter::Null);
        let deps = FakeDependencies::new();
        assert!(matcher.matches(&abbbc, &mut deps.new_matcher_io()));
        assert_eq!(
            fix_up_slashes("./test_data/simple/abbbc\0"),
            deps.get_output_as_string()
        );
    }
}
