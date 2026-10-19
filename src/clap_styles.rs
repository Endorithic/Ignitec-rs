use clap::builder::styling::AnsiColor;
use clap::builder::styling::Styles;

pub const CLAP_STYLE: Styles = Styles::styled()
    .usage(AnsiColor::BrightCyan.on_default().bold())
    .header(AnsiColor::BrightCyan.on_default().bold())
    .placeholder(AnsiColor::Blue.on_default().bold())
    .literal(AnsiColor::Blue.on_default().bold());
