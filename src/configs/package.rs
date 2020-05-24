use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PackageConfig<'a> {
    pub prefix: &'a str,
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub display_private: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PackageConfig<'a> {
    fn new() -> Self {
        PackageConfig {
            prefix: "is ",
            symbol: SegmentConfig::new("📦 "),
            style: Color::Fixed(208).bold(),
            display_private: false,
            disabled: false,
        }
    }
}
