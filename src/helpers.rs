use anyhow::Context;
use notify_rust::Notification;

pub trait Notifier {
    fn send_notification(&self, title: &str, content: &str) -> anyhow::Result<()>;
}

pub struct DesktopNotifier;

impl Notifier for DesktopNotifier {
    fn send_notification(&self, title: &str, content: &str) -> anyhow::Result<()> {
        Notification::new()
            .summary(title)
            .body(content)
            .show()
            .context("Sending notification")?;

        Ok(())
    }
}
