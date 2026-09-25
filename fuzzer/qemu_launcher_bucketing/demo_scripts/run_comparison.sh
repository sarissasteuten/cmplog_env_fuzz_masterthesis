#!/bin/bash

SAMPLE="$HOME/Desktop/thesis-sarissa/test_samples/sample1"
SAMPLE_STATIC="$HOME/Desktop/thesis-sarissa/test_samples/sample1_static"
INPUT="$HOME/Desktop/thesis-sarissa/fuzzer/corpus"
OUTPUT_BASELINE="$HOME/Desktop/thesis-sarissa/results_baseline/output"
OUTPUT_CMPLOG="$HOME/Desktop/thesis-sarissa/results_cmplog/output"
OUTPUT_CMPLOG_STATIC="$HOME/Desktop/thesis-sarissa/results_cmplog_static/output"

rm -rf $OUTPUT_BASELINE/*
rm -rf $OUTPUT_CMPLOG/*
rm -rf $OUTPUT_CMPLOG_STATIC/*


echo "START DEMO"
echo

echo "------------------------"
echo "RUNNING BASELINE"
echo "------------------------"

timeout 600s \
./../target/debug/qemu_launcher \
  --input $INPUT \
  --output $OUTPUT_BASELINE \
  --cores 0-1 \
  --snapshots \
  --verbose \
  -- $SAMPLE \
  > baseline_sample1.log 2>&1
# # | tee basline_sample1.log
  
pkill -f qemu_launcher
sleep 5

echo "------------------------"
echo "DONE running"
echo "------------------------"

rm -rf $OUTPUT_BASELINE/*
rm -rf $OUTPUT_CMPLOG/*
rm -rf $OUTPUT_CMPLOG_STATIC/*

echo
echo

echo "------------------------"
echo "RUNNING CMPLOG dynamic"
echo "------------------------"

timeout 600s \
./../target/debug/qemu_launcher \
  --input $INPUT \
  --output $OUTPUT_CMPLOG \
  --cores 0-1 \
  --snapshots \
  --cmplog-cores 1 \
  --verbose \
  -- $SAMPLE \
  > cmplog_sample1_dynamic.log 2>&1

pkill -f qemu_launcher
sleep 5

echo "------------------------"
echo "DONE running cmplog dynamic"
echo "------------------------"

rm -rf $OUTPUT_BASELINE/*
rm -rf $OUTPUT_CMPLOG/*
rm -rf $OUTPUT_CMPLOG_STATIC/*

echo
echo

echo
echo

echo "------------------------"
echo "RUNNING CMPLOG static"
echo "------------------------"

timeout 600s \
 ./../target/debug/qemu_launcher \
  --input $INPUT \
  --output $OUTPUT_CMPLOG_STATIC \
  --cores 0-1 \
  --snapshots \
  --cmplog-cores 1 \
  --verbose \
  -- $SAMPLE_STATIC \
  > cmplog_sample1_static.log 2>&1

echo "------------------------"
echo "DONE running cmplog static"
echo "------------------------"

pkill -f qemu_launcher
sleep 5
echo
echo

echo "------------------------"
echo "RESULTS"
echo "------------------------"

echo 
echo "---Baseline--"
echo

baseline_cov=$(grep "map_feedback" baseline_sample1.log | tail -n 1 | grep -oP 'map_feedback: \K[0-9]+/[0-9]+')

baseline_exec=$(grep "exec/sec" baseline_sample1.log | tail -n 1 | grep -oE 'exec/sec: [^,]+' | cut -d' ' -f2)

baseline_corpus=$(grep "corpus:" baseline_sample1.log | tail -n 1 | grep -oE 'corpus: [0-9]+' | cut -d' ' -f2)

echo "Corpus states discovered: $baseline_corpus"
echo "Coverage reached: $baseline_cov"
echo "Execution speed : $baseline_exec exec/sec"

echo 
echo "---Cmplog dynamic --"
echo

cmplog_cov=$(grep "map_feedback" cmplog_sample1_dynamic.log | tail -n 1 | grep -oP 'map_feedback: \K[0-9]+/[0-9]+')

cmplog_exec=$(grep "exec/sec" cmplog_sample1_dynamic.log | tail -n 1 | grep -oE 'exec/sec: [^,]+' | cut -d' ' -f2)

cmplog_corpus=$(grep "corpus:" cmplog_sample1_dynamic.log | tail -n 1 | grep -oE 'corpus: [0-9]+' | cut -d' ' -f2)

echo "Corpus states discovered : $cmplog_corpus"
echo "Coverage reached         : $cmplog_cov"
echo "Execution speed          : $cmplog_exec exec/sec"

echo 
echo "---Cmplog static ---"
echo

cmplog_static_cov=$(grep "map_feedback" cmplog_sample1_static.log | tail -n 1 | grep -oP 'map_feedback: \K[0-9]+/[0-9]+')

cmplog_static_exec=$(grep "exec/sec" cmplog_sample1_static.log | tail -n 1 | grep -oE 'exec/sec: [^,]+' | cut -d' ' -f2)

cmplog_static_corpus=$(grep "corpus:" cmplog_sample1_static.log | tail -n 1 | grep -oE 'corpus: [0-9]+' | cut -d' ' -f2)

echo "Corpus states discovered: $cmplog_static_corpus"
echo "Coverage reached: $cmplog_static_cov"
echo "Execution speed: $cmplog_static_exec exec/sec"

echo
echo "END DEMO"
