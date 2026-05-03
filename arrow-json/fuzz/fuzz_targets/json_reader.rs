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

use std::io::{BufReader, Cursor};
use std::sync::Arc;

use arrow_json::reader::{infer_json_schema_from_seekable, ReaderBuilder};
use libfuzzer_sys::fuzz_target;

// Drives the full arrow-json read path: schema inference + record-batch
// decoding. The harness intentionally feeds the same bytes through both
// stages because that's how `arrow_json::reader::Reader` is normally
// constructed (`infer_json_schema_from_seekable` then `ReaderBuilder::build`).
fuzz_target!(|data: &[u8]| {
    // Cap per-input work — long lines or huge integer payloads can otherwise
    // make a single fuzz iteration exceed libFuzzer's per-input timeout
    // before any interesting branch coverage is achieved.
    if data.len() > 1 << 16 {
        return;
    }

    let cursor = Cursor::new(data);
    let mut buf_reader = BufReader::new(cursor);

    let Ok((schema, _read)) = infer_json_schema_from_seekable(&mut buf_reader, Some(8)) else {
        return;
    };

    // `infer_json_schema_from_seekable` rewinds the cursor for us.
    let Ok(mut reader) = ReaderBuilder::new(Arc::new(schema)).build(buf_reader) else {
        return;
    };

    for batch in &mut reader {
        let _ = batch;
    }
});
