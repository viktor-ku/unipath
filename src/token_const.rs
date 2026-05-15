// header
// ---

pub(crate) const B_NULL: [u8; 1] = [0];

pub(crate) const B_DOT1: [u8; 1] = *b".";

pub(crate) const B_DOT2: [u8; 2] = *b"..";

// data
// ---

pub(crate) const B_DOT_CONFIG: [u8; 7] = *b".config";

pub(crate) const B_DOT_LOCAL: [u8; 6] = *b".local";
pub(crate) const B_DOT_CACHE: [u8; 6] = *b".cache";

pub(crate) const B_SHARE: [u8; 5] = *b"share";
pub(crate) const B_LOCAL: [u8; 5] = *b"local";
pub(crate) const B_STATE: [u8; 5] = *b"state";

pub(crate) const B_HOME: [u8; 4] = *b"home";

pub(crate) const B_ETC: [u8; 3] = *b"etc";
pub(crate) const B_XDG: [u8; 3] = *b"xdg";
pub(crate) const B_USR: [u8; 3] = *b"usr";
pub(crate) const B_BIN: [u8; 3] = *b"bin";
