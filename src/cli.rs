use clap::{App, Arg};

fn integer_non_zero_validator(s: String) -> Result<(), String> {
    match s.parse::<u32>() {
        Ok(samples) => {
            if samples == 0 {
                Err("must not be zero".into())
            } else {
                Ok(())
            }
        }
        Err(_) => return Err("must be a number".into()),
    }
}

#[derive(Clone)]
struct AspectRatioParser;

impl clap::builder::TypedValueParser for AspectRatioParser {
    type Value = (crate::Float, crate::Float);

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        if let Some((w, h)) = value.to_str().unwrap().split_once(':') {
            let w: crate::Float = w.parse::<crate::Float>().map_err(|_| {
                clap::Error::raw(clap::ErrorKind::InvalidValue, "cannot parse width in ratio")
            })?;
            let h: crate::Float = h.parse::<crate::Float>().map_err(|_| {
                clap::Error::raw(
                    clap::ErrorKind::InvalidValue,
                    "cannot parse height in ratio",
                )
            })?;
            if w == 0.0 || h == 0.0 {
                return Err(clap::Error::raw(
                    clap::ErrorKind::InvalidValue,
                    "w and h cannot be zero",
                ));
            }
            Ok((w, h))
        } else {
            Err(clap::Error::raw(
                clap::ErrorKind::InvalidValue,
                "ratio must have format w:h",
            ))
        }
    }
}

fn aspect_ratio_validator(s: String) -> Result<(), String> {
    if let Some((w, h)) = s.split_once(':') {
        integer_non_zero_validator(w.to_string())?;
        integer_non_zero_validator(h.to_string())?;
        return Ok(());
    } else {
        return Err("wrong format".into());
    }
}

pub fn build_app() -> App<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("scene")
                .default_value("0")
                .value_parser(clap::builder::RangedU64ValueParser::<u32>::new().range(0..=8))
                .takes_value(true)
                .long("scene"),
        )
        .arg(
            Arg::with_name("width")
                .takes_value(true)
                .long("width")
                .value_parser(clap::builder::RangedU64ValueParser::<u32>::new().range(1..=100000)),
        )
        .arg(
            Arg::with_name("aspect ratio")
                .takes_value(true)
                .long("ratio")
                .value_parser(AspectRatioParser),
        )
        .arg(
            Arg::with_name("samples per pixel")
                .takes_value(true)
                .long("samples")
                .value_parser(clap::builder::RangedU64ValueParser::<u32>::new().range(1..=100000)),
        )
        .arg(Arg::with_name("use bvh").takes_value(false).long("bvh"))
        .arg(
            Arg::with_name("job")
                .takes_value(true)
                .short('j')
                .value_parser(clap::value_parser!(u16).range(1..)),
        )
}
