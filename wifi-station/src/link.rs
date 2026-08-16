//! Link administration via rtnetlink, replacing `ip link`.
//!
//! Only the operations `wifi-station` actually needs: bringing an interface up
//! or down and reading its current state.

use anyhow::{Context, Result};
use futures::TryStreamExt;
use netlink_packet_route::link::LinkFlags;
use rtnetlink::{LinkMessageBuilder, LinkUnspec};

/// Wraps a connection plus its driver task, so the driver is aborted on drop.
struct RtConn {
    handle: rtnetlink::Handle,
    task: tokio::task::JoinHandle<()>,
}

impl RtConn {
    fn open() -> Result<Self> {
        let (connection, handle, _) =
            rtnetlink::new_connection().context("failed to open rtnetlink socket")?;
        let task = tokio::spawn(connection);
        Ok(RtConn { handle, task })
    }
}

impl Drop for RtConn {
    fn drop(&mut self) {
        self.task.abort();
    }
}

/// Bring an interface up or down (`ip link set <name> up|down`).
pub(crate) async fn set_up(name: &str, up: bool) -> Result<()> {
    let conn = RtConn::open()?;
    let builder = LinkMessageBuilder::<LinkUnspec>::new().name(name.to_string());
    let builder = if up { builder.up() } else { builder.down() };

    // `link().set()` is reserved for bridge commands upstream; `change()` is
    // the `ip link set` equivalent.
    conn.handle
        .link()
        .change(builder.build())
        .execute()
        .await
        .with_context(|| format!("failed to bring {name} {}", if up { "up" } else { "down" }))?;
    Ok(())
}

/// Whether an interface currently has `IFF_UP` set.
///
/// Returns `false` when the interface does not exist or cannot be queried,
/// matching the previous behaviour of grepping `ip link show` for `state UP`.
pub(crate) async fn is_up(name: &str) -> bool {
    let Ok(conn) = RtConn::open() else {
        return false;
    };
    let mut links = conn
        .handle
        .link()
        .get()
        .match_name(name.to_string())
        .execute();

    match links.try_next().await {
        Ok(Some(msg)) => msg.header.flags.contains(LinkFlags::Up),
        _ => false,
    }
}
