use chksum::{Args, ArgsBuilder};

#[test]
fn args_new() {
    let args = Args::new();
    assert_eq!(args, Args { chunk_size: None });
}

#[test]
fn args_default() {
    let args = Args::default();
    assert_eq!(args, Args { chunk_size: None });
}

#[test]
fn args_builder_new() {
    let args = ArgsBuilder::new().build();
    assert_eq!(args, Args { chunk_size: None });
}

#[test]
fn args_builder_default() {
    let args = ArgsBuilder::default().build();
    assert_eq!(args, Args { chunk_size: None });
}

#[test]
fn args_builder_chunk_size() {
    for chunk_size in [1, 2, 1024, 9999] {
        let args = ArgsBuilder::new().chunk_size(chunk_size).build();
        assert_eq!(
            args,
            Args {
                chunk_size: Some(chunk_size)
            }
        )
    }
}
