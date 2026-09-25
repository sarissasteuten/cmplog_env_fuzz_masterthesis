#!/bin/bash

SAMPLE="$HOME/Desktop/thesis-sarissa/test_samples/sample1"
INPUT="$HOME/Desktop/thesis-sarissa/fuzzer/corpus"
OUTPUT_BASELINE="$HOME/Desktop/thesis-sarissa/results_baseline/output"
OUTPUT_CMPLOG="$HOME/Desktop/thesis-sarissa/results_cmplog/output"

rm -rf $OUTPUT_BASELINE/*
rm -rf $OUTPUT_CMPLOG/*

echo "START DEMO"
echo

# log_sample_info ~/Desktop/thesis-sarissa/test_samples/sample1_static_stripped sample_info.log

echo "------------------------"
echo "RUN BASELINE with sample"
echo "------------------------"

timeout 180s \
./../target/debug/qemu_launcher \
  --input $INPUT \
  --output $OUTPUT_BASELINE \
  --cores 0-1 \
  --snapshots \
  --verbose \
  -- $SAMPLE \
  > baseline_sample1.log 2>&1
  
# pkill -f qemu_launcher
# sleep 5

echo "------------------------"
echo "DONE with baseline"
echo "------------------------"


rm -rf $OUTPUT_BASELINE/*
rm -rf $OUTPUT_CMPLOG/*


echo
echo

echo "------------------------"
echo "RUN CMPLOG with sample 1"
echo "------------------------"

timeout 180s \
 ./../target/debug/qemu_launcher \
  --input $INPUT \
  --output $OUTPUT_CMPLOG \
  --cores 0-1 \
  --snapshots \
  --cmplog-cores 1 \
  --verbose \
  -- $SAMPLE \
  > cmplog_sample1.log 2>&1

echo "------------------------"
echo "DONE running cmplog"
echo "------------------------"

# pkill -f qemu_launcher
# sleep 5
echo
echo

echo "------------------------"
echo "RESULTS"
echo "------------------------"

echo 
echo "-- Baseline--  "
echo

baseline_cov=$(grep "map_feedback" baseline_sample1.log | tail -n 1 | grep -oP 'map_feedback: \K[0-9]+/[0-9]+')

baseline_exec=$(grep "exec/sec" baseline_sample1.log | tail -n 1 | grep -oE 'exec/sec: [^,]+' | cut -d' ' -f2)

baseline_corpus=$(grep "corpus:" baseline_sample1.log | tail -n 1 | grep -oE 'corpus: [0-9]+' | cut -d' ' -f2)

echo "Corpus states discovered: $baseline_corpus"
echo "Coverage reached: $baseline_cov"
echo "Execution speed: $baseline_exec exec/sec"


echo 
echo "-- Cmplog--"
echo

cmplog_cov=$(grep "map_feedback" cmplog_sample1.log | tail -n 1 | grep -oP 'map_feedback: \K[0-9]+/[0-9]+')

cmplog_exec=$(grep "exec/sec" cmplog_sample1.log | tail -n 1 | grep -oE 'exec/sec: [^,]+' | cut -d' ' -f2)

cmplog_corpus=$(grep "corpus:" cmplog_sample1.log | tail -n 1 | grep -oE 'corpus: [0-9]+' | cut -d' ' -f2)

echo "Corpus states discovered: $cmplog_corpus"
echo "Coverage reached: $cmplog_cov"
echo "Execution speed: $cmplog_exec exec/sec"

echo
echo "END DEMO"