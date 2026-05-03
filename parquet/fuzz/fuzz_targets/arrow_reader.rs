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

use bytes::Bytes;
use libfuzzer_sys::fuzz_target;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

fuzz_target!(|data: &[u8]| {
    let bytes = Bytes::copy_from_slice(data);
    let Ok(builder) = ParquetRecordBatchReaderBuilder::try_new(bytes) else {
        return;
    };
    let Ok(reader) = builder.build() else {
        return;
    };
    for batch in reader {
        let _ = batch;
    }
});
