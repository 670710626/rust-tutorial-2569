pub trait Notifier {
    fn channel_name(&self) -> &'static str;
    fn send(&self, recipient: &str, message: &str) -> Result<(), String>;
}

pub struct EmailNotifier {
    pub smtp_server: String,
}

impl Notifier for EmailNotifier {
    fn channel_name(&self) -> &'static str {
        "Email (SMTP)"
    }

    fn send(&self, recipient: &str, message: &str) -> Result<(), String> {
        if !recipient.contains('@') {
            return Err(format!("Invalid email address: '{}'", recipient));
        }
        println!("[Email via {}] To: {} | Body: '{}'", self.smtp_server, recipient, message);
        Ok(())
    }
}

pub struct SmsNotifier {
    pub country_code: String,
}

impl Notifier for SmsNotifier {
    fn channel_name(&self) -> &'static str {
        "SMS Gateway"
    }

    fn send(&self, recipient: &str, message: &str) -> Result<(), String> {
        if recipient.len() < 9 {
            return Err(format!("Invalid phone number: '{}'", recipient));
        }
        println!("[SMS +{}] To: {} | SMS: '{}'", self.country_code, recipient, message);
        Ok(())
    }
}

pub struct DiscordWebhookNotifier {
    pub bot_name: String,
}

impl Notifier for DiscordWebhookNotifier {
    fn channel_name(&self) -> &'static str {
        "Discord Webhook"
    }

    fn send(&self, recipient: &str, message: &str) -> Result<(), String> {
        println!("[Discord Bot: {}] Channel #{}: '{}'", self.bot_name, recipient, message);
        Ok(())
    }
}

// 1. Static Dispatch: ส่งข้อความด่วนแบบ Compile-time Direct Call
pub fn send_urgent_static<N: Notifier>(notifier: &N, recipient: &str, alert: &str) {
    println!(">>> [STATIC DISPATCH - Inlined Code] <<<");
    match notifier.send(recipient, alert) {
        Ok(_) => println!("Successfully delivered via {}", notifier.channel_name()),
        Err(e) => eprintln!("Failed to deliver: {}", e),
    }
    println!();
}

// 2. Dynamic Dispatch: ศูนย์กระจายข่าวสารผ่าน Vec<Box<dyn Notifier>>
pub struct NotificationBroadcaster {
    channels: Vec<Box<dyn Notifier>>,
}

impl NotificationBroadcaster {
    pub fn new() -> Self {
        Self { channels: Vec::new() }
    }

    pub fn register_channel(&mut self, channel: Box<dyn Notifier>) {
        self.channels.push(channel);
    }

    pub fn broadcast(&self, recipient: &str, message: &str) {
        println!(">>> [DYNAMIC DISPATCH - Broadcasting to {} Channels] <<<", self.channels.len());
        for channel in &self.channels {
            match channel.send(recipient, message) {
                Ok(_) => println!("✔ [{}] Delivered successfully", channel.channel_name()),
                Err(e) => println!("✘ [{}] Error: {}", channel.channel_name(), e),
            }
        }
        println!();
    }
}

fn main() {
    let email = EmailNotifier {
        smtp_server: "smtp.office365.com".to_string(),
    };

    // 1. Static Dispatch
    send_urgent_static(&email, "admin@university.ac.th", "High CPU Load Detected!");

    // 2. Dynamic Dispatch (Heterogeneous Collection)
    let mut broadcaster = NotificationBroadcaster::new();
    broadcaster.register_channel(Box::new(email));
    broadcaster.register_channel(Box::new(SmsNotifier {
        country_code: "66".to_string(),
    }));
    broadcaster.register_channel(Box::new(DiscordWebhookNotifier {
        bot_name: "SecurityGuard-AI".to_string(),
    }));

    broadcaster.broadcast("student_team@su.ac.th", "Rust Tutorial Project Deadline: 4 Oct 24:00!");
}

