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

use std::io::Cursor;
use std::sync::Arc;

use arrow_csv::reader::{Format, ReaderBuilder};
use libfuzzer_sys::fuzz_target;

// Drives the full arrow-csv read path: schema inference + record-batch decode.
// Mirrors the structure of the arrow-json harness so coverage hits both the
// type-inference machinery and the per-row parser.
fuzz_target!(|data: &[u8]| {
    if data.len() > 1 << 16 {
        return;
    }

    let format = Format::default().with_header(false);

    // `infer_schema` takes the reader by value, so feed two independent
    // cursors over the same bytes.
    let Ok((schema, _read)) = format.infer_schema(Cursor::new(data), Some(16)) else {
        return;
    };

    let Ok(mut reader) = ReaderBuilder::new(Arc::new(schema))
        .with_format(format)
        .build(Cursor::new(data))
    else {
        return;
    };

    for batch in &mut reader {
        let _ = batch;
    }
});
