use std::fs::{read_dir, File};

use assert_fs::prelude::{FileTouch, FileWriteBin, PathChild};
use assert_fs::TempDir;
use chksum::hash::MD5;
use chksum::{chksum, chksum_with, ArgsBuilder};

mod common;
use common::Result;

#[test]
fn empty_directory_as_path() -> Result {
    let temp_dir = TempDir::new()?;

    let dir = temp_dir.path();
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

    Ok(())
}

#[test]
fn empty_directory_as_path_with_args() -> Result {
    let temp_dir = TempDir::new()?;

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = temp_dir.path();
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }

    Ok(())
}

#[test]
fn empty_directory_as_pathbuf() -> Result {
    let temp_dir = TempDir::new()?;

    let dir = temp_dir.to_path_buf();
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

    let dir = &temp_dir.to_path_buf();
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

    Ok(())
}

#[test]
fn empty_directory_as_pathbuf_with_args() -> Result {
    let temp_dir = TempDir::new()?;

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = temp_dir.to_path_buf();
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

        let dir = &temp_dir.to_path_buf();
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }

    Ok(())
}

#[test]
fn empty_directory_as_readdir() -> Result {
    let temp_dir = TempDir::new()?;

    let dir = read_dir(temp_dir.path())?;
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

    Ok(())
}

#[test]
fn empty_directory_as_readdir_with_args() -> Result {
    let temp_dir = TempDir::new()?;

    for chunk_size in [1, 2, 3, 127] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();

        let dir = read_dir(temp_dir.path())?;
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
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
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

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
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
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
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

    let dir = &temp_dir.to_path_buf();
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

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
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

        let dir = &temp_dir.to_path_buf();
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
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
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

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
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
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
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

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
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");
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
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

    let dir = &temp_dir.to_path_buf();
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

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
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

        let dir = &temp_dir.to_path_buf();
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");
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
    let digest = chksum::<MD5, _>(dir)?.to_hex_lowercase();
    assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

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
        let digest = chksum_with::<MD5, _>(dir, &args)?.to_hex_lowercase();
        assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");
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
    let digest = chksum::<MD5, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

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
        let digest = chksum_with::<MD5, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
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
    let digest = chksum::<MD5, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

    let file = &child.to_path_buf();
    let digest = chksum::<MD5, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

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
        let digest = chksum_with::<MD5, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

        let file = &child.to_path_buf();
        let digest = chksum_with::<MD5, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
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
    let digest = chksum::<MD5, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

    let file = &File::open(child.path())?;
    let digest = chksum::<MD5, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

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
        let digest = chksum_with::<MD5, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");

        let file = &File::open(child.path())?;
        let digest = chksum_with::<MD5, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
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
    let digest = chksum::<MD5, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

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
        let digest = chksum_with::<MD5, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");
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
    let digest = chksum::<MD5, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

    let file = &child.to_path_buf();
    let digest = chksum::<MD5, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

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
        let digest = chksum_with::<MD5, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

        let file = &child.to_path_buf();
        let digest = chksum_with::<MD5, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");
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
    let digest = chksum::<MD5, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

    let file = &File::open(child.path())?;
    let digest = chksum::<MD5, _>(file)?.to_hex_lowercase();
    assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

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
        let digest = chksum_with::<MD5, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");

        let file = &File::open(child.path())?;
        let digest = chksum_with::<MD5, _>(file, &args)?.to_hex_lowercase();
        assert_eq!(digest, "8d777f385d3dfec8815d20f7496026dc");
    }

    Ok(())
}
