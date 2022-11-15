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

use std::io::BufWriter;
use std::io::Write;
// use std::rc::Rc;
/// This matcher just prints the name of the file to stdout.
pub struct Printer {
    delimiter: PrintDelimiter,
    colors: Option<LsColors>,
    // buf: BufWriter<Write>
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

// TODO: this function is performance critical and can probably be optimized
// pub fn print_entry<W: Write>(stdout: &mut W, entry: &DirEntry, config: &Config) {
//     let r = if let Some(ref ls_colors) = config.ls_colors {
//         print_entry_colorized(stdout, entry, config, ls_colors)
//     } else {
//         print_entry_uncolorized(stdout, entry, config)
//     };

//     if let Err(e) = r {
//         if e.kind() == ::std::io::ErrorKind::BrokenPipe {
//             // Exit gracefully in case of a broken pipe (e.g. 'fd ... | head -n 3').
//             ExitCode::Success.exit();
//         } else {
//             print_error(format!("Could not write to output: {}", e));
//             ExitCode::GeneralError.exit();
//         }
//     }
// }

// Display a trailing slash if the path is a directory and the config option is enabled.
// If the path_separator option is set, display that instead.
// The trailing slash will not be colored.
// #[inline]
// fn print_trailing_slash<W: Write>(
//     stdout: &mut W,
//     entry: &DirEntry,
//     config: &Config,
//     style: Option<&Style>,
// ) -> io::Result<()> {
//     if entry.file_type().map_or(false, |ft| ft.is_dir()) {
//         write!(
//             stdout,
//             "{}",
//             style
//                 .map(Style::to_ansi_term_style)
//                 .unwrap_or_default()
//                 .paint(&config.actual_path_separator)
//         )?;
//     }
//     Ok(())
// }

// TODO: this function is performance critical and can probably be optimized
// fn print_entry_colorized<W: Write>(
// fn matches(
//     &self,
//     file_info: &DirEntry,
//     matcher_io: &mut MatcherIO,
    
//     // stdout: &mut W,
//     // entry: &DirEntry,
//     // config: &Config,
//     // ls_colors: &LsColors,
// ) -> bool {
//     let ls_colors = LsColors::from_env().unwrap_or_default();
//     let mut stdout = matcher_io.deps.get_output().borrow_mut();
//     // Split the path between the parent and the last component
//     let mut offset = 0;
//     // let path = file_info.stripped_path(config);
//     let path = file_info.path();
//     let path_str = path.to_string_lossy();

//     if let Some(parent) = path.parent() {
//         offset = parent.to_string_lossy().len();
//         for c in path_str[offset..].chars() {
//             if std::path::is_separator(c) {
//                 offset += c.len_utf8();
//             } else {
//                 break;
//             }
//         }
//     }

//     if offset > 0 {
//         let mut parent_str = Cow::from(&path_str[..offset]);
//         // if let Some(ref separator) =  {
//         //     *parent_str.to_mut() = replace_path_separator(&parent_str, separator);
//         // }

//         let style = ls_colors
//             .style_for_indicator(Indicator::Directory)
//             .map(Style::to_ansi_term_style)
//             .unwrap_or_default();
//         write!(stdout, "{}", style.paint(parent_str))?;
//     }

//     let style = file_info
//         .style(ls_colors)
//         .map(Style::to_ansi_term_style)
//         .unwrap_or_default();
//     write!(stdout, "{}", style.paint(&path_str[offset..]))?;

//     print_trailing_slash(
//         stdout,
//         file_info,
//         config,
//         ls_colors.style_for_indicator(Indicator::Directory),
//     )?;

//     if config.null_separator {
//         write!(stdout, "\0")?;
//     } else {
//         writeln!(stdout)?;
//     }

//     Ok(())
// }

// TODO: this function is performance critical and can probably be optimized
// fn print_entry_uncolorized_base<W: Write>(
//     stdout: &mut W,
//     entry: &DirEntry,
//     config: &Config,
// ) -> io::Result<()> {
//     let separator = if config.null_separator { "\0" } else { "\n" };
//     let path = entry.stripped_path(config);

//     let mut path_string = path.to_string_lossy();
//     if let Some(ref separator) = config.path_separator {
//         *path_string.to_mut() = replace_path_separator(&path_string, separator);
//     }
//     write!(stdout, "{}", path_string)?;
//     print_trailing_slash(stdout, entry, config, None)?;
//     write!(stdout, "{}", separator)
// }

// #[cfg(not(unix))]
// fn print_entry_uncolorized<W: Write>(
//     stdout: &mut W,
//     entry: &DirEntry,
//     config: &Config,
// ) -> io::Result<()> {
//     print_entry_uncolorized_base(stdout, entry, config)
// }

// #[cfg(unix)]
// fn print_entry_uncolorized<W: Write>(
//     stdout: &mut W,
//     entry: &DirEntry,
//     config: &Config,
// ) -> io::Result<()> {
//     use std::os::unix::ffi::OsStrExt;

//     if config.interactive_terminal || config.path_separator.is_some() {
//         // Fall back to the base implementation
//         print_entry_uncolorized_base(stdout, entry, config)
//     } else {
//         // Print path as raw bytes, allowing invalid UTF-8 filenames to be passed to other processes
//         let separator = if config.null_separator { b"\0" } else { b"\n" };
//         stdout.write_all(entry.stripped_path(config).as_os_str().as_bytes())?;
//         print_trailing_slash(stdout, entry, config, None)?;
//         stdout.write_all(separator)
//     }
// }
    // fn matches(&self, file_info: &DirEntry, matcher_io: &mut MatcherIO) -> bool {
    //     let mut out = matcher_io.deps.get_output().borrow_mut();
    //     let path = file_info.path();

    //     let lscolors = LsColors::from_env().unwrap_or_default();

    //     for (component, style) in lscolors.style_for_path_components(path) {
    //         let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
    //         write!(out, "{}", ansi_style.paint(component.to_string_lossy())).unwrap();
    //     }

    //     write!(
    //         out,
    //         "{}",
    //         self.delimiter
    //     )
    //     .unwrap();
    //     out.flush().unwrap();
    //     true
    // }
    
    // IMPORTANT
    // this code works the best right now
    // fn matches(&self, file_info: &DirEntry, matcher_io: &mut MatcherIO) -> bool {
    //     let mut out = matcher_io.deps.get_output().borrow_mut();
    //     let path = file_info.path();

    //     let lscolors = LsColors::from_env().unwrap_or_default();
    //     let mut colored_path = String::new();

    //     for (component, style) in lscolors.style_for_path_components(path) {
    //         let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
    //         write!(&mut colored_path, "{}", ansi_style.paint(component.to_string_lossy())).unwrap();
    //     }

    //     write!(
    //         out,
    //         "{}{}",
    //         colored_path,
    //         self.delimiter
    //     )
    //     .unwrap();
    //     out.flush().unwrap();
    //     true
    // }




    // THIS WORKS
/*
    fn matches(&self, file_info: &DirEntry, matcher_io: &mut MatcherIO) -> bool {
        let mut out = matcher_io.deps.get_output().borrow_mut();
        let path = file_info.path();

        let parent = path.parent();
        let parent_str_orig = parent.unwrap().to_string_lossy();

        let (parent_str, sep) = if parent_str_orig == "" {
            (path.to_string_lossy(), "".to_string())
        } else {
            (parent_str_orig, std::path::MAIN_SEPARATOR.to_string())
        };
        
        let lscolors = self.colors.as_ref();

        let final_str = if let Some(c) = lscolors {
            let style = c.style_for_indicator(Indicator::Directory);
            let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
            let parent_colored = ansi_style.paint(parent_str.to_owned());

            parent_colored.to_string()
        } else {
            parent_str.to_string()
        };

        let fname = path.file_name().unwrap_or_default().to_string_lossy();
        
        write!(
            out,
            "{}{}{}{}",
            final_str,
            sep,
            fname,
            self.delimiter
        )
        .unwrap();
        out.flush().unwrap();
        true
    }
*/

    fn matches(&self, file_info: &DirEntry, matcher_io: &mut MatcherIO) -> bool {
        const END_ANSI: &str = "\u{1b}[0m";

        let mut out = matcher_io.deps.get_output().borrow_mut();
        let path = file_info.path();

        let parent = path.parent();
        let parent_str_orig = parent.unwrap().to_string_lossy();

        let fname = path.file_name().unwrap_or_default().to_string_lossy();
        let lscolors = self.colors.as_ref();


        // let (parent_str, sep) = match (parent_str_orig.is_empty(), fname.is_empty()) {
        //     (true,   true) => (path.to_string_lossy(), "".to_string()),
        //     (true,  false) => (parent_str_orig, "".to_string()),
        //     (false,  true) => (parent_str_orig, "".to_string()),
        //     (false, false) => (parent_str_orig, std::path::MAIN_SEPARATOR.to_string())
        // };
        let (parent_str, sep) = match (parent_str_orig.is_empty(), fname.is_empty()) {
            (true,  true )  => (path.to_string_lossy(),                 "".to_string()),
            (false, false)  => (parent_str_orig, std::path::MAIN_SEPARATOR.to_string()),
            (_,     _    )  => (parent_str_orig,                        "".to_string())
        };
        // let (parent_str, sep) = match (parent_str_orig.is_empty(), fname.is_empty()) {
        //     (true,  true )  => (path.to_string_lossy(),                 "".to_string()),
        //     (false, false)  => (f, std::path::MAIN_SEPARATOR.to_string()),
        //     (_,     _    )  => (parent_str_orig,                        "".to_string())
        // };


        let (color_code_parent, color_code_path) = if let Some(ref c) = lscolors {
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
            &mut out,
            "{}{}{}{}{}{}{}",
            color_code_parent,
            parent_str,
            sep,
            color_code_path,
            fname,
            // if color_code_path == END_ANSI { "" } else { END_ANSI },
            END_ANSI,
            self.delimiter
        )
        .unwrap();
        // out.flush().unwrap();
        true
    }

/*
    fn matches(&self, file_info: &DirEntry, matcher_io: &mut MatcherIO) -> bool {
        let mut out = matcher_io.deps.get_output().borrow_mut();
        let path = file_info.path();

        let lscolors = LsColors::from_env().unwrap_or_default();
        let mut colored_path = String::new();

        for (component, style) in lscolors.style_for_path_components(path) {
            let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
            write!(colored_path, "{}", ansi_style.paint(component.to_string_lossy())).unwrap();
        }

        write!(
            out,
            "{}{}",
            colored_path,
            self.delimiter
        )
        .unwrap();
        out.flush().unwrap();
        true
    }
*/






//     fn matches(&self, file_info: &DirEntry, matcher_io: &mut MatcherIO) -> bool {
//         let mut out = matcher_io.deps.get_output().borrow_mut();
//         // let file_path = file_info.path().to_string_lossy();

//         // writeln!(out, "\x1b[93mError\x1b[0m").unwrap();
//         let lscolors = LsColors::from_env().unwrap_or_default();

//         // let style = lscolors.style_for_path(file_path);

//         // If you want to use `ansi_term`:
//         // let ansi_style = style.map(Style::to_ansi_term_style)
//         //               .unwrap_or_default();
//         // let colored_path = ansi_style.paint(file_path);
//         // let colored_path = style.paint(file_path);
//         // let style = Style::new().bold().on(Colour::Black);
        
//         // let meta = &file_info.metadata();
//         // let s = lscolors.style_for_path_with_metadata(file_info.path(), meta.ok());
//         // let s = lscolors.style_for_path_with_metadata(file_info.path(), meta.ok());
//         // let s = lscolors.style_for_path(file_info.path()).unwrap();
//         // let s = lscolors.style_for_path(file_info.path());
//         // let colored_path = (*s).paint(file_info.path().to_string_lossy());
//         // let colored_path = format!("{s:?}");

//         let s = lscolors.style_for_path(file_info.path());
//         let ansi_style = s.map(Style::to_ansi_term_style)
//                       .unwrap_or_default();
// // println!("{}", ansi_style.paint(path));
//         write!(
//             out,
//             "{}{}",
//             // "\x1b[93m{}{}\x1b[0m",
//             // file_path,
//             // colored_path,
//             ansi_style.paint(file_info.path().to_string_lossy()),
//             self.delimiter
//         )
//         .unwrap();
//         out.flush().unwrap();
//         true
//     }
    // fn matches(&self, file_info: &DirEntry, matcher_io: &mut MatcherIO) -> bool {
    //     let mut out = matcher_io.deps.get_output().borrow_mut();
    //     write!(
    //         out,
    //         "{}{}",
    //         file_info.path().to_string_lossy(),
    //         self.delimiter
    //     )
    //     .unwrap();
    //     out.flush().unwrap();
    //     true
    // }

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
