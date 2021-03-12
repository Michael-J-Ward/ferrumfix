//! Tag mnemonics for FIXT 1.1.

#![allow(dead_code)]

pub const BEGIN_SEQ_NO: u32 = 7;
pub const BEGIN_STRING: u32 = 8;
pub const BODY_LENGTH: u32 = 9;
pub const CHECK_SUM: u32 = 10;
pub const END_SEQ_NO: u32 = 16;
pub const MSG_SEQ_NUM: u32 = 34;
pub const MSG_TYPE: u32 = 35;
pub const NEW_SEQ_NO: u32 = 36;
pub const POSS_DUP_FLAG: u32 = 43;
pub const REF_SEQ_NUM: u32 = 45;
pub const SENDER_COMP_ID: u32 = 49;
pub const SENDER_SUB_ID: u32 = 50;
pub const SENDING_TIME: u32 = 52;
pub const TARGET_COMP_ID: u32 = 56;
pub const TARGET_SUB_ID: u32 = 57;
pub const TEXT: u32 = 58;
pub const SIGNATURE: u32 = 89;
pub const SECURE_DATA_LEN: u32 = 90;
pub const SECURE_DATA: u32 = 91;
pub const SIGNATURE_LENGTH: u32 = 93;
pub const RAW_DATA_LENGTH: u32 = 95;
pub const RAW_DATA: u32 = 96;
pub const POSS_RESEND: u32 = 97;
pub const ENCRYPT_METHOD: u32 = 98;
pub const HEART_BT_INT: u32 = 108;
pub const TEST_REQ_ID: u32 = 112;
pub const ON_BEHALF_OF_COMP_ID: u32 = 115;
pub const ON_BEHALF_OF_SUB_ID: u32 = 116;
pub const ORIG_SENDING_TIME: u32 = 122;
pub const GAP_FILL_FLAG: u32 = 123;
pub const DELIVER_TO_COMP_ID: u32 = 128;
pub const DELIVER_TO_SUB_ID: u32 = 129;
pub const RESET_SEQ_NUM_FLAG: u32 = 141;
pub const SENDER_LOCATION_ID: u32 = 142;
pub const TARGET_LOCATION_ID: u32 = 143;
pub const ON_BEHALF_OF_LOCATION_ID: u32 = 144;
pub const DELIVER_TO_LOCATION_ID: u32 = 145;
pub const XML_DATA_LEN: u32 = 212;
pub const XML_DATA: u32 = 213;
pub const MESSAGE_ENCODING: u32 = 347;
pub const ENCODED_TEXT_LEN: u32 = 354;
pub const ENCODED_TEXT: u32 = 355;
pub const LAST_MSG_SEQ_NUM_PROCESSED: u32 = 369;
pub const REF_TAG_ID: u32 = 371;
pub const REF_MSG_TYPE: u32 = 372;
pub const SESSION_REJECT_REASON: u32 = 373;
pub const MAX_MESSAGE_SIZE: u32 = 383;
pub const NO_MSG_TYPES: u32 = 384;
pub const MSG_DIRECTION: u32 = 385;
pub const TEST_MESSAGE_INDICATOR: u32 = 464;
pub const USERNAME: u32 = 553;
pub const PASSWORD: u32 = 554;
pub const NO_HOPS: u32 = 627;
pub const HOP_COMP_ID: u32 = 628;
pub const HOP_SENDING_TIME: u32 = 629;
pub const HOP_REF_ID: u32 = 630;
pub const NEXT_EXPECTED_MSG_SEQ_NUM: u32 = 789;
pub const APPL_VER_ID: u32 = 1128;
pub const CSTM_APPL_VER_ID: u32 = 1129;
pub const REF_APPL_VER_ID: u32 = 1130;
pub const REF_CSTM_APPL_VER_ID: u32 = 1131;
pub const DEFAULT_APPL_VER_ID: u32 = 1137;
