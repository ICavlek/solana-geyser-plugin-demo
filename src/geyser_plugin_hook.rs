use std::str::FromStr;

use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaTransactionInfoVersions, Result,
};
use log::info;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub struct GeyserPluginHook {}

impl GeyserPlugin for GeyserPluginHook {
    fn name(&self) -> &'static str {
        "DINAMO"
    }

    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> Result<()> {
        solana_logger::setup_with_default("info");
        println!("DINAMO");
        info!("DINAMO");
        Ok(())
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }

    fn notify_transaction(
        &self,
        transaction: ReplicaTransactionInfoVersions,
        _slot: u64,
    ) -> Result<()> {
        info!("TX Income");
        match transaction {
            ReplicaTransactionInfoVersions::V0_0_1(_) => return Ok(()),
            ReplicaTransactionInfoVersions::V0_0_2(val) => {
                let tx = val.transaction;
                let message = tx.message();
                let account_keys = message.account_keys();
                if account_keys.iter().any(|&key| {
                    key == Pubkey::from_str("BXdJ8cMNpx9NXhDDYGw25LsTg7R8tChvVvayzFcgETmP").unwrap()
                }) {
                    info!("Our TX");
                }
            }
        }
        Ok(())
    }
}
