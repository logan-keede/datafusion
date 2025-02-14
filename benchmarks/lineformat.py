#!/usr/bin/env python
# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

from __future__ import annotations

import json
from dataclasses import dataclass
from typing import Dict, List, Any
from pathlib import Path
from argparse import ArgumentParser

try:
    from rich.console import Console
    from rich.table import Table
except ImportError:
    print("Couldn't import modules -- run `./bench.sh venv` first")
    raise


@dataclass
class QueryResult:
    elapsed: float
    row_count: int

    @classmethod
    def load_from(cls, data: Dict[str, Any]) -> QueryResult:
        return cls(elapsed=data["elapsed"], row_count=data["row_count"])


@dataclass
class QueryRun:
    query: int
    iterations: List[QueryResult]
    start_time: int

    @classmethod
    def load_from(cls, data: Dict[str, Any]) -> QueryRun:
        return cls(
            query=data["query"],
            iterations=[QueryResult(**iteration) for iteration in data["iterations"]],
            start_time=data["start_time"],
        )

    @property
    def execution_time(self) -> float:
        assert len(self.iterations) >= 1

        # Use minimum execution time to account for variations / other
        # things the system was doing
        return min(iteration.elapsed for iteration in self.iterations)


@dataclass
class Context:
    benchmark_version: str
    datafusion_version: str
    num_cpus: int
    start_time: int
    arguments: List[str]
    name: str

    @classmethod
    def load_from(cls, data: Dict[str, Any]) -> Context:
        return cls(
            benchmark_version=data["benchmark_version"],
            datafusion_version=data["datafusion_version"],
            num_cpus=data["num_cpus"],
            start_time=data["start_time"],
            arguments=data["arguments"],
            name=data["arguments"][0]
        )


@dataclass
class BenchmarkRun:
    context: Context
    queries: List[QueryRun]

    @classmethod
    def load_from(cls, data: Dict[str, Any]) -> BenchmarkRun:
        return cls(
            context=Context.load_from(data["context"]),
            queries=[QueryRun.load_from(result) for result in data["queries"]],
        )

    @classmethod
    def load_from_file(cls, path: Path) -> BenchmarkRun:
        with open(path, "r") as f:
            return cls.load_from(json.load(f))


def compare(
    baseline: Path,
) -> None:
    baseline = BenchmarkRun.load_from_file(baseline)
    context = baseline.context
    benchamrk_str = f"benchmark,name={context.name},version={context.benchmark_version},datafusion_version={context.datafusion_version},num_cpus={context.num_cpus}"
    for query in baseline.queries:
        query_str = f"query=\"{query.query}\""
        timestamp = f"{query.start_time*10**9}"
        for iter_num, result in enumerate(query.iterations):
            print(f"{benchamrk_str} {query_str},iteration={iter_num},row_count={result.row_count},elapsed_ms={result.elapsed*1000:.0f} {timestamp}")
    
def main() -> None:
    parser = ArgumentParser()
    compare_parser = parser
    compare_parser.add_argument(
        "baseline_path",
        type=Path,
        help="Path to the baseline summary file.",
    )

    options = parser.parse_args()

    compare(options.baseline_path)



if __name__ == "__main__":
    main()
