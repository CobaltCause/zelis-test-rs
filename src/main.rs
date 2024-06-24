#![doc = env!("CARGO_PKG_DESCRIPTION")]

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, stdin, stdout, BufWriter, Read, Write},
    path::PathBuf,
    process::ExitCode,
};

use clap::Parser;
use either::Either::{Left, Right};

/// Command line arguments
#[derive(Parser)]
struct Args {
    /// Specify the input file
    ///
    /// If omitted, `stdin` is used.
    #[arg(long)]
    input: Option<PathBuf>,

    /// Specify the output file
    ///
    /// If omitted, `stdout` is used.
    #[arg(long)]
    output: Option<PathBuf>,
}

fn main() -> ExitCode {
    let Err(e) = try_main() else {
        return ExitCode::SUCCESS;
    };

    eprintln!("error: {e}");

    ExitCode::FAILURE
}

/// Fallible entrypoint
fn try_main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input = args
        .input
        .map(|x| File::open(x).map(Left))
        .transpose()?
        .unwrap_or_else(|| Right(stdin().lock()));

    let output = args
        .output
        .map(|x| {
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(x)
                .map(Left)
        })
        .transpose()?
        .unwrap_or_else(|| Right(stdout().lock()));

    fixing_copy(input, output)?;

    Ok(())
}

/// A writer that swaps one byte for another
///
/// This should be wrapped in a [`BufWriter`] to improve performance because it
/// can call [`Write::write`] on the wrapped writer multiple times per call to
/// [`SwapByteWriter::write`].
struct SwapByteWriter<T> {
    /// The inner type to write to
    inner: T,

    /// The input value to replace
    input: u8,

    /// The value to replace the specified input value with
    output: u8,
}

impl<T> Write for SwapByteWriter<T>
where
    T: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for &byte in buf {
            if byte == self.input {
                self.inner.write_all(&[self.output])?;
            } else {
                self.inner.write_all(&[byte])?;
            }
        }

        // We don't change the amount of bytes written, only *which* bytes were
        // written.
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

/// Copies an input stream to an output stream, replacing `;` with `:`
fn fixing_copy<I, O>(mut input: I, output: O) -> Result<u64, io::Error>
where
    I: Read,
    O: Write,
{
    let mut output = SwapByteWriter {
        inner: BufWriter::new(output),
        input: b';',
        output: b':',
    };

    io::copy(&mut input, &mut output)
}
