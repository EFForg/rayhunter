use anyhow::Result;

use crate::MoxeeArgs;

pub async fn install(args: MoxeeArgs) -> Result<()> {
    let data_dir = args.data_dir.or(Some("/cache/rayhunter-data".to_string()));
    crate::orbic_network::install(
        args.admin_ip,
        args.admin_username,
        args.admin_password,
        args.reset_config,
        data_dir,
        args.wifi_ssid.as_deref(),
        args.wifi_password.as_deref(),
    )
    .await
}
