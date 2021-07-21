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

fn aspect_ratio_validator(s: String) -> Result<(), String> {
    if let Some((w, h)) = s.split_once(':') {
        if let Err(e) = integer_non_zero_validator(w.to_string()) {
            return Err(e);
        }
        if let Err(e) = integer_non_zero_validator(h.to_string()) {
            return Err(e);
        }
        return Ok(());
    } else {
        return Err("wrong format".into());
    }
}

pub fn build_app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("scene")
                .default_value("0")
                .takes_value(true)
                .long("scene"),
        )
        .arg(
            Arg::with_name("width")
                .takes_value(true)
                .long("width")
                .validator(integer_non_zero_validator),
        )
        .arg(
            Arg::with_name("aspect ratio")
                .takes_value(true)
                .long("ratio")
                .validator(aspect_ratio_validator),
        )
        .arg(
            Arg::with_name("samples per pixel")
                .takes_value(true)
                .long("samples")
                .validator(integer_non_zero_validator),
        )
        .arg(Arg::with_name("use bvh").takes_value(false).long("bvh"))
        .arg(Arg::with_name("job").takes_value(true).short("j"))
        .arg(
            Arg::with_name("gpu")
                .takes_value(false)
                .long("gpu")
                .conflicts_with_all(&["use bvh", "job"]),
        )
}
