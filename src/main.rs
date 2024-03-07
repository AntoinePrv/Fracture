mod segment {
    use std::fmt;

    pub struct Path {
        path: std::path::PathBuf,
        show_home: bool,
    }

    impl Path {
        pub fn new(path: std::path::PathBuf, show_home: bool) -> Self {
            Path { path, show_home }
        }

        fn home_subdir(&self) -> Option<&std::path::Path> {
            let home = std::env::home_dir()?;
            match self.path.strip_prefix(home) {
                Ok(relative) => Some(relative),
                Err(_) => None,
            }
        }

        fn component_to_str(component: std::path::Component) -> std::borrow::Cow<'_, str> {
            component.as_os_str().to_string_lossy()
        }

        pub fn elements(&self) -> impl Iterator<Item = impl fmt::Display + '_> {
            type MaybeStr = Option<std::borrow::Cow<'static, str>>;

            if self.show_home {
                if let Some(subdir) = self.home_subdir() {
                    let home: MaybeStr = Some(std::borrow::Cow::from("~"));
                    return home
                        .into_iter()
                        .chain(subdir.components().map(Self::component_to_str));
                }
            }

            let none: MaybeStr = None;
            self.path.components().count();
            none.into_iter()
                .chain(self.path.components().map(Self::component_to_str))
        }
    }

    pub struct DateTime {
        date: chrono::DateTime<chrono::Local>,
        format: String,
    }

    impl DateTime {
        pub fn new(format: String) -> Self {
             DateTime{
                date: chrono::Local::now(),
                format,
            }
        }
    }
}

mod style {
    use termcolor;

    pub struct MutliSegmentJoiner<T: std::fmt::Display> {
        pub separator: T,
        pub max_elems: usize,
        pub elipsis: T,
    }

    pub struct MutliSegmentStyle {
        pub style: termcolor::ColorSpec,
    }

    impl MutliSegmentStyle {
        pub fn from_colors(fg: termcolor::Color, bg: termcolor::Color) -> MutliSegmentStyle {
            let mut style = termcolor::ColorSpec::new();
            style.set_fg(Some(fg));
            style.set_bg(Some(bg));
            MutliSegmentStyle { style }
        }
    }
}

fn write_multi_segment(
    stream: &mut impl std::io::Write,
    seg: &segment::Path,
    joiner: &style::MutliSegmentJoiner<&str>,
) -> std::io::Result<()> {
    let mut elems = seg.elements().take(joiner.max_elems);
    match elems.next() {
        Some(e) => {
            write!(stream, "{}", e)?;
        }
        None => {
            return Ok(());
        }
    }
    for elem in elems {
        write!(stream, "{}{}", joiner.separator, elem)?;
    }
    Ok(())
}

fn easy(seg: &segment::Path, style: &style::MutliSegmentStyle) -> std::io::Result<()> {
    use termcolor::WriteColor;

    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout.set_color(&style.style)?;

    let res = write_multi_segment(
        &mut stdout,
        seg,
        &style::MutliSegmentJoiner {
            separator: " / ",
            max_elems: 5,
            elipsis: "...",
        },
    );
    stdout.reset()?;
    res
}

fn peasy() -> Option<String> {
    Some(itertools::join(
        segment::Path::new(std::path::PathBuf::from("this/dir/is/long"), true).elements(),
        " > ",
    ))
}

fn main() {
    easy(
        &segment::Path::new(std::env::current_dir().unwrap(), true),
        &style::MutliSegmentStyle::from_colors(
            termcolor::Color::Blue,
            termcolor::Color::Ansi256(19),
        ),
    )
    .unwrap();
    println!("");
    println!("{}", peasy().unwrap());
}
