use clap::{Parser, Subcommand};
use rust_templates::{Template, TemplateRef, TemplateAssembler, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tokio;

#[derive(Parser)]
#[command(author, version, about = "CLI tool for managing Rust templates")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new template file
    New {
        /// Path to save the template
        #[arg(short, long)]
        output: PathBuf,
        
        /// Template content
        #[arg(short, long)]
        content: Option<String>,
        
        /// Template content from file
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    
    /// Render a template with provided values
    Render {
        /// Path to template file
        #[arg(short, long)]
        template: PathBuf,
        
        /// Key-value pairs for template placeholders (format: key=value)
        #[arg(short, long)]
        values: Vec<String>,
        
        /// Output path for rendered content
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Execute a template using rust-script
    Execute {
        /// Path to template file
        #[arg(short, long)]
        template: PathBuf,
        
        /// Key-value pairs for template placeholders (format: key=value)
        #[arg(short, long)]
        values: Vec<String>,
        
        /// Dependencies to include (format: name=version)
        #[arg(short, long)]
        dependencies: Vec<String>,
    },
    
    /// Combine multiple templates
    Assemble {
        /// Paths to template files
        #[arg(short, long)]
        templates: Vec<PathBuf>,
        
        /// Global key-value pairs for template placeholders
        #[arg(short, long)]
        values: Vec<String>,
        
        /// Output path for combined template
        #[arg(short, long)]
        output: PathBuf,
    },
}

/// Parse key-value pairs from command line arguments
fn parse_key_values(pairs: &[String]) -> HashMap<String, String> {
    pairs.iter()
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect()
}

/// Apply values to a template
fn apply_template_values(template: &mut Template, values: &HashMap<String, String>) -> Result<()> {
    for (key, value) in values {
        template.set(key, value)?;
    }
    Ok(())
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { output, content, file } => {
            let template_content = match (content, file) {
                (Some(content), None) => content,
                (None, Some(file)) => fs::read_to_string(file)?,
                (None, None) => return Err(rust_templates::TemplateError::Parse(
                    "Either content or file must be provided".into()
                )),
                (Some(_), Some(_)) => return Err(rust_templates::TemplateError::Parse(
                    "Cannot provide both content and file".into()
                )),
            };
            
            fs::write(output, template_content)?;
        }
        
        Commands::Render { template, values, output } => {
            let mut template = Template::from_file(template)?;
            let values = parse_key_values(&values);
            apply_template_values(&mut template, &values)?;
            
            let rendered = template.render()?;
            match output {
                Some(path) => fs::write(path, rendered)?,
                None => println!("{}", rendered),
            }
        }
        
        Commands::Execute { template, values, dependencies } => {
            let template = Template::from_file(template)?;
            let mut template_ref = TemplateRef::new(template);
            
            // Add dependencies
            for dep in dependencies {
                template_ref = template_ref.with_dependency(&dep);
            }
            
            // Set values
            let values = parse_key_values(&values);
            apply_template_values(&mut template_ref.template, &values)?;
            
            // Execute and print output
            let output = template_ref.execute().await?;
            println!("{}", output);
        }
        
        Commands::Assemble { templates, values, output } => {
            let mut assembler = TemplateAssembler::new();
            
            // Load all templates
            for path in templates {
                let template = Template::from_file(path)?;
                assembler.add_template(template);
            }
            
            // Set global values
            let values = parse_key_values(&values);
            for (key, value) in values {
                assembler.set_global(&key, &value)?;
            }
            
            // Render and save
            let combined = assembler.render_all()?;
            fs::write(output, combined)?;
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}