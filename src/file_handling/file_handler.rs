pub fn unzip_file(zip_path: &str, output_dir: &str) -> zip::result::ZipResult<()> { 
    let file = open_file(zip_path)?;
    let mut zip_reader = zip::ZipArchive::new(file)?;
    zip_reader.extract(output_dir)?;
    Ok(())
}

pub fn decompress_gz(gz_path: &str, output_file: &str) -> std::io::Result<()> { 
    let file = open_file(gz_path)?;
    let mut gz_reader = flate2::read::GzDecoder::new(file);
    let mut output_file = std::fs::File::create(output_file)?;
    std::io::copy(&mut gz_reader, &mut output_file)?;
    Ok(())
}

fn open_file(file_path: &str) -> std::io::Result<std::fs::File> { 
    if !std::path::Path::new(file_path).exists() { return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")); }
    Ok(std::fs::File::open(file_path)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_file_notfound() { 
        assert!(open_file("tests/data/somefile").is_err()); 
    }

    #[test]
    fn test_unzip_file() { 
        assert!(unzip_file("tests/data/sometextfile.zip", "tests/out/sometextfile").is_ok()); 
        assert!(std::path::Path::new("tests/out/sometextfile/sometextfile.txt").exists());
    }

    #[test]
    fn test_decompress_gzip_file() { 
        let explicit_result = decompress_gz("tests/data/activity.tcx.gz", "tests/out/explicitactivity.tcx");
        assert!(explicit_result.is_ok()); 
        assert!(std::path::Path::new("tests/out/explicitactivity.tcx").exists());
    }
}