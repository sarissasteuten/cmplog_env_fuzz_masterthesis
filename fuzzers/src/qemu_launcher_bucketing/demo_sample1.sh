#!/bin/bash
rm -rf /home/sarissa/Desktop/thesis-sarissa/results_baseline/output
rm -rf /home/sarissa/Desktop/thesis-sarissa/results_cmplog/output

echo "START DEMO"
echo
echo "------------------------"
echo "RUNNING BASELINE with sample 1"
echo "------------------------"

timeout 60s \
./target/debug/qemu_launcher \
  --input ~/Desktop/thesis-sarissa/fuzzers/corpus \
  --output ~/Desktop/thesis-sarissa/results_baseline/output \
  --cores 0-1 \
  --verbose \
  -- ~/Desktop/thesis-sarissa/test_samples/sample1 \
  > baseline_sample1.log 2>&1
# | tee basline_sample1.log
  

echo "------------------------"
echo "DONE running baseline"
echo "------------------------"

echo
echo

echo "------------------------"
echo "RUNNING CMPLOG with sample 1"
echo "------------------------"

timeout 60s \
./target/debug/qemu_launcher \
  --input ~/Desktop/thesis-sarissa/fuzzers/corpus \
  --output ~/Desktop/thesis-sarissa/results_cmplog/output \
  --cores 0-1 \
  --cmplog-cores 1 \
  --verbose \
  -- ~/Desktop/thesis-sarissa/test_samples/sample1 \
  > cmplog_sample1.log 2>&1
  # | tee cmplog_sample1.log

echo "------------------------"
echo "DONE running cmplog"
echo "------------------------"

echo
echo

echo "------------------------"
echo "RESULTS"
echo "------------------------"

echo 
echo "############# Baseline #######################################"
echo

baseline_cov=$(grep "map_feedback" baseline_sample1.log | tail -n 1 | grep -oP 'map_feedback: \K[0-9]+/[0-9]+')

baseline_exec=$(grep "exec/sec" baseline_sample1.log | tail -n 1 | grep -oE 'exec/sec: [^,]+' | cut -d' ' -f2)

baseline_corpus=$(grep "corpus:" baseline_sample1.log | tail -n 1 | grep -oE 'corpus: [0-9]+' | cut -d' ' -f2)

echo "Corpus states discovered : $baseline_corpus"
echo "Coverage reached         : $baseline_cov"
echo "Execution speed          : $baseline_exec exec/sec"

echo
echo "STAGES PASSED:"
echo
grep -oE -- "-+ STAGE [0-9]+ [A-Z_]+|PASSED STRING COMPARE|PASSED NUMERIC COMPARE|target environment accepted|payload stage reached|" baseline_sample1.log | sort -u

echo 
echo "############# Cmplog #######################################"
echo

cmplog_cov=$(grep "map_feedback" cmplog_sample1.log | tail -n 1 | grep -oP 'map_feedback: \K[0-9]+/[0-9]+')

cmplog_exec=$(grep "exec/sec" cmplog_sample1.log | tail -n 1 | grep -oE 'exec/sec: [^,]+' | cut -d' ' -f2)

cmplog_corpus=$(grep "corpus:" cmplog_sample1.log | tail -n 1 | grep -oE 'corpus: [0-9]+' | cut -d' ' -f2)

echo "Corpus states discovered : $cmplog_corpus"
echo "Coverage reached         : $cmplog_cov"
echo "Execution speed          : $cmplog_exec exec/sec"

echo
echo "STAGES PASSED:"
echo
grep -oE -- "-+ STAGE [0-9]+ [A-Z_]+|PASSED STRING COMPARE|PASSED NUMERIC COMPARE|target environment accepted|payload stage reached|" cmplog_sample1.log | sort -u

echo
echo "END DEMO"

