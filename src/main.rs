use std::error::Error;
use std::ffi::{c_char, CStr};
use std::io::{self, Write};
use std::process;

use clap::Parser as _;

use libxml::bindings::{
    xmlBufferContent, xmlBufferCreate, xmlBufferFree, xmlKeepBlanksDefault, xmlNodeDump,
};
use libxml::parser::Parser;
use libxml::tree::{Document, Node, NodeType};
use libxml::xpath::{Context, Object};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Force input to be parsed as HTML
    #[arg(long, conflicts_with = "xml")]
    html: bool,

    /// Force input to be parsed as XML
    #[arg(long, conflicts_with = "html")]
    xml: bool,

    /// The XPath expression to evaluate
    expression: String,
}

fn pretty_print(doc: &Document, node: &Node) -> String {
    unsafe {
        let buf = xmlBufferCreate();
        xmlNodeDump(
            buf,
            doc.doc_ptr(),
            node.node_ptr(),
            0, // indent level
            1, // pretty print
        );
        let result = xmlBufferContent(buf);
        let c_string = CStr::from_ptr(result as *const c_char);
        let node_string = c_string.to_string_lossy().into_owned();
        xmlBufferFree(buf);
        node_string
    }
}

fn print_results(doc: &Document, results: &Object) -> Result<(), io::Error> {
    for node in results.get_nodes_as_vec() {
        match node.get_type() {
            Some(NodeType::AttributeNode) => writeln!(io::stdout(), "{}", node.get_content())?,
            Some(NodeType::ElementNode) => writeln!(io::stdout(), "{}", &pretty_print(doc, &node))?,
            Some(NodeType::TextNode) => writeln!(io::stdout(), "{}", node.get_content())?,
            _ => unimplemented!("{:?}", node.get_type()),
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();
    let input = io::read_to_string(&mut io::stdin())?;

    let is_html = if args.html {
        true
    } else if args.xml {
        false
    } else {
        // Neither --xml nor --html was given, so sniff the beginning of the input
        // to try and guess the file type
        let trimmed = input.trim_start();
        trimmed.starts_with("<!DOCTYPE html>")
            || trimmed.starts_with("<!DOCTYPE HTML")
            || trimmed.starts_with("<html>")
    };

    unsafe {
        // when parsing, treat whitespace as not significant, so that pretty printing works
        xmlKeepBlanksDefault(0);
    }

    let parser = if is_html {
        Parser::default_html()
    } else {
        Parser::default()
    };

    let doc = parser.parse_string(input)?;

    let context = Context::new(&doc).unwrap(); // FIXME: is this fallible?
    let results = context.evaluate(&args.expression).unwrap_or_else(|_| {
        eprintln!("failed to parse xpath expression");
        process::exit(2);
    });

    match print_results(&doc, &results) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == io::ErrorKind::BrokenPipe {
                Ok(())
            } else {
                Err(e.into())
            }
        }
    }
}
