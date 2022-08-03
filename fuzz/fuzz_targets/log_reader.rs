#![no_main]
use std::error::Error;

use libfuzzer_sys::fuzz_target;

use rhit::args::Args;
use rhit::BaseContent;
use rhit::FileReader;

fuzz_target!(|data: &[u8]| {
    let _ = fuzz(data);
});

fn fuzz(data: &[u8]) -> Result<(), Box<dyn Error>> {
    let args = Args::default();
    let mut consumer = BaseContent::default();

    let file = memfd_path::InMemFilePath::new(data)?;
    let mut fr = FileReader::new(&[file.as_ref().to_owned()], &args, &mut consumer)?;
    let _ = fr.read_all_files()?;
    drop(file);
    Ok(())
}
