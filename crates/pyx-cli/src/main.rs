use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("pyx: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Vec<String>) -> Result<(), CliError> {
    let command = parse_args(args)?;

    match command {
        Command::Build { input, output } => {
            let source = fs::read_to_string(&input).map_err(|source| CliError::Io {
                path: input.clone(),
                source,
            })?;
            let bytes = pyx_compiler::compile_source(&source)?;
            let output = output.unwrap_or_else(|| default_output_path(&input));

            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent).map_err(|source| CliError::Io {
                    path: parent.to_path_buf(),
                    source,
                })?;
            }

            fs::write(&output, bytes).map_err(|source| CliError::Io {
                path: output.clone(),
                source,
            })?;
            println!("wrote {}", output.display());
            Ok(())
        }
    }
}

fn parse_args(args: Vec<String>) -> Result<Command, CliError> {
    match args.as_slice() {
        [command, input] if command == "build" => Ok(Command::Build {
            input: PathBuf::from(input),
            output: None,
        }),
        [command, input, flag, output] if command == "build" && flag == "-o" => {
            Ok(Command::Build {
                input: PathBuf::from(input),
                output: Some(PathBuf::from(output)),
            })
        }
        [command, input, flag, output] if command == "build" && flag == "--output" => {
            Ok(Command::Build {
                input: PathBuf::from(input),
                output: Some(PathBuf::from(output)),
            })
        }
        [] => Err(CliError::Usage),
        _ => Err(CliError::Usage),
    }
}

fn default_output_path(input: &Path) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default();
    PathBuf::from("build").join(stem).with_extension("pbc")
}

enum Command {
    Build {
        input: PathBuf,
        output: Option<PathBuf>,
    },
}

#[derive(Debug)]
enum CliError {
    Usage,
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Compile(pyx_compiler::CompileError),
}

impl From<pyx_compiler::CompileError> for CliError {
    fn from(source: pyx_compiler::CompileError) -> Self {
        Self::Compile(source)
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Usage => write!(
                f,
                "usage: pyx build <file.juz> [-o <file.pbc>|--output <file.pbc>]"
            ),
            CliError::Io { path, source } => write!(f, "{}: {source}", path.display()),
            CliError::Compile(source) => write!(f, "{source}"),
        }
    }
}

impl std::error::Error for CliError {}
