#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
//
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//
extern crate num;
//
mod enums;
mod support;
//
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// -----
// Tests
// -----
#[cfg(test)]
mod enum_tests {
    use super::*;
    #[test]
    fn codepage() {
        assert_eq!(CodePage::try_from(0), Err(0));
        assert_eq!(CodePage::try_from(20127), Ok(CodePage::ASCII));
        assert_eq!(CodePage::try_from(28591), Ok(CodePage::ISO_8859_1));
        assert_eq!(CodePage::try_from(20866), Ok(CodePage::KOI8_R));
        assert_eq!(CodePage::try_from(874), Ok(Self::WINDOWS_874));
    }
    #[test]
    fn access_flags() {
        assert_eq!(AccessFlags::try_from(0), Err(0));
        assert_eq!(AccessFlags::try_from(1), Ok(AccessFlags::Read));
        assert_eq!(AccessFlags::try_from(2), Ok(AccessFlags::Write));
    }
    #[test]
    fn recovery_flags() {
        assert_eq!(RecoveryFlags::try_from(0), Err(0));
        assert_eq!(
            RecoveryFlags::try_from(1),
            Ok(RecoveryFlags::IgnoreAllocationData)
        );
        assert_eq!(
            RecoveryFlags::try_from(2),
            Ok(RecoveryFlags::ScanForFragments)
        );
    }
    #[test]
    fn file_type() {
        assert_eq!(FileTypes::try_from(0), Err(0));
        assert_eq!(FileTypes::try_from(32), Ok(FileTypes::Bit32));
        assert_eq!(FileTypes::try_from(64), Ok(FileTypes::Bit64));
        assert_eq!(FileTypes::try_from(65), Ok(FileTypes::Bit64Page4k));
    }
    #[test]
    fn content_type() {
        assert_eq!(ContentType::try_from(0), Err(0));
        assert_eq!(ContentType::try_from(97), Ok(ContentType::Pab));
        assert_eq!(ContentType::try_from(112), Ok(ContentType::Pst));
        assert_eq!(ContentType::try_from(111), Ok(ContentType::Ost));
    }
    #[test]
    fn encryption_type() {
        assert_eq!(EncryptionType::try_from(-1), Err(-1));
        assert_eq!(EncryptionType::try_from(0), Ok(EncryptionType::None));
        assert_eq!(EncryptionType::try_from(1), Ok(EncryptionType::High));
        assert_eq!(
            EncryptionType::try_from(2),
            Ok(EncryptionType::Compressible)
        );
    }

    #[test]
    fn item_type() {
        assert_eq!(ItemType::try_from(-1), Err(-1));
        assert_eq!(ItemType::try_from(0), Ok(ItemType::Undefined));
        assert_eq!(ItemType::try_from(3), Ok(ItemType::Attachment));
        assert_eq!(ItemType::try_from(12), Ok(ItemType::EmailSmime));
        assert_eq!(ItemType::try_from(14), Ok(ItemType::Folder));
        assert_eq!(ItemType::try_from(29), Ok(ItemType::Unknown));
    }
    #[test]
    fn attachment_type() {
        assert_eq!(AttachmentType::try_from(-1), Err(-1));
        assert_eq!(AttachmentType::try_from(0), Ok(AttachmentType::Undefined));
        assert_eq!(AttachmentType::try_from(100), Ok(AttachmentType::Data));
        assert_eq!(AttachmentType::try_from(105), Ok(AttachmentType::Item));
        assert_eq!(AttachmentType::try_from(114), Ok(AttachmentType::Reference));
    }
    #[test]
    fn block_type() {
        assert_eq!(BlockType::try_from(0), Err(0));
        assert_eq!(BlockType::try_from(100), Ok(BlockType::Data));
        assert_eq!(BlockType::try_from(112), Ok(BlockType::Page));
    }
    #[test]
    fn map_entry_type() {
        assert_eq!(MapEntryType::try_from(0), Err(0));
        assert_eq!(MapEntryType::try_from(100), Ok(MapEntryType::Numeric));
        assert_eq!(MapEntryType::try_from(115), Ok(MapEntryType::String));
    }
    #[test]
    fn entry_value_flag() {
        assert_eq!(EntryValueFlag::try_from(0), Err(0));
        assert_eq!(
            EntryValueFlag::try_from(1),
            Ok(EntryValueFlag::MatchAnyValue)
        );
        assert_eq!(
            EntryValueFlag::try_from(2),
            Ok(EntryValueFlag::IgnoreNameToIdMap)
        );
    }
    #[test]
    fn error_domain() {
        assert_eq!(ErrorDomain::try_from(0), Err(0));
        assert_eq!(ErrorDomain::try_from(97), Ok(ErrorDomain::Arguments));
        assert_eq!(ErrorDomain::try_from(99), Ok(ErrorDomain::Conversion));
        assert_eq!(ErrorDomain::try_from(105), Ok(ErrorDomain::Input));
        assert_eq!(ErrorDomain::try_from(111), Ok(ErrorDomain::Output));
        assert_eq!(ErrorDomain::try_from(114), Ok(ErrorDomain::Runtime));
    }
    #[test]
    fn argument_error() {
        assert_eq!(ArugmentError::try_from(-1), Err(-1));
        assert_eq!(ArugmentError::try_from(0), Ok(ArugmentError::Generic));
        assert_eq!(ArugmentError::try_from(2), Ok(ArugmentError::Conversion));
        assert_eq!(ArugmentError::try_from(5), Ok(ArugmentError::ValueTooSmall));
        assert_eq!(
            ArugmentError::try_from(7),
            Ok(ArugmentError::ValueOutOfBounds)
        );
        assert_eq!(
            ArugmentError::try_from(9),
            Ok(ArugmentError::ConflictingValue)
        );
    }
    #[test]
    fn conversion_error() {
        assert_eq!(ConversionError::try_from(-1), Err(-1));
        assert_eq!(ConversionError::try_from(0), Ok(ConversionError::Generic));
        assert_eq!(
            ConversionError::try_from(1),
            Ok(ConversionError::InputFailed)
        );
        assert_eq!(
            ConversionError::try_from(2),
            Ok(ConversionError::OutputFailed)
        );
    }
    #[test]
    fn compression_error() {
        assert_eq!(CompressionError::try_from(-1), Err(-1));
        assert_eq!(CompressionError::try_from(0), Ok(CompressionError::Generic));
        assert_eq!(
            CompressionError::try_from(1),
            Ok(CompressionError::InputFailed)
        );
        assert_eq!(
            CompressionError::try_from(2),
            Ok(CompressionError::OutputFailed)
        );
    }
    // Need to fill out the rest of the enums
}
