use zip_structs::zip_central_directory::ZipCDEntry;
use zip_structs::zip_eocd::ZipEOCD;
use zip_structs::zip_local_file_header::ZipLocalFileHeader;
mod test_util;
use test_util::*;

#[test]
fn recover_cd_from_file_header_test() -> anyhow::Result<()> {
    let mut zip_file = open_assets_file("explicit_utf-8.zip")?;
    let eocd = ZipEOCD::from_reader(&mut zip_file)?;
    let mut cd_list = ZipCDEntry::all_from_eocd(&mut zip_file, &eocd)?;
    assert_eq!(cd_list.len(), 1);
    let model_cd = cd_list.pop().unwrap();
    let local_file_header = ZipLocalFileHeader::from_central_directory(&mut zip_file, &model_cd)?;
    let recovered_cd = ZipCDEntry::from_local_file_header(
        &local_file_header,
        model_cd.starting_position_with_signature,
    );
    assert_eq!(&recovered_cd.compressed_size, &model_cd.compressed_size);
    assert_eq!(
        &recovered_cd.compression_method,
        &model_cd.compression_method
    );
    assert_eq!(&recovered_cd.crc32, &model_cd.crc32);
    assert_eq!(&recovered_cd.disk_number_start, &model_cd.disk_number_start);
    // Check whether file or directory
    // A = B qeuals to A ^ B = 0
    assert_eq!(
        (recovered_cd.external_file_attributes ^ model_cd.external_file_attributes) & 0x10,
        0
    );
    assert_eq!(&recovered_cd.extra_field, &model_cd.extra_field);
    assert_eq!(
        &recovered_cd.extra_field_length,
        &model_cd.extra_field_length
    );
    assert_eq!(&recovered_cd.file_comment, &model_cd.file_comment);
    assert_eq!(
        &recovered_cd.file_comment_length,
        &model_cd.file_comment_length
    );
    assert_eq!(&recovered_cd.file_name_length, &model_cd.file_name_length);
    assert_eq!(&recovered_cd.file_name_raw, &model_cd.file_name_raw);
    assert_eq!(
        &recovered_cd.general_purpose_flags,
        &model_cd.general_purpose_flags
    );
    // just guess, don't have to compare
    // assert_eq!(
    //     &recovered_cd.internal_file_attributes,
    //     &model_cd.internal_file_attributes
    // );
    assert_eq!(&recovered_cd.last_mod_date, &model_cd.last_mod_date);
    assert_eq!(&recovered_cd.last_mod_time, &model_cd.last_mod_time);
    assert_eq!(
        &recovered_cd.local_header_position,
        &model_cd.local_header_position
    );
    assert_eq!(
        &recovered_cd.starting_position_with_signature,
        &model_cd.starting_position_with_signature
    );
    assert_eq!(
        &recovered_cd.starting_position_without_signature,
        &model_cd.starting_position_without_signature
    );
    assert_eq!(&recovered_cd.uncompressed_size, &model_cd.uncompressed_size);
    // I don't know how to compare
    // assert_eq!(&recovered_cd.version_made_by, &model_cd.version_made_by);
    assert_eq!(
        &recovered_cd.version_required_to_extract,
        &model_cd.version_required_to_extract
    );
    Ok(())
}
