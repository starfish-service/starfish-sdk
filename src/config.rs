use anyhow::Result;

pub fn init<T, DK, DV>(name: Option<&str>, defaults: Vec<(DK, DV)>) -> Result<T>
where
    T: Sized + for<'a> serde::Deserialize<'a>,
    DK: AsRef<str>,
    DV: Into<config::Value>,
{
    let mut settings = config::Config::builder()
        .add_source(config::File::with_name(name.unwrap_or("config")).required(false))
        .add_source(
            config::Environment::with_prefix("APP")
                .try_parsing(true)
                .separator("_")
                .list_separator(" "),
        );

    for (key, value) in defaults {
        settings = settings.set_default(key, value)?;
    }

    let settings = settings.build()?;

    Ok(settings.try_deserialize()?)
}
