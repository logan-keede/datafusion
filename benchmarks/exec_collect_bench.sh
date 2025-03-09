COMMAND=${1:-"${COMMAND}"}
ARG2=$1
echo $ARG2
cp collect_bench collect_bench.sh
sh collect_bench.sh $ARG2