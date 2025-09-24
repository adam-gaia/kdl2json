use anyhow::Result;
use kdl::KdlNode;
use serde_kdl::Node;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let nodes: Vec<KdlNode> = kdl::parse_document(&input)?;
    let node = nodes.get(0).unwrap();
    let node = node.clone();
    let node = Node(node);
    let json = serde_json::to_string_pretty(&node)?;
    println!("{}", json);

    Ok(())
}
