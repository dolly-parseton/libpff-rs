use std::convert::TryFrom;
//
pub enum CodePage {
    ASCII = 20127,
    ISO_8859_1 = 28591,
    ISO_8859_2 = 28592,
    ISO_8859_3 = 28593,
    ISO_8859_4 = 28594,
    ISO_8859_5 = 28595,
    ISO_8859_6 = 28596,
    ISO_8859_7 = 28597,
    ISO_8859_8 = 28598,
    ISO_8859_9 = 28599,
    ISO_8859_10 = 28600,
    ISO_8859_11 = 28601,
    ISO_8859_13 = 28603,
    ISO_8859_14 = 28604,
    ISO_8859_15 = 28605,
    ISO_8859_16 = 28606,
    KOI8_R = 20866,
    KOI8_U = 21866,
    WINDOWS_874 = 874,
    WINDOWS_932 = 932,
    WINDOWS_936 = 936,
    WINDOWS_949 = 949,
    WINDOWS_950 = 950,
    WINDOWS_1250 = 1250,
    WINDOWS_1251 = 1251,
    WINDOWS_1252 = 1252,
    WINDOWS_1253 = 1253,
    WINDOWS_1254 = 1254,
    WINDOWS_1255 = 1255,
    WINDOWS_1256 = 1256,
    WINDOWS_1257 = 1257,
    WINDOWS_1258 = 1258,
}

impl TryFrom<i32> for CodePage {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            20127 => Ok(Self::ASCII),
            28591 => Ok(Self::ISO_8859_1),
            28592 => Ok(Self::ISO_8859_2),
            28593 => Ok(Self::ISO_8859_3),
            28594 => Ok(Self::ISO_8859_4),
            28595 => Ok(Self::ISO_8859_5),
            28596 => Ok(Self::ISO_8859_6),
            28597 => Ok(Self::ISO_8859_7),
            28598 => Ok(Self::ISO_8859_8),
            28599 => Ok(Self::ISO_8859_9),
            28600 => Ok(Self::ISO_8859_10),
            28601 => Ok(Self::ISO_8859_11),
            28603 => Ok(Self::ISO_8859_13),
            28604 => Ok(Self::ISO_8859_14),
            28605 => Ok(Self::ISO_8859_15),
            28606 => Ok(Self::ISO_8859_16),
            20866 => Ok(Self::KOI8_R),
            21866 => Ok(Self::KOI8_U),
            874 => Ok(Self::WINDOWS_874),
            932 => Ok(Self::WINDOWS_932),
            936 => Ok(Self::WINDOWS_936),
            949 => Ok(Self::WINDOWS_949),
            950 => Ok(Self::WINDOWS_950),
            1250 => Ok(Self::WINDOWS_1250),
            1251 => Ok(Self::WINDOWS_1251),
            1252 => Ok(Self::WINDOWS_1252),
            1253 => Ok(Self::WINDOWS_1253),
            1254 => Ok(Self::WINDOWS_1254),
            1255 => Ok(Self::WINDOWS_1255),
            1256 => Ok(Self::WINDOWS_1256),
            1257 => Ok(Self::WINDOWS_1257),
            1258 => Ok(Self::WINDOWS_1258),
            _ => Err(v),
        }
    }
}
//
pub enum AccessFlags {
    Read = 1,
    Write = 2,
}

impl TryFrom<i32> for AccessFlags {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Read),
            2 => Ok(Self::Write),
            _ => Err(v),
        }
    }
}
//
pub enum RecoveryFlags {
    IgnoreAllocationData = 1,
    ScanForFragments = 2,
}

impl TryFrom<i32> for RecoveryFlags {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::IgnoreAllocationData),
            2 => Ok(Self::ScanForFragments),
            _ => Err(v),
        }
    }
}
//
pub enum FileTypes {
    Bit32 = 32,
    Bit64 = 64,
    Bit64Page4k = 65,
}

impl TryFrom<i32> for FileTypes {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            32 => Ok(Self::Bit32),
            64 => Ok(Self::Bit64),
            65 => Ok(Self::Bit64Page4k),
            _ => Err(v),
        }
    }
}
//
pub enum ContentType {
    Pab = 97,
    Pst = 112,
    Ost = 111,
}

impl TryFrom<i32> for ContentType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            97 => Ok(Self::Pab),
            112 => Ok(Self::Pst),
            111 => Ok(Self::Ost),
            _ => Err(v),
        }
    }
}
//
pub enum EncryptionType {
    None = 0,
    Compressible = 1,
    High = 2,
}

impl TryFrom<i32> for EncryptionType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Compressible),
            2 => Ok(Self::High),
            _ => Err(v),
        }
    }
}
//
pub enum ItemType {
    Undefined = 0,
    Activity = 1,
    Appointment = 2,
    Attachment = 3,
    Attachments = 4,
    Common = 5,
    Configuration = 6,
    ConflictMessage = 7,
    Contact = 8,
    DistributionList = 9,
    Document = 10,
    Email = 11,
    EmailSmime = 12,
    Fax = 13,
    Folder = 14,
    Meeting = 15,
    Mms = 16,
    Note = 17,
    PostingNote = 18,
    Recipients = 19,
    RssFeed = 20,
    Sharing = 21,
    Sms = 22,
    SubAssociatedContents = 23,
    SubFolders = 24,
    SubMessages = 25,
    Task = 26,
    TaskRequest = 27,
    Voicemail = 28,
    Unknown = 29,
}

impl TryFrom<i32> for ItemType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Undefined),
            1 => Ok(Self::Activity),
            2 => Ok(Self::Appointment),
            3 => Ok(Self::Attachment),
            4 => Ok(Self::Attachments),
            5 => Ok(Self::Common),
            6 => Ok(Self::Configuration),
            7 => Ok(Self::ConflictMessage),
            8 => Ok(Self::Contact),
            9 => Ok(Self::DistributionList),
            10 => Ok(Self::Document),
            11 => Ok(Self::Email),
            12 => Ok(Self::EmailSmime),
            13 => Ok(Self::Fax),
            14 => Ok(Self::Folder),
            15 => Ok(Self::Meeting),
            16 => Ok(Self::Mms),
            17 => Ok(Self::Note),
            18 => Ok(Self::PostingNote),
            19 => Ok(Self::Recipients),
            20 => Ok(Self::RssFeed),
            21 => Ok(Self::Sharing),
            22 => Ok(Self::Sms),
            23 => Ok(Self::SubAssociatedContents),
            24 => Ok(Self::SubFolders),
            25 => Ok(Self::SubMessages),
            26 => Ok(Self::Task),
            27 => Ok(Self::TaskRequest),
            28 => Ok(Self::Voicemail),
            29 => Ok(Self::Unknown),
            _ => Err(v),
        }
    }
}
//
pub enum AttachmentType {
    Undefined = 0,
    Data = 100,
    Item = 105,
    Reference = 114,
}

impl TryFrom<i32> for AttachmentType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Undefined),
            100 => Ok(Self::Data),
            105 => Ok(Self::Item),
            114 => Ok(Self::Reference),
            _ => Err(v),
        }
    }
}
//
pub enum BlockType {
    Data = 100,
    Page = 112,
}

impl TryFrom<i32> for BlockType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            100 => Ok(Self::Data),
            112 => Ok(Self::Page),
            _ => Err(v),
        }
    }
}
//
pub enum MapEntryType {
    Numeric = 110,
    String = 115,
}

impl TryFrom<i32> for MapEntryType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            110 => Ok(Self::Numeric),
            115 => Ok(Self::String),
            _ => Err(v),
        }
    }
}
//
pub enum EntryValueFlag {
    MatchAnyValue = 1,
    IgnoreNameToIdMap = 2,
}

impl TryFrom<i32> for EntryValueFlag {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::MatchAnyValue),
            2 => Ok(Self::IgnoreNameToIdMap),
            _ => Err(v),
        }
    }
}
//
pub enum ErrorDomain {
    Arguments = 97,
    Conversion = 99,
    Compression = 67,
    Io = 73,
    Input = 105,
    Memory = 109,
    Output = 111,
    Runtime = 114,
}

impl TryFrom<i32> for ErrorDomain {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            97 => Ok(Self::Arguments),
            99 => Ok(Self::Conversion),
            67 => Ok(Self::Compression),
            73 => Ok(Self::Io),
            105 => Ok(Self::Input),
            109 => Ok(Self::Memory),
            111 => Ok(Self::Output),
            114 => Ok(Self::Runtime),
            _ => Err(v),
        }
    }
}
//
pub enum ArgumentError {
    Generic = 0,
    InvalidValue = 1,
    ValueLessThanZero = 2,
    ValueZeroOrLess = 3,
    ValueExceedsMaximum = 4,
    ValueTooSmall = 5,
    ValueTooLarge = 6,
    ValueOutOfBounds = 7,
    UnsupportedValue = 8,
    ConflictingValue = 9,
}

impl TryFrom<i32> for ArgumentError {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Generic),
            1 => Ok(Self::InvalidValue),
            2 => Ok(Self::ValueLessThanZero),
            3 => Ok(Self::ValueZeroOrLess),
            4 => Ok(Self::ValueExceedsMaximum),
            5 => Ok(Self::ValueTooSmall),
            6 => Ok(Self::ValueTooLarge),
            7 => Ok(Self::ValueOutOfBounds),
            8 => Ok(Self::UnsupportedValue),
            9 => Ok(Self::ConflictingValue),
            _ => Err(v),
        }
    }
}
//
pub enum ConversionError {
    Generic = 0,
    InputFailed = 1,
    OutputFailed = 2,
}

impl TryFrom<i32> for ConversionError {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Generic),
            1 => Ok(Self::InputFailed),
            2 => Ok(Self::OutputFailed),
            _ => Err(v),
        }
    }
}
//
pub enum CompressionError {
    Generic = 0,
    CompressFailed = 1,
    DecompressFailed = 2,
}

impl TryFrom<i32> for CompressionError {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Generic),
            1 => Ok(Self::CompressFailed),
            2 => Ok(Self::DecompressFailed),
            _ => Err(v),
        }
    }
}
//
pub enum IoError {
    Generic = 0,
    OpenFailed = 1,
    CloseFailed = 2,
    SeekFailed = 3,
    ReadFailed = 4,
    WriteFailed = 5,
    AccessDenied = 6,
    InvalidResource = 7,
    IoctlFailed = 8,
    UnlinkFailed = 9,
}

impl TryFrom<i32> for IoError {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Generic),
            1 => Ok(Self::OpenFailed),
            2 => Ok(Self::CloseFailed),
            3 => Ok(Self::SeekFailed),
            4 => Ok(Self::ReadFailed),
            5 => Ok(Self::WriteFailed),
            6 => Ok(Self::AccessDenied),
            7 => Ok(Self::InvalidResource),
            8 => Ok(Self::IoctlFailed),
            9 => Ok(Self::UnlinkFailed),
            _ => Err(v),
        }
    }
}
//
pub enum InputError {
    Generic = 0,
    InvalidData = 1,
    SignatureMismatch = 2,
    ChecksumMismatch = 3,
    ValueMismatch = 4,
}

impl TryFrom<i32> for InputError {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Generic),
            1 => Ok(Self::InvalidData),
            2 => Ok(Self::SignatureMismatch),
            3 => Ok(Self::ChecksumMismatch),
            4 => Ok(Self::ValueMismatch),
            _ => Err(v),
        }
    }
}
//
pub enum MemoryError {
    Generic = 0,
    Insufficient = 1,
    CopyFailed = 2,
    SetFailed = 3,
}

impl TryFrom<i32> for MemoryError {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Generic),
            1 => Ok(Self::Insufficient),
            2 => Ok(Self::CopyFailed),
            3 => Ok(Self::SetFailed),
            _ => Err(v),
        }
    }
}
//
pub enum OutputError {
    Generic = 0,
    InsufficientSpace = 1,
}

impl TryFrom<i32> for OutputError {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Generic),
            1 => Ok(Self::InsufficientSpace),
            _ => Err(v),
        }
    }
}
//
pub enum RuntimeError {
    Generic = 0,
    ValueMissing = 1,
    ValueAlreadySet = 2,
    InitializeFailed = 3,
    ResizeFailed = 4,
    FinalizeFailed = 5,
    GetFailed = 6,
    SetFailed = 7,
    AppendFailed = 8,
    CopyFailed = 9,
    RemoveFailed = 10,
    PrintFailed = 11,
    ValueOutOfBounds = 12,
    ValueExceedsMaximum = 13,
    UnsupportedValue = 14,
    AbortRequested = 15,
}

impl TryFrom<i32> for RuntimeError {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Generic),
            1 => Ok(Self::ValueMissing),
            2 => Ok(Self::ValueAlreadySet),
            3 => Ok(Self::InitializeFailed),
            4 => Ok(Self::ResizeFailed),
            5 => Ok(Self::FinalizeFailed),
            6 => Ok(Self::GetFailed),
            7 => Ok(Self::SetFailed),
            8 => Ok(Self::AppendFailed),
            9 => Ok(Self::CopyFailed),
            10 => Ok(Self::RemoveFailed),
            11 => Ok(Self::PrintFailed),
            12 => Ok(Self::ValueOutOfBounds),
            13 => Ok(Self::ValueExceedsMaximum),
            14 => Ok(Self::UnsupportedValue),
            15 => Ok(Self::AbortRequested),
            _ => Err(v),
        }
    }
}
//
pub enum AttachmentMethod {
    None = 0,
    ByValue = 1,
    ByReference = 2,
    ByReferenceResolve = 3,
    ByReferenceOnly = 4,
    EmbeddedMessage = 5,
    Ole = 6,
}

impl TryFrom<i32> for AttachmentMethod {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::ByValue),
            2 => Ok(Self::ByReference),
            3 => Ok(Self::ByReferenceResolve),
            4 => Ok(Self::ByReferenceOnly),
            5 => Ok(Self::EmbeddedMessage),
            6 => Ok(Self::Ole),
            _ => Err(v),
        }
    }
}
//
pub enum MessageFlag {
    Read = 1,
    Unmodified = 2,
    Submit = 4,
    Unsent = 8,
    HasAttachments = 16,
    FromMe = 32,
    Associated = 64,
    Resend = 128,
    RnPending = 256,
    NrnPending = 512,
}

impl TryFrom<i32> for MessageFlag {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Read),
            2 => Ok(Self::Unmodified),
            4 => Ok(Self::Submit),
            8 => Ok(Self::Unsent),
            16 => Ok(Self::HasAttachments),
            32 => Ok(Self::FromMe),
            64 => Ok(Self::Associated),
            128 => Ok(Self::Resend),
            256 => Ok(Self::RnPending),
            512 => Ok(Self::NrnPending),
            _ => Err(v),
        }
    }
}
//
pub enum MessageImportanceType {
    Low = 0,
    Normal = 1,
    High = 2,
}

impl TryFrom<i32> for MessageImportanceType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Low),
            1 => Ok(Self::Normal),
            2 => Ok(Self::High),
            _ => Err(v),
        }
    }
}
//
pub enum MessagePriorityType {
    NonUrgent = -1,
    Normal = 0,
    Urgent = 1,
}

impl TryFrom<i32> for MessagePriorityType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            -1 => Ok(Self::NonUrgent),
            0 => Ok(Self::Normal),
            1 => Ok(Self::Urgent),
            _ => Err(v),
        }
    }
}
//
pub enum MessageSensitivityType {
    None = 0,
    Personal = 1,
    Private = 2,
    Confidential = 3,
}

impl TryFrom<i32> for MessageSensitivityType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Personal),
            2 => Ok(Self::Private),
            3 => Ok(Self::Confidential),
            _ => Err(v),
        }
    }
}
//
pub enum MessageStatusFlag {
    Highlighted = 1,
    Tagged = 2,
    Hidden = 4,
    Deleted = 8,
    Draft = 256,
    Answered = 512,
    RemoteDownload = 4096,
    RemoteDeleted = 8192,
}

impl TryFrom<i32> for MessageStatusFlag {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Highlighted),
            2 => Ok(Self::Tagged),
            4 => Ok(Self::Hidden),
            8 => Ok(Self::Deleted),
            256 => Ok(Self::Draft),
            512 => Ok(Self::Answered),
            4096 => Ok(Self::RemoteDownload),
            8192 => Ok(Self::RemoteDeleted),
            _ => Err(v),
        }
    }
}
//
pub enum MessageStoreValidFolderMasks {
    Subtree = 1,
    Inbox = 2,
    Outbox = 4,
    Wastebox = 8,
    Sentmail = 16,
    Views = 32,
    CommonViews = 64,
    Finder = 128,
}

impl TryFrom<i32> for MessageStoreValidFolderMasks {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Subtree),
            2 => Ok(Self::Inbox),
            4 => Ok(Self::Outbox),
            8 => Ok(Self::Wastebox),
            16 => Ok(Self::Sentmail),
            32 => Ok(Self::Views),
            64 => Ok(Self::CommonViews),
            128 => Ok(Self::Finder),
            _ => Err(v),
        }
    }
}
//
pub enum RecipientType {
    Originator = 0,
    To = 1,
    Cc = 2,
    Bcc = 3,
}

impl TryFrom<i32> for RecipientType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Originator),
            1 => Ok(Self::To),
            2 => Ok(Self::Cc),
            3 => Ok(Self::Bcc),
            _ => Err(v),
        }
    }
}
//
pub enum ValueType {
    Unspecified = 0,
    Null = 1,
    Integer16bitSigned = 2,
    Integer32bitSigned = 3,
    Float32bit = 4,
    Double64bit = 5,
    Currency = 6,
    Floatingtime = 7,
    Error = 10,
    Boolean = 11,
    Object = 13,
    Integer64bitSigned = 20,
    StringAscii = 30,
    StringUnicode = 31,
    Filetime = 64,
    Guid = 72,
    ServerIdentifier = 251,
    Restriction = 253,
    RuleAction = 254,
    BinaryData = 258,
    MultiValueInteger16bitSigned = 4098,
    MultiValueInteger32bitSigned = 4099,
    MultiValueFloat32bit = 4100,
    MultiValueDouble64bit = 4101,
    MultiValueCurrency = 4102,
    MultiValueFloatingtime = 4103,
    MultiValueInteger64bitSigned = 4116,
    MultiValueStringAscii = 4126,
    MultiValueStringUnicode = 4127,
    MultiValueFiletime = 4160,
    MultiValueGuid = 4168,
    MultiValueBinaryData = 4354,
}

impl TryFrom<i32> for ValueType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, i32> {
        match v {
            0 => Ok(Self::Unspecified),
            1 => Ok(Self::Null),
            2 => Ok(Self::Integer16bitSigned),
            3 => Ok(Self::Integer32bitSigned),
            4 => Ok(Self::Float32bit),
            5 => Ok(Self::Double64bit),
            6 => Ok(Self::Currency),
            7 => Ok(Self::Floatingtime),
            10 => Ok(Self::Error),
            11 => Ok(Self::Boolean),
            13 => Ok(Self::Object),
            20 => Ok(Self::Integer64bitSigned),
            30 => Ok(Self::StringAscii),
            31 => Ok(Self::StringUnicode),
            64 => Ok(Self::Filetime),
            72 => Ok(Self::Guid),
            251 => Ok(Self::ServerIdentifier),
            253 => Ok(Self::Restriction),
            254 => Ok(Self::RuleAction),
            258 => Ok(Self::BinaryData),
            4098 => Ok(Self::MultiValueInteger16bitSigned),
            4099 => Ok(Self::MultiValueInteger32bitSigned),
            4100 => Ok(Self::MultiValueFloat32bit),
            4101 => Ok(Self::MultiValueDouble64bit),
            4102 => Ok(Self::MultiValueCurrency),
            4103 => Ok(Self::MultiValueFloatingtime),
            4116 => Ok(Self::MultiValueInteger64bitSigned),
            4126 => Ok(Self::MultiValueStringAscii),
            4127 => Ok(Self::MultiValueStringUnicode),
            4160 => Ok(Self::MultiValueFiletime),
            4168 => Ok(Self::MultiValueGuid),
            4354 => Ok(Self::MultiValueBinaryData),
            _ => Err(v),
        }
    }
}
//
pub enum EntryType {
    MessageImportance = 23,
    MessageClass = 26,
    MessagePriority = 38,
    MessageSensitivity = 54,
    MessageSubject = 55,
    MessageClientSubmitTime = 57,
    MessageSentRepresentingSearchKey = 59,
    MessageReceivedByEntryIdentifier = 63,
    MessageReceivedByName = 64,
    MessageSentRepresentingEntryIdentifier = 65,
    MessageSentRepresentingName = 66,
    MessageReceivedRepresentingEntryIdentifier = 67,
    MessageReceivedRepresentingName = 68,
    MessageReplyRecipientEntries = 79,
    MessageReplyRecipientNames = 80,
    MessageReceivedBySearchKey = 81,
    MessageReceivedRepresentingSearchKey = 82,
    MessageSentRepresentingAddressType = 100,
    MessageSentRepresentingEmailAddress = 101,
    MessageConversationTopic = 112,
    MessageConversationIndex = 113,
    MessageReceivedByAddressType = 117,
    MessageReceivedByEmailAddress = 118,
    MessageReceivedRepresentingAddressType = 119,
    MessageReceivedRepresentingEmailAddress = 120,
    MessageTransportHeaders = 125,
    RecipientType = 3093,
    MessageSenderEntryIdentifier = 3097,
    MessageSenderName = 3098,
    MessageSenderSearchKey = 3101,
    MessageSenderAddressType = 3102,
    MessageSenderEmailAddress = 3103,
    MessageDisplayTo = 3588,
    MessageDeliveryTime = 3590,
    MessageFlags = 3591,
    MessageSize = 3592,
    MessageStatus = 3607,
    AttachmentSize = 3616,
    MessageInternetArticleNumber = 3619,
    MessagePermission = 3623,
    MessageUrlComputerNameSet = 3682,
    MessageTrustSender = 3705,
    MessageBodyPlainText = 4096,
    MessageBodyCompressedRtf = 4105,
    MessageBodyHtml = 4115,
    EmailEmlFilename = 4339,
    DisplayName = 12289,
    AddressType = 12290,
    EmailAddress = 12291,
    MessageCreationTime = 12295,
    MessageModificationTime = 12296,
    MessageStoreValidFolderMask = 13791,
    FolderType = 13825,
    NumberOfContentItems = 13826,
    NumberOfUnreadContentItems = 13827,
    HasSubFolders = 13834,
    ContainerClass = 13843,
    NumberOfAssociatedContent = 13847,
    AttachmentDataObject = 14081,
    AttachmentFilenameShort = 14084,
    AttachmentMethod = 14085,
    AttachmentFilenameLong = 14087,
    AttachmentRenderingPosition = 14091,
    ContactCallbackPhoneNumber = 14850,
    ContactGenerationalAbbreviation = 14853,
    ContactGivenName = 14854,
    ContactBusinessPhoneNumber1 = 14856,
    ContactHomePhoneNumber = 14857,
    ContactInitials = 14858,
    ContactSurname = 14865,
    ContactPostalAddress = 14869,
    ContactCompanyName = 14870,
    ContactJobTitle = 14871,
    ContactDepartmentName = 14872,
    ContactOfficeLocation = 14873,
    ContactPrimaryPhoneNumber = 14874,
    ContactBusinessPhoneNumber2 = 14875,
    ContactMobilePhoneNumber = 14876,
    ContactBusinessFaxNumber = 14884,
    ContactCountry = 14886,
    ContactLocality = 14887,
    ContactTitle = 14917,
    MessageBodyCodepage = 16350,
    MessageCodepage = 16381,
    RecipientDisplayName = 24566,
    FolderChildCount = 26168,
    SubItemIdentifier = 26610,
    MessageStorePasswordChecksum = 26623,
    AddressFileUnder = 32773,
    DistributionListName = 32851,
    DistributionListMemberOneOffEntryIdentifiers = 32852,
    DistributionListMemberEntryIdentifiers = 32853,
    ContactEmailAddress1 = 32899,
    ContactEmailAddress2 = 32915,
    ContactEmailAddress3 = 32931,
    TaskStatus = 33025,
    TaskPercentageComplete = 33026,
    TaskStartDate = 33028,
    TaskDueDate = 33029,
    TaskActualEffort = 33040,
    TaskTotalEffort = 33041,
    TaskVersion = 33042,
    TaskIsComplete = 33052,
    TaskIsRecurring = 33062,
    AppointmentBusyStatus = 33285,
    AppointmentLocation = 33288,
    AppointmentStartTime = 33293,
    AppointmentEndTime = 33294,
    AppointmentDuration = 33299,
    AppointmentIsRecurring = 33315,
    AppointmentRecurrencePattern = 33330,
    AppointmentTimezoneDescription = 33332,
    AppointmentFirstEffectiveTime = 33333,
    AppointmentLastEffectiveTime = 33334,
    MessageReminderTime = 34050,
    MessageIsReminder = 34051,
    MessageIsPrivate = 34054,
    MessageReminderSignalTime = 34128,
}

impl TryFrom<i32> for EntryType {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            23 => Ok(Self::MessageImportance),
            26 => Ok(Self::MessageClass),
            38 => Ok(Self::MessagePriority),
            54 => Ok(Self::MessageSensitivity),
            55 => Ok(Self::MessageSubject),
            57 => Ok(Self::MessageClientSubmitTime),
            59 => Ok(Self::MessageSentRepresentingSearchKey),
            63 => Ok(Self::MessageReceivedByEntryIdentifier),
            64 => Ok(Self::MessageReceivedByName),
            65 => Ok(Self::MessageSentRepresentingEntryIdentifier),
            66 => Ok(Self::MessageSentRepresentingName),
            67 => Ok(Self::MessageReceivedRepresentingEntryIdentifier),
            68 => Ok(Self::MessageReceivedRepresentingName),
            79 => Ok(Self::MessageReplyRecipientEntries),
            80 => Ok(Self::MessageReplyRecipientNames),
            81 => Ok(Self::MessageReceivedBySearchKey),
            82 => Ok(Self::MessageReceivedRepresentingSearchKey),
            100 => Ok(Self::MessageSentRepresentingAddressType),
            101 => Ok(Self::MessageSentRepresentingEmailAddress),
            112 => Ok(Self::MessageConversationTopic),
            113 => Ok(Self::MessageConversationIndex),
            117 => Ok(Self::MessageReceivedByAddressType),
            118 => Ok(Self::MessageReceivedByEmailAddress),
            119 => Ok(Self::MessageReceivedRepresentingAddressType),
            120 => Ok(Self::MessageReceivedRepresentingEmailAddress),
            125 => Ok(Self::MessageTransportHeaders),
            3093 => Ok(Self::RecipientType),
            3097 => Ok(Self::MessageSenderEntryIdentifier),
            3098 => Ok(Self::MessageSenderName),
            3101 => Ok(Self::MessageSenderSearchKey),
            3102 => Ok(Self::MessageSenderAddressType),
            3103 => Ok(Self::MessageSenderEmailAddress),
            3588 => Ok(Self::MessageDisplayTo),
            3590 => Ok(Self::MessageDeliveryTime),
            3591 => Ok(Self::MessageFlags),
            3592 => Ok(Self::MessageSize),
            3607 => Ok(Self::MessageStatus),
            3616 => Ok(Self::AttachmentSize),
            3619 => Ok(Self::MessageInternetArticleNumber),
            3623 => Ok(Self::MessagePermission),
            3682 => Ok(Self::MessageUrlComputerNameSet),
            3705 => Ok(Self::MessageTrustSender),
            4096 => Ok(Self::MessageBodyPlainText),
            4105 => Ok(Self::MessageBodyCompressedRtf),
            4115 => Ok(Self::MessageBodyHtml),
            4339 => Ok(Self::EmailEmlFilename),
            12289 => Ok(Self::DisplayName),
            12290 => Ok(Self::AddressType),
            12291 => Ok(Self::EmailAddress),
            12295 => Ok(Self::MessageCreationTime),
            12296 => Ok(Self::MessageModificationTime),
            13791 => Ok(Self::MessageStoreValidFolderMask),
            13825 => Ok(Self::FolderType),
            13826 => Ok(Self::NumberOfContentItems),
            13827 => Ok(Self::NumberOfUnreadContentItems),
            13834 => Ok(Self::HasSubFolders),
            13843 => Ok(Self::ContainerClass),
            13847 => Ok(Self::NumberOfAssociatedContent),
            14081 => Ok(Self::AttachmentDataObject),
            14084 => Ok(Self::AttachmentFilenameShort),
            14085 => Ok(Self::AttachmentMethod),
            14087 => Ok(Self::AttachmentFilenameLong),
            14091 => Ok(Self::AttachmentRenderingPosition),
            14850 => Ok(Self::ContactCallbackPhoneNumber),
            14853 => Ok(Self::ContactGenerationalAbbreviation),
            14854 => Ok(Self::ContactGivenName),
            14856 => Ok(Self::ContactBusinessPhoneNumber1),
            14857 => Ok(Self::ContactHomePhoneNumber),
            14858 => Ok(Self::ContactInitials),
            14865 => Ok(Self::ContactSurname),
            14869 => Ok(Self::ContactPostalAddress),
            14870 => Ok(Self::ContactCompanyName),
            14871 => Ok(Self::ContactJobTitle),
            14872 => Ok(Self::ContactDepartmentName),
            14873 => Ok(Self::ContactOfficeLocation),
            14874 => Ok(Self::ContactPrimaryPhoneNumber),
            14875 => Ok(Self::ContactBusinessPhoneNumber2),
            14876 => Ok(Self::ContactMobilePhoneNumber),
            14884 => Ok(Self::ContactBusinessFaxNumber),
            14886 => Ok(Self::ContactCountry),
            14887 => Ok(Self::ContactLocality),
            14917 => Ok(Self::ContactTitle),
            16350 => Ok(Self::MessageBodyCodepage),
            16381 => Ok(Self::MessageCodepage),
            24566 => Ok(Self::RecipientDisplayName),
            26168 => Ok(Self::FolderChildCount),
            26610 => Ok(Self::SubItemIdentifier),
            26623 => Ok(Self::MessageStorePasswordChecksum),
            32773 => Ok(Self::AddressFileUnder),
            32851 => Ok(Self::DistributionListName),
            32852 => Ok(Self::DistributionListMemberOneOffEntryIdentifiers),
            32853 => Ok(Self::DistributionListMemberEntryIdentifiers),
            32899 => Ok(Self::ContactEmailAddress1),
            32915 => Ok(Self::ContactEmailAddress2),
            32931 => Ok(Self::ContactEmailAddress3),
            33025 => Ok(Self::TaskStatus),
            33026 => Ok(Self::TaskPercentageComplete),
            33028 => Ok(Self::TaskStartDate),
            33029 => Ok(Self::TaskDueDate),
            33040 => Ok(Self::TaskActualEffort),
            33041 => Ok(Self::TaskTotalEffort),
            33042 => Ok(Self::TaskVersion),
            33052 => Ok(Self::TaskIsComplete),
            33062 => Ok(Self::TaskIsRecurring),
            33285 => Ok(Self::AppointmentBusyStatus),
            33288 => Ok(Self::AppointmentLocation),
            33293 => Ok(Self::AppointmentStartTime),
            33294 => Ok(Self::AppointmentEndTime),
            33299 => Ok(Self::AppointmentDuration),
            33315 => Ok(Self::AppointmentIsRecurring),
            33330 => Ok(Self::AppointmentRecurrencePattern),
            33332 => Ok(Self::AppointmentTimezoneDescription),
            33333 => Ok(Self::AppointmentFirstEffectiveTime),
            33334 => Ok(Self::AppointmentLastEffectiveTime),
            34050 => Ok(Self::MessageReminderTime),
            34051 => Ok(Self::MessageIsReminder),
            34054 => Ok(Self::MessageIsPrivate),
            34128 => Ok(Self::MessageReminderSignalTime),
            _ => Err(v),
        }
    }
}
