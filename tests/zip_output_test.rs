use vfs::{MemoryFS, VfsPath};

mod test_util;
use test_util::*;

use zip_structs::zip_central_directory::ZipCDEntry;
use zip_structs::zip_eocd::ZipEOCD;
use zip_structs::zip_local_file_header::ZipLocalFileHeader;

#[test]
fn zip_clone_test() -> anyhow::Result<()> {
    let mut zip_file = open_assets_file("explicit_utf-8.zip")?;
    let eocd = ZipEOCD::from_reader(&mut zip_file)?;
    let mut cd_list = ZipCDEntry::all_from_eocd(&mut zip_file, &eocd)?;
    assert_eq!(cd_list.len(), 1);
    let cd = cd_list.pop().unwrap();
    let local_file_header = ZipLocalFileHeader::from_central_directory(&mut zip_file, &cd)?;

    let vfs_root: VfsPath = MemoryFS::new().into();
    let out_zip_vpath = vfs_root.join("out.zip")?;
    let mut zip_out_vfile = out_zip_vpath.create_file()?;

    let local_file_header_write_size = local_file_header.write(&mut zip_out_vfile)?;
    let cd_write_size = cd.write(&mut zip_out_vfile)?;
    eocd.write(&mut zip_out_vfile)?;
    assert_eq!(
        local_file_header_write_size,
        eocd.cd_starting_position as u64
    );
    assert_eq!(
        local_file_header_write_size + cd_write_size,
        eocd.starting_position_with_signature
    );
    drop(zip_out_vfile);

    let mut zip_reread_vfile = out_zip_vpath.open_file()?;
    let output_zip_eocd = ZipEOCD::from_reader(&mut zip_reread_vfile)?;
    let mut output_zip_cd_list =
        ZipCDEntry::all_from_eocd(&mut zip_reread_vfile, &output_zip_eocd)?;

    assert_eq!(
        output_zip_eocd.starting_position_with_signature,
        eocd.starting_position_with_signature
    );
    assert_eq!(
        output_zip_eocd.cd_starting_position,
        eocd.cd_starting_position
    );
    let output_zip_cd = output_zip_cd_list.pop().unwrap();
    let output_zip_local_header =
        ZipLocalFileHeader::from_central_directory(&mut zip_reread_vfile, &output_zip_cd)?;
    assert_eq!(output_zip_cd.local_header_position,cd.local_header_position);
    assert_eq!(output_zip_cd.file_name_raw, local_file_header.file_name_raw);
    assert_eq!(output_zip_local_header.file_name_raw, cd.file_name_raw);
    assert_eq!(output_zip_local_header.compressed_data, local_file_header.compressed_data);

    Ok(())
}
