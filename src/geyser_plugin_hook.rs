use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaTransactionInfoVersions, Result,
};
use log::info;

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
        _transaction: ReplicaTransactionInfoVersions,
        _slot: u64,
    ) -> Result<()> {
        info!("TX Income");
        Ok(())
    }
}
