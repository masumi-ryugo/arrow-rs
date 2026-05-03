// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![no_main]

use libfuzzer_sys::fuzz_target;
use parquet::file::metadata::ParquetMetaDataReader;

// Drives the public thrift footer-decoding entry point with arbitrary bytes.
// Exercises `parquet::parquet_thrift::read_thrift_vec` and the per-field
// allocation paths inside `parquet::file::metadata::thrift` without needing
// the input to be wrapped in a valid `[footer_len][PAR1]` envelope.
fuzz_target!(|data: &[u8]| {
    let _ = ParquetMetaDataReader::decode_metadata(data);
});
