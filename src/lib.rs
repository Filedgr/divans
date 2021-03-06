// Copyright 2017 Dropbox, Inc
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.//   See the License for the specific language governing permissions and
//   limitations under the License.
//! This is documentation for the `divans` crate
//!
//! The `divans` crate is meant to be used for generic data compression

#![cfg_attr(feature="benchmark", feature(test))]
//#![cfg_attr(feature="simd", feature(platform_intrinsics))]
#![cfg_attr(not(feature="no-stdlib-rust-binding"),cfg_attr(not(feature="std"), feature(lang_items)))]
#![cfg_attr(not(feature="no-stdlib-rust-binding"),cfg_attr(not(feature="std"), feature(compiler_builtins_lib)))]
#![cfg_attr(not(feature="no-stdlib-rust-binding"),cfg_attr(not(feature="std"), crate_type="cdylib"))]
#![no_std]

#[cfg(not(test))]
#[cfg(any(feature="findspeed", feature="billing"))]
#[macro_use]
extern crate std;

#[cfg(feature="std")]
#[cfg(not(test))]
#[cfg(not(any(feature="billing", feature="findspeed")))]
#[macro_use]
extern crate std;

#[cfg(test)]
#[macro_use]
extern crate std;
#[cfg(feature="simd")]
#[macro_use(shuffle)]
extern crate packed_simd;
#[cfg(feature="std")]
extern crate alloc_stdlib;

extern crate alloc_no_stdlib as alloc;
extern crate brotli;
pub mod resizable_buffer;
pub mod interface;
pub mod slice_util;
pub mod alloc_util;
mod probability;
#[macro_use]
mod priors;
#[macro_use]
mod arithmetic_coder;
mod debug_encoder;
mod cmd_to_raw;
mod raw_to_cmd;
mod codec;
mod cmd_to_divans;
mod divans_to_raw;
#[macro_use]
mod billing;
pub mod test_helper;
mod test_ans;
mod test_mux;
mod ans;
mod brotli_ir_gen;
mod divans_compressor;
mod divans_decompressor;
mod parallel_decompressor;
mod stub_parallel_decompressor;
pub mod ir_optimize;
pub mod mux;
pub mod constants;
pub mod threading;
pub mod multithreading;
pub use self::interface::{DivansInputResult,DivansOpResult,DivansOutputResult, DivansResult, ErrMsg, MAGIC_NUMBER};
pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};
pub use interface::{DivansCompressorFactory, BlockSwitch, LiteralBlockSwitch, Command, Compressor, CopyCommand, Decompressor, DictCommand, LiteralCommand, Nop, NewWithAllocator, ArithmeticEncoderOrDecoder, LiteralPredictionModeNibble, PredictionModeContextMap, free_cmd, FeatureFlagSliceType,
                    DefaultCDF16};

pub use brotli_ir_gen::{BrotliDivansHybridCompressor,BrotliDivansHybridCompressorFactory};
pub use cmd_to_raw::DivansRecodeState;
pub use codec::CMD_BUFFER_SIZE;
pub use divans_to_raw::DecoderSpecialization;
pub use cmd_to_divans::EncoderSpecialization;
pub use codec::{EncoderOrDecoderSpecialization, DivansCodec, StrideSelection};
pub use divans_compressor::{DivansCompressor, DivansCompressorFactoryStruct};

#[cfg(not(feature="safe"))]
mod ffi;
#[cfg(not(feature="safe"))]
pub use ffi::*;
mod reader;
mod writer;
#[cfg(feature="std")]
pub use reader::DivansBrotliHybridCompressorReader;
#[cfg(feature="std")]
pub use reader::DivansExperimentalCompressorReader;
#[cfg(feature="std")]
pub use reader::DivansDecompressorReader;

#[cfg(feature="std")]
pub use writer::DivansBrotliHybridCompressorWriter;
#[cfg(feature="std")]
pub use writer::DivansExperimentalCompressorWriter;
#[cfg(feature="std")]
pub use writer::DivansDecompressorWriter;


pub use probability::Speed;


pub use probability::CDF2;
pub use probability::CDF16;
pub use probability::BaseCDF;

pub use interface::BrotliCompressionSetting;
pub use interface::DivansCompressorOptions;
pub use divans_decompressor::{DivansDecompressor,
                              DivansDecompressorFactory,
                              DivansDecompressorFactoryStruct,
                              StaticCommand};
