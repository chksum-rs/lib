use std::fs::{read_dir, File};

use assert_fs::prelude::{FileTouch, FileWriteBin, PathChild};
use assert_fs::TempDir;
use chksum::hash::SHA2_512;
use chksum::{chksum, chksum_with, ArgsBuilder};

mod common;
use common::Result;

#[test]
fn empty_directory_as_path() -> Result {
    let temp_dir = TempDir::new()?;

    let dir = temp_dir.path();
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    Ok(())
}

#[test]
fn empty_directory_as_path_with_args() -> Result {
    let temp_dir = TempDir::new()?;

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = temp_dir.path();
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    }

    Ok(())
}

#[test]
fn empty_directory_as_pathbuf() -> Result {
    let temp_dir = TempDir::new()?;

    let dir = temp_dir.to_path_buf();
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    let dir = &temp_dir.to_path_buf();
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    Ok(())
}

#[test]
fn empty_directory_as_pathbuf_with_args() -> Result {
    let temp_dir = TempDir::new()?;

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = temp_dir.to_path_buf();
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

        let dir = &temp_dir.to_path_buf();
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    }

    Ok(())
}

#[test]
fn empty_directory_as_readdir() -> Result {
    let temp_dir = TempDir::new()?;

    let dir = read_dir(temp_dir.path())?;
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    Ok(())
}

#[test]
fn empty_directory_as_readdir_with_args() -> Result {
    let temp_dir = TempDir::new()?;

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = read_dir(temp_dir.path())?;
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    }

    Ok(())
}

#[test]
fn non_empty_directory_with_empty_file_as_path() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        temp_dir.child("file.txt").touch()?;
        temp_dir
    };

    let dir = temp_dir.path();
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    Ok(())
}

#[test]
fn non_empty_directory_with_empty_file_as_path_with_args() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        temp_dir.child("file.txt").touch()?;
        temp_dir
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = temp_dir.path();
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    }

    Ok(())
}

#[test]
fn non_empty_directory_with_empty_file_as_pathbuf() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        temp_dir.child("file.txt").touch()?;
        temp_dir
    };

    let dir = temp_dir.to_path_buf();
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    let dir = &temp_dir.to_path_buf();
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    Ok(())
}

#[test]
fn non_empty_directory_with_empty_file_as_pathbuf_with_args() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        temp_dir.child("file.txt").touch()?;
        temp_dir
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = temp_dir.to_path_buf();
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

        let dir = &temp_dir.to_path_buf();
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    }

    Ok(())
}

#[test]
fn non_empty_directory_with_empty_file_as_readdir() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        temp_dir.child("file.txt").touch()?;
        temp_dir
    };

    let dir = read_dir(temp_dir.path())?;
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    Ok(())
}

#[test]
fn non_empty_directory_with_empty_file_as_readdir_with_args() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        temp_dir.child("file.txt").touch()?;
        temp_dir
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = read_dir(temp_dir.path())?;
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    }

    Ok(())
}

#[test]
fn non_empty_directory_with_non_empty_file_as_path() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        temp_dir
    };

    let dir = temp_dir.path();
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

    Ok(())
}

#[test]
fn non_empty_directory_with_non_empty_file_as_path_with_args() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        temp_dir
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = temp_dir.path();
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");
    }

    Ok(())
}

#[test]
fn non_empty_directory_with_non_empty_file_as_pathbuf() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        temp_dir
    };

    let dir = temp_dir.to_path_buf();
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

    let dir = &temp_dir.to_path_buf();
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

    Ok(())
}

#[test]
fn non_empty_directory_with_non_empty_file_as_pathbuf_with_args() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        temp_dir
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = temp_dir.to_path_buf();
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

        let dir = &temp_dir.to_path_buf();
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");
    }

    Ok(())
}

#[test]
fn non_empty_directory_with_non_empty_file_as_readdir() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        temp_dir
    };

    let dir = read_dir(temp_dir.path())?;
    let digest = chksum::<SHA2_512, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

    Ok(())
}

#[test]
fn non_empty_directory_with_non_empty_file_as_readdir_with_args() -> Result {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        temp_dir
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = read_dir(temp_dir.path())?;
        let digest = chksum_with::<SHA2_512, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");
    }

    Ok(())
}

#[test]
fn empty_file_as_path() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file
    };

    let file = child.path();
    let digest = chksum::<SHA2_512, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    Ok(())
}

#[test]
fn empty_file_as_path_with_args() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let file = child.path();
        let digest = chksum_with::<SHA2_512, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    }

    Ok(())
}

#[test]
fn empty_file_as_pathbuf() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file
    };

    let file = child.to_path_buf();
    let digest = chksum::<SHA2_512, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    let file = &child.to_path_buf();
    let digest = chksum::<SHA2_512, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    Ok(())
}

#[test]
fn empty_file_as_pathbuf_with_args() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let file = child.to_path_buf();
        let digest = chksum_with::<SHA2_512, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

        let file = &child.to_path_buf();
        let digest = chksum_with::<SHA2_512, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    }

    Ok(())
}

#[test]
fn empty_file_as_file() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file
    };

    let file = File::open(child.path())?;
    let digest = chksum::<SHA2_512, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    let file = &File::open(child.path())?;
    let digest = chksum::<SHA2_512, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    Ok(())
}

#[test]
fn empty_file_as_file_with_args() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let file = File::open(child.path())?;
        let digest = chksum_with::<SHA2_512, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

        let file = &File::open(child.path())?;
        let digest = chksum_with::<SHA2_512, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    }

    Ok(())
}

#[test]
fn non_empty_file_as_path() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        file
    };

    let file = child.path();
    let digest = chksum::<SHA2_512, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

    Ok(())
}

#[test]
fn non_empty_file_as_path_with_args() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        file
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let file = child.path();
        let digest = chksum_with::<SHA2_512, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");
    }

    Ok(())
}

#[test]
fn non_empty_file_as_pathbuf() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        file
    };

    let file = child.to_path_buf();
    let digest = chksum::<SHA2_512, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

    let file = &child.to_path_buf();
    let digest = chksum::<SHA2_512, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

    Ok(())
}

#[test]
fn non_empty_file_as_pathbuf_with_args() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        file
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let file = child.to_path_buf();
        let digest = chksum_with::<SHA2_512, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

        let file = &child.to_path_buf();
        let digest = chksum_with::<SHA2_512, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");
    }

    Ok(())
}

#[test]
fn non_empty_file_as_file() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        file
    };

    let file = File::open(child.path())?;
    let digest = chksum::<SHA2_512, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

    let file = &File::open(child.path())?;
    let digest = chksum::<SHA2_512, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

    Ok(())
}

#[test]
fn non_empty_file_as_file_with_args() -> Result {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        file
    };

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let file = File::open(child.path())?;
        let digest = chksum_with::<SHA2_512, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");

        let file = &File::open(child.path())?;
        let digest = chksum_with::<SHA2_512, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876");
    }

    Ok(())
}
