trap 'git checkout main' EXIT #checkout to main on exit


usage() {
    echo "
Orchestrates running benchmarks against DataFusion checkouts

Usage:
$0 data [benchmark]
$0 run [benchmark]
$0 compare <branch1> <branch2>
$0 venv

**********
Examples:
**********
# Create the datasets for all benchmarks in $DATA_DIR
./bench.sh data

# Run the 'tpch' benchmark on the datafusion checkout in /source/datafusion
DATAFUSION_DIR=/source/datafusion ./bench.sh run tpch

**********
* Commands
**********
data:         Generates or downloads data needed for benchmarking
run:          Runs the named benchmark
compare:      Compares results from benchmark runs
venv:         Creates new venv (unless already exists) and installs compare's requirements into it

**********
* Benchmarks
**********
all(default): Data/Run/Compare for all benchmarks
tpch:                   TPCH inspired benchmark on Scale Factor (SF) 1 (~1GB), single parquet file per table, hash join
tpch_mem:               TPCH inspired benchmark on Scale Factor (SF) 1 (~1GB), query from memory
tpch10:                 TPCH inspired benchmark on Scale Factor (SF) 10 (~10GB), single parquet file per table, hash join
tpch_mem10:             TPCH inspired benchmark on Scale Factor (SF) 10 (~10GB), query from memory
cancellation:           How long cancelling a query takes
parquet:                Benchmark of parquet reader's filtering speed
sort:                   Benchmark of sorting speed
sort_tpch:              Benchmark of sorting speed for end-to-end sort queries on TPCH dataset
clickbench_1:           ClickBench queries against a single parquet file
clickbench_partitioned: ClickBench queries against a partitioned (100 files) parquet
clickbench_extended:    ClickBench \"inspired\" queries against a single parquet (DataFusion specific)
external_aggr:          External aggregation benchmark
h2o_small:              h2oai benchmark with small dataset (1e7 rows) for groupby,  default file format is csv
h2o_medium:             h2oai benchmark with medium dataset (1e8 rows) for groupby, default file format is csv
h2o_big:                h2oai benchmark with large dataset (1e9 rows) for groupby,  default file format is csv
h2o_small_join:         h2oai benchmark with small dataset (1e7 rows) for join,  default file format is csv
h2o_medium_join:        h2oai benchmark with medium dataset (1e8 rows) for join, default file format is csv
h2o_big_join:           h2oai benchmark with large dataset (1e9 rows) for join,  default file format is csv
imdb:                   Join Order Benchmark (JOB) using the IMDB dataset converted to parquet

**********
* Supported Configuration (Environment Variables)
**********
DATA_DIR            directory to store datasets
CARGO_COMMAND       command that runs the benchmark binary
DATAFUSION_DIR      directory to use (default $DATAFUSION_DIR)
RESULTS_NAME        folder where the benchmark files are stored
PREFER_HASH_JOIN    Prefer hash join algorithm (default true)
VENV_PATH           Python venv to use for compare and venv commands (default ./venv, override by <your-venv>/bin/activate)
"
    exit 1
}

# https://stackoverflow.com/questions/192249/how-do-i-parse-command-line-arguments-in-bash
POSITIONAL_ARGS=()

while [[ $# -gt 0 ]]; do
    case $1 in
        # -e|--extension)
        #   EXTENSION="$2"
        #   shift # past argument
        #   shift # past value
        #   ;;
        -h|--help)
            shift # past argument
            usage
            ;;
        -*)
            echo "Unknown option $1"
            exit 1
            ;;
        *)
            POSITIONAL_ARGS+=("$1") # save positional arg
            shift # past argument
            ;;
    esac
done

set -- "${POSITIONAL_ARGS[@]}" # restore positional parameters
COMMAND=${1:-"${COMMAND}"}
ARG2=$2


main(){

git fetch upstream main
git checkout main

# get current major version 
output=$(cargo metadata --format-version=1 --no-deps | jq '.packages[] | select(.name == "datafusion") | .version')
major_version=$(echo "$output" | grep -oE '[0-9]+' | head -n1)

# run for current main
echo "current major version: $major_version"  
./bench.sh run $ARG2

# run for last 5 major releases
for i in {1..5}; do
    echo "running benchmark on  $((major_version-i)).0.0"
    git fetch upstream $((major_version-i)).0.0
    git checkout $((major_version-i)).0.0
    ./bench.sh run $ARG2
done
}

main