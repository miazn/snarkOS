// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

// Re-export the metrics macros.
pub use metrics::*;

use metrics_exporter_prometheus::PrometheusBuilder;

pub fn initialize() -> Option<tokio::task::JoinHandle<()>> {
    let (recorder, exporter) = PrometheusBuilder::new()
        .build_with_exporter()
        .expect("can't build the prometheus exporter");

    metrics::set_boxed_recorder(Box::new(recorder)).expect("can't set the prometheus exporter");

    let metrics_exporter_task = tokio::task::spawn(async move {
        exporter.await.expect("can't await the prometheus exporter");
    });

    Some(metrics_exporter_task)
}