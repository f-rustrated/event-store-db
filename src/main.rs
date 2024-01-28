use serde::{Deserialize, Serialize};

mod event_store_commands;

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    is_rust_a_nice_language: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = "esdb://localhost:2113?tls=false".parse()?;
    let client = eventstore::Client::new(settings)?;

    loop {
        let input = read_user_input("Enter a command: ");

        match input.as_str() {
            "create_stream" => event_store_commands::create_stream(&client, read_user_input("Text: ")).await?,
            "add_to_stream" => event_store_commands::add_to_stream(&client, read_user_input("Text: ")).await?,
            "read_from_stream" => event_store_commands::read_from_stream(&client).await?,
            "read_from_stream_position" => event_store_commands::read_from_stream_position(&client, read_user_input("Position: ")).await?,
            "read_stream_overriding_user_credentials" => event_store_commands::read_stream_overriding_user_credentials(&client).await?,
            "read_from_stream_position_check" => event_store_commands::read_from_stream_position_check(&client).await?,
            "read_stream_backwards" => event_store_commands::read_stream_backwards(&client).await?,
            "read_from_all_stream" => event_store_commands::read_from_all_stream(&client).await?,
            "read_all_overriding_user_credentials" => event_store_commands::read_all_overriding_user_credentials(&client).await?,
            "ignore_system_events" => event_store_commands::ignore_system_events(&client).await?,
            "read_from_all_stream_backwards" => event_store_commands::read_from_all_stream_backwards(&client).await?,
            "filtering_out_system_events" => event_store_commands::filtering_out_system_events(&client).await?,
            "read_from_stream_resolving_link_tos" => event_store_commands::read_from_stream_resolving_link_tos(&client).await?,
            _ => println!("Invalid command")
        }
    }
}

fn read_user_input(text_to_show: &str) -> String {
    println!("{}", text_to_show);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    String::from(input.trim())
}