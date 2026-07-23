use crate::structs::StatementConfig;
use std::collections::HashMap;

pub mod au;

type RegionConfigFactory = fn() -> Vec<StatementConfig>;

pub fn get_config_map() -> HashMap<String, StatementConfig> {
    // Register all region config factories here
    let regions: Vec<RegionConfigFactory> = vec![au::get_all_configs];

    let mut config_map = HashMap::new();
    for region_builder in regions {
        for cfg in region_builder() {
            config_map.insert(cfg.key.clone(), cfg);
        }
    }

    config_map
}
