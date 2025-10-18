use anyhow::Context;
use notify_rust::Notification;

pub fn send_notification(title: &str, content: &str) -> anyhow::Result<()> {
    Notification::new()
        .summary(title)
        .body(content)
        .show()
        .context("Sending notification")?;

    Ok(())
}
