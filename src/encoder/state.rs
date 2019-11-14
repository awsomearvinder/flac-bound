use flac_sys::{FLAC__StreamEncoderState, FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_OK, FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_UNINITIALIZED,
               FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_OGG_ERROR, FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_VERIFY_DECODER_ERROR,
               FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_VERIFY_MISMATCH_IN_AUDIO_DATA, FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_CLIENT_ERROR,
               FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_IO_ERROR, FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_FRAMING_ERROR,
               FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_MEMORY_ALLOCATION_ERROR};
use std::convert::TryFrom;


/// State values for a `FLAC__StreamEncoder`.
///
/// The encoder's state can be obtained by calling `FLAC__stream_encoder_get_state()`.
///
/// If the encoder gets into any other state besides `FLAC__STREAM_ENCODER_OK`
/// or `FLAC__STREAM_ENCODER_UNINITIALIZED`, it becomes invalid for encoding and
/// must be deleted with FLAC__stream_encoder_delete().
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FlacEncoderState {
    /// The encoder is in the normal OK state and samples can be processed.
    Ok = FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_OK,

    /// The encoder is in the uninitialized state; one of the `FLAC__stream_encoder_init_*()` functions must be called before
    /// samples can be processed.
    Uninitialized = FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_UNINITIALIZED,

    /// An error occurred in the underlying Ogg layer.
    OggError = FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_OGG_ERROR,

    /// An error occurred in the underlying verify stream decoder; check `FLAC__stream_encoder_get_verify_decoder_state()`.
    VerifyDecoderError = FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_VERIFY_DECODER_ERROR,

    /// The verify decoder detected a mismatch between the original audio signal and the decoded audio signal.
    VerifyMismatchInAudioData = FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_VERIFY_MISMATCH_IN_AUDIO_DATA,

    /// One of the callbacks returned a fatal error.
    ClientError = FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_CLIENT_ERROR,

    /// An I/O error occurred while opening/reading/writing a file. Check `errno`.
    IoError = FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_IO_ERROR,

    /// An error occurred while writing the stream; usually, the write_callback returned an error.
    FramingError = FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_FRAMING_ERROR,

    /// Memory allocation failed.
    MemoryAllocationError = FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_MEMORY_ALLOCATION_ERROR,
}

impl Into<FLAC__StreamEncoderState> for FlacEncoderState {
    fn into(self) -> FLAC__StreamEncoderState {
        self as FLAC__StreamEncoderState
    }
}

impl TryFrom<FLAC__StreamEncoderState> for FlacEncoderState {
    type Error = ();

    #[allow(non_upper_case_globals)]
    fn try_from(raw: FLAC__StreamEncoderState) -> Result<FlacEncoderState, ()> {
        Ok(match raw {
            FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_OK => FlacEncoderState::Ok,
            FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_UNINITIALIZED => FlacEncoderState::Uninitialized,
            FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_OGG_ERROR => FlacEncoderState::OggError,
            FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_VERIFY_DECODER_ERROR => FlacEncoderState::VerifyDecoderError,
            FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_VERIFY_MISMATCH_IN_AUDIO_DATA => FlacEncoderState::VerifyMismatchInAudioData,
            FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_CLIENT_ERROR => FlacEncoderState::ClientError,
            FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_IO_ERROR => FlacEncoderState::IoError,
            FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_FRAMING_ERROR => FlacEncoderState::FramingError,
            FLAC__StreamEncoderState_FLAC__STREAM_ENCODER_MEMORY_ALLOCATION_ERROR => FlacEncoderState::MemoryAllocationError,
            _ => Err(())?,
        })
    }
}
