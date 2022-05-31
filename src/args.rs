use clap::Parser;

/// Experimental, functional configuration file editor
#[derive(Parser, Debug)]
#[clap(name="Configuratrix", about, version)]
pub struct Args {
    /// The path to the configuration file to edit
    #[clap(short='p', long="path")]
    pub config_path: String,

    /// The type of config file you're editing
    #[clap(short='t', long="type")]
    pub config_type: String,

    /// The options to edit
    #[clap(short='o', long="option")]
    pub options: String,
}