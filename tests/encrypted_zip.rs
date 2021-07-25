use zip_structs::zip_central_directory::ZipCDEntry;
use zip_structs::zip_eocd::ZipEOCD;
use zip_structs::zip_local_file_header::ZipLocalFileHeader;
mod test_util;
use test_util::*;

#[test]
fn regular_encryption_comparing_test() -> anyhow::Result<()> {
    let mut sjis_zip = open_assets_file("zipcrypto_sjis.zip")?;
    let mut utf8_zip = open_assets_file("zipcrypto_utf-8.zip")?;

    let sjis_eocd = ZipEOCD::from_reader(&mut sjis_zip)?;
    let utf8_eocd = ZipEOCD::from_reader(&mut utf8_zip)?;

    let mut sjis_cd_list = ZipCDEntry::all_from_eocd(&mut sjis_zip, &sjis_eocd)?;
    let mut utf8_cd_list = ZipCDEntry::all_from_eocd(&mut utf8_zip, &utf8_eocd)?;

    assert_eq!(sjis_cd_list.len(), 1);
    assert_eq!(utf8_cd_list.len(), 1);

    let sjis_cd = sjis_cd_list.pop().unwrap();
    let utf8_cd = utf8_cd_list.pop().unwrap();

    assert_eq!(sjis_cd.uncompressed_size, utf8_cd.uncompressed_size);
    assert_eq!(sjis_cd.version_made_by, utf8_cd.version_made_by);
    assert_eq!(
        sjis_cd.version_required_to_extract,
        utf8_cd.version_required_to_extract
    );
    assert_eq!(sjis_cd.compression_method, utf8_cd.compression_method);
    assert_eq!(sjis_cd.crc32, utf8_cd.crc32);
    assert_eq!(sjis_cd.is_encoded_in_utf8(), false);
    assert_eq!(utf8_cd.is_encoded_in_utf8(), true);
    assert_eq!(sjis_cd.is_encrypted_central_directory(), false);
    assert_eq!(sjis_cd.is_encrypted_data(), true);
    assert_eq!(sjis_cd.is_strongly_encrypted_data(), false);
    assert_eq!(utf8_cd.is_encrypted_central_directory(), false);
    assert_eq!(utf8_cd.is_encrypted_data(), true);
    assert_eq!(utf8_cd.is_strongly_encrypted_data(), false);

    let sjis_local_header = ZipLocalFileHeader::from_central_directory(&mut sjis_zip, &sjis_cd)?;
    let utf8_local_header = ZipLocalFileHeader::from_central_directory(&mut utf8_zip, &sjis_cd)?;

    assert_eq!(sjis_local_header.file_name_length, sjis_cd.file_name_length);
    assert_eq!(utf8_local_header.file_name_length, utf8_cd.file_name_length);
    assert_eq!(
        sjis_local_header.uncompressed_size,
        sjis_cd.uncompressed_size
    );
    assert_eq!(
        utf8_local_header.uncompressed_size,
        utf8_cd.uncompressed_size
    );

    return Ok(());
}

/// WinZip-style AES encryption
///
/// https://www.winzip.com/win/en/aes_info.html
///
/// 7-Zip adopts this format
#[test]
fn aes_encryption_comparing_test() -> anyhow::Result<()> {
    let mut sjis_zip = open_assets_file("aes256_sjis.zip")?;
    let mut utf8_zip = open_assets_file("aes256_utf-8.zip")?;

    let sjis_eocd = ZipEOCD::from_reader(&mut sjis_zip)?;
    let utf8_eocd = ZipEOCD::from_reader(&mut utf8_zip)?;

    let mut sjis_cd_list = ZipCDEntry::all_from_eocd(&mut sjis_zip, &sjis_eocd)?;
    let mut utf8_cd_list = ZipCDEntry::all_from_eocd(&mut utf8_zip, &utf8_eocd)?;

    assert_eq!(sjis_cd_list.len(), 1);
    assert_eq!(utf8_cd_list.len(), 1);

    let sjis_cd = sjis_cd_list.pop().unwrap();
    let utf8_cd = utf8_cd_list.pop().unwrap();

    assert_eq!(sjis_cd.uncompressed_size, utf8_cd.uncompressed_size);
    assert_eq!(sjis_cd.version_made_by, utf8_cd.version_made_by);
    assert_eq!(
        sjis_cd.version_required_to_extract,
        utf8_cd.version_required_to_extract
    );
    // Compression method must be 99 (II. B.)
    assert_eq!(sjis_cd.compression_method, 99);
    assert_eq!(utf8_cd.compression_method, 99);
    // CRC must be 0 (II. C.)
    assert_eq!(sjis_cd.crc32, 0);
    assert_eq!(utf8_cd.crc32, 0);
    assert_eq!(sjis_cd.is_encoded_in_utf8(), false);
    assert_eq!(utf8_cd.is_encoded_in_utf8(), true);
    assert_eq!(sjis_cd.is_encrypted_central_directory(), false);
    assert_eq!(utf8_cd.is_encrypted_central_directory(), false);
    // bit #0 (encrypted file) must be true (II. B.)
    assert_eq!(sjis_cd.is_encrypted_data(), true);
    assert_eq!(utf8_cd.is_encrypted_data(), true);
    // bit #6 (strong encryption) must be false
    assert_eq!(sjis_cd.is_strongly_encrypted_data(), false);
    assert_eq!(utf8_cd.is_strongly_encrypted_data(), false);

    let sjis_local_header = ZipLocalFileHeader::from_central_directory(&mut sjis_zip, &sjis_cd)?;
    let utf8_local_header = ZipLocalFileHeader::from_central_directory(&mut utf8_zip, &sjis_cd)?;

    assert_eq!(sjis_local_header.file_name_length, sjis_cd.file_name_length);
    assert_eq!(utf8_local_header.file_name_length, utf8_cd.file_name_length);
    assert_eq!(
        sjis_local_header.uncompressed_size,
        sjis_cd.uncompressed_size
    );
    assert_eq!(
        utf8_local_header.uncompressed_size,
        utf8_cd.uncompressed_size
    );

    return Ok(());
}
