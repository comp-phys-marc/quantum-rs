extern crate dotenv;
extern crate qasm;

use rustsimulationservice::parser;

use std::env;
use std::collections::BTreeMap;
use amiquip::{
    AmqpProperties, Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish,
    QueueDeclareOptions, Result
};


fn main() -> Result<()> {

    dotenv::dotenv();

    let rb_user = env::var("RABBIT_USER").expect("Queue user not configured!");
    let rb_pass = env::var("RABBIT_PASSWORD").expect("Queue password not configured!");
    let rb_port = env::var("RABBIT_PORT").expect("Queue port not configured!");
    let rb_host = env::var("RABBIT_HOST").expect("Queue host not configured!");

    // Open RabbitMQ connection.
    let mut connection = Connection::insecure_open(&format!("amqp://{}:{}@{}:{}", rb_user, rb_pass, rb_host, rb_port))?;

    // Open a channel - None lets the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the default direct exchange.
    let exchange = Exchange::direct(&channel);

    // Declare the queue that will receive RPC requests.
    let queue = channel.queue_declare("rpc_queue", QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Awaiting RPC requests...");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) fib({})", i, body);

                let (reply_to, corr_id) = match (
                    delivery.properties.reply_to(),
                    delivery.properties.correlation_id(),
                ) {
                    (Some(r), Some(c)) => (r.clone(), c.clone()),
                    _ => {
                        println!("received delivery without reply_to or correlation_id");
                        consumer.ack(delivery)?;
                        continue;
                    }
                };

                let mut response:String = "{".to_string();
                let result = parser::execute_qasm(&body);
                let regs: Vec<BTreeMap<usize, usize>> = result.values().cloned().collect();
                let keys:Vec<char> = result.keys().cloned().collect();

                let mut i = 0;
                for reg in regs {
                    response.push_str(&format!(" \"{}\": ", keys[i]));
                    let bitstring:Vec<usize> = reg.values().cloned().collect();
                    for bit in bitstring {
                        response.push_str(bit.to_string().as_str());
                    }
                    response.push_str(",");
                    i += 1;
                }
                response.push_str("}");

                exchange.publish(Publish::with_properties(
                    response.as_bytes(),
                    reply_to,
                    AmqpProperties::default().with_correlation_id(corr_id),
                ))?;
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}