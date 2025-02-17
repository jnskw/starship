use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitBranchConfig<'a> {
    pub prefix: &'a str,
    pub symbol: SegmentConfig<'a>,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub branch_name: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitBranchConfig<'a> {
    fn new() -> Self {
        GitBranchConfig {
            prefix: "on ",
            symbol: SegmentConfig::new(" "),
            truncation_length: std::i64::MAX,
            truncation_symbol: "…",
            branch_name: SegmentConfig::default(),
            style: Color::Purple.bold(),
            disabled: false,
        }
    }
}
