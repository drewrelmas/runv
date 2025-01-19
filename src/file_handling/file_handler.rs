pub fn unzip_file(zip_path: &str, output_path: Option<&str>) -> zip::result::ZipResult<()> { 
    let file = open_file(zip_path)?;
    // If no output file is provided, remove the .gz extension from the input file name
    let calculated_output_path = output_path.unwrap_or(zip_path.trim_end_matches(".zip"));
    let mut zip_reader = zip::ZipArchive::new(file)?;
    zip_reader.extract(calculated_output_path)?;
    Ok(())
}

pub fn decompress_gz(gz_path: &str, output_path: Option<&str>) -> std::io::Result<()> { 
    let file = open_file(gz_path)?;
    let mut gz_reader = flate2::read::GzDecoder::new(file);
    // If no output file is provided, remove the .gz extension from the input file name
    let calculated_output_path = output_path.unwrap_or(gz_path.trim_end_matches(".gz"));
    let mut output_file = std::fs::File::create(calculated_output_path)?;
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
    fn test_unzip_zip_file() { 
        assert!(unzip_file("tests/data/sometextfile.zip", Some("tests/out/sometextfile")).is_ok()); 
        assert!(std::path::Path::new("tests/out/sometextfile/sometextfile.txt").exists());

        assert!(unzip_file("tests/data/sometextfile.zip", None).is_ok()); 
        assert!(std::path::Path::new("tests/data/sometextfile/sometextfile.txt").exists());
        // clean up
        std::fs::remove_dir_all(std::path::Path::new("tests/data/sometextfile")).unwrap();
    }

    #[test]
    fn test_decompress_gzip_file() { 
        assert!(decompress_gz("tests/data/activity.tcx.gz", Some("tests/out/explicitactivity.tcx")).is_ok()); 
        assert!(std::path::Path::new("tests/out/explicitactivity.tcx").exists());

        assert!(decompress_gz("tests/data/activity.tcx.gz", None).is_ok()); 
        assert!(std::path::Path::new("tests/data/activity.tcx").exists());
        // clean up
        std::fs::remove_file("tests/data/activity.tcx").unwrap();
    }
}