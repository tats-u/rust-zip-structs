use zip_structs::zip_central_directory::ZipCDEntry;
use zip_structs::zip_eocd::ZipEOCD;
use zip_structs::zip_local_file_header::ZipLocalFileHeader;
mod test_util;
use test_util::*;

#[test]
fn find_eocd_test() -> anyhow::Result<()> {
    let mut zip_file = open_assets_file("explicit_utf-8.zip")?;

    let eocd = ZipEOCD::from_reader(&mut zip_file)?;
    assert_eq!(eocd.starting_position_with_signature, 0x6F);
    assert_eq!(eocd.starting_position_without_signature, 0x73);
    assert_eq!(eocd.comment_length, 0);
    assert_eq!(eocd.comment.len(), 0);
    assert_eq!(eocd.eocd_disk_index, 0);
    assert_eq!(eocd.cd_start_disk_index, 0);
    assert_eq!(eocd.n_cd_entries_in_disk, 1);
    assert_eq!(eocd.n_cd_entries, 1);
    assert_eq!(eocd.cd_starting_position, 0x34);
    assert_eq!(
        eocd.cd_size as u64,
        eocd.starting_position_with_signature - eocd.cd_starting_position as u64
    );

    let mut dummy_file = open_assets_file("ness_special_moves.txt")?;
    assert!(ZipEOCD::from_reader(&mut dummy_file).is_err());

    Ok(())
}

#[test]
fn eocd_cd_chain_read_test() -> anyhow::Result<()> {
    let mut zip_file = open_assets_file("explicit_utf-8.zip")?;
    let eocd = ZipEOCD::from_reader(&mut zip_file)?;
    let mut cd_list = ZipCDEntry::all_from_eocd(&mut zip_file, &eocd)?;

    assert_eq!(cd_list.len(), 1);
    let cd = cd_list.pop().unwrap();
    let file_name = String::from_utf8_lossy(&cd.file_name_raw);
    assert_eq!(file_name, "テスト.txt");

    assert!(cd.is_encoded_in_utf8());
    assert_eq!(cd.local_header_position, 0);
    assert_eq!(cd.disk_number_start, 0);
    assert_eq!(file_name.len(), cd.file_name_length as usize);

    Ok(())
}

#[test]
fn eocd_cd_lf_chain_read_test() -> anyhow::Result<()> {
    let mut zip_file = open_assets_file("explicit_utf-8.zip")?;
    let eocd = ZipEOCD::from_reader(&mut zip_file)?;
    let mut cd_list = ZipCDEntry::all_from_eocd(&mut zip_file, &eocd)?;
    assert_eq!(cd_list.len(), 1);
    let cd = cd_list.pop().unwrap();
    let local_file_header = ZipLocalFileHeader::from_central_directory(&mut zip_file, &cd)?;

    let file_name = String::from_utf8_lossy(&local_file_header.file_name_raw);
    assert_eq!(file_name, "テスト.txt");

    assert_eq!(local_file_header.starting_position_with_signature, 0);
    assert_eq!(local_file_header.starting_position_without_signature, 4);

    let expected_content = "テスト";

    // 0 = no compression
    assert_eq!(local_file_header.compression_method, 0);
    assert_eq!(
        local_file_header.compressed_size as usize,
        expected_content.len()
    );
    assert_eq!(
        local_file_header.uncompressed_size as usize,
        expected_content.len()
    );
    assert_eq!(
        String::from_utf8_lossy(&local_file_header.compressed_data),
        expected_content
    );

    Ok(())
}

#[test]
fn legacy_filename_encoding_zip_read_test() -> anyhow::Result<()> {
    let mut zip_file = open_assets_file("windows_sjis.zip")?;
    let eocd = ZipEOCD::from_reader(&mut zip_file)?;
    let mut cd_list = ZipCDEntry::all_from_eocd(&mut zip_file, &eocd)?;

    assert_eq!(cd_list.len(), 1);
    let cd = cd_list.pop().unwrap();
    // テスト.txt in Shift-JIS
    let expected_file_name = [0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0x2e, 0x74, 0x78, 0x74];
    assert_eq!(cd.file_name_length as usize, expected_file_name.len());
    assert_eq!(&cd.file_name_raw, &expected_file_name);

    assert!(!cd.is_encoded_in_utf8());
    assert_eq!(cd.local_header_position, 0);
    assert_eq!(cd.disk_number_start, 0);

    Ok(())
}
