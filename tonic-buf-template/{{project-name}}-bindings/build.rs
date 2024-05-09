use std::{os::unix::fs::PermissionsExt, path::PathBuf};

use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "../proto";
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir_path = std::path::Path::new(&out_dir);
    let metadata = out_dir_path.metadata()?;
    let permissions = metadata.permissions();
    println!(
        "OUT_DIR={}, is_dir={}, permissions={:#o}",
        out_dir,
        out_dir_path.is_dir(),
        permissions.mode()
    );

    // Build a `FileDescriptorSet`
    let descriptor_path = PathBuf::from(out_dir).join("_descriptor.bin");

    // Grab all the proto files.
    //
    // TEMPLATE_NOTE(canardleteer): You may wish to filter out some files, etc.
    let mut proto_files = vec![];
    for entry in WalkDir::new(proto_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map_or(false, |ext| ext.to_str().unwrap() == "proto")
        })
    {
        proto_files.push(entry.path().to_owned());
    }

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // TEMPLATE_WARNING(canardleteer): Using the default stubs will using
        //                                 Streaming endpoints "harder." You
        //                                 loose some nice prebuilt `type`
        //                                 interfaces, that are used in various
        //                                 tonic tutorial documents.
        //
        //                                 For that reason, I have disabled it
        //                                 for novice framework users.
        .generate_default_stubs(false)
        .file_descriptor_set_path(descriptor_path)
        .compile(&proto_files, &[proto_root])?;
    Ok(())
}
