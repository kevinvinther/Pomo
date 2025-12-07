pub trait Notifier {
    fn send_notification(&self, title: &str, content: &str) -> anyhow::Result<()>;
}
