use covid_cert_uvci::uvcis_to_graph;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

/// cargo run covid_uvci.txt graph_cypher.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print!("USAGE:\n");
        print!("    [Name of Covid UVCI input file] [Name of Graph Cypher output file]");
        return;
    }
    let infile = &args[1];
    let outfile = &args[2];

    let cert_ids_sweden = lines_from_file(infile);
    let mut graph_output = uvcis_to_graph(&cert_ids_sweden);
    graph_output.push_str("\nRETURN *\n");

    let path = Path::new(outfile);
    let display = path.display();
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(graph_output.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
