use zip_structs::zip_central_directory::ZipCDEntry;
use zip_structs::zip_local_file_header::ZipLocalFileHeader;
mod test_util;
use test_util::*;

static FILE_NAME: &[u8] = "テスト.txt".as_bytes();
static FILE_CONTENT: &[u8] = "テスト".as_bytes();
#[test]
fn direct_read_structures_test() -> anyhow::Result<()> {
    let mut zip_file = open_assets_file("explicit_utf-8.zip")?;
    let local_header = ZipLocalFileHeader::read_and_generate_from_signature(&mut zip_file)?;
    let cd = ZipCDEntry::read_and_generate_from_signature(&mut zip_file)?;
    assert_eq!(local_header.file_name_raw, FILE_NAME);
    assert_eq!(local_header.file_name_length as usize, FILE_NAME.len());
    assert_eq!(local_header.uncompressed_size as usize, FILE_CONTENT.len());
    assert_eq!(local_header.compressed_size as usize, FILE_CONTENT.len());
    assert_eq!(local_header.compressed_data, FILE_CONTENT);
    assert_eq!(cd.file_name_raw, FILE_NAME);
    assert_eq!(cd.file_name_length as usize, FILE_NAME.len());
    assert_eq!(cd.uncompressed_size as usize, FILE_CONTENT.len());
    assert_eq!(cd.compressed_size as usize, FILE_CONTENT.len());
    assert_eq!(cd.local_header_position, 0);
    assert_eq!(local_header.starting_position_with_signature, 0);
    Ok(())
}
