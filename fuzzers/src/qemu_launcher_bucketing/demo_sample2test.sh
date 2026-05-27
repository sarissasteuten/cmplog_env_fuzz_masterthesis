#!/bin/bash
rm -rf /home/sarissa/Desktop/thesis-sarissa/results_baseline/output
rm -rf /home/sarissa/Desktop/thesis-sarissa/results_cmplog/output

echo "START DEMO"
echo
echo "------------------------"
echo "RUNNING BASELINE with sample 2"
echo "------------------------"

# timeout 10s \
# ./target/debug/qemu_launcher \
#   --input ~/Desktop/thesis-sarissa/fuzzers/corpus \
#   --output ~/Desktop/thesis-sarissa/results_baseline/output \
#   --cores 0 \
#   --verbose \
#   -- ~/Desktop/thesis-sarissa/test_samples/sample2 \
#   > baseline_sample2.log 2>&1
# # | tee basline_sample1.log
  

echo "------------------------"
echo "DONE running baseline"
echo "------------------------"

echo
echo

echo "------------------------"
echo "RUNNING CMPLOG with sample 2"
echo "------------------------"

timeout 120s \
./target/debug/qemu_launcher \
  --input ~/Desktop/thesis-sarissa/fuzzers/corpus \
  --output ~/Desktop/thesis-sarissa/results_cmplog/output \
  --cores 0-1 \
  --cmplog-cores 1 \
  --verbose \
  -- ~/Desktop/thesis-sarissa/test_samples/sample2 \
  > cmplog_sample2.log 2>&1

echo "------------------------"
echo "DONE running cmplog"
echo "------------------------"

echo
echo

echo "------------------------"
echo "RESULTS"
echo "------------------------"

echo 
echo "--------- Baseline -----------"
echo
echo "Stages reached:"
echo
grep -oE -- "-+ STAGE [0-9]+ [A-Z_]+|target environment accepted|payload stage reached|" baseline_sample2.log | sort -u

echo 
echo "--------- Cmplog -----------"
echo
echo "Stages reached:"
echo
grep -oE -- "-+ STAGE [0-9]+ [A-Z_]+|target environment accepted|payload stage reached|" cmplog_sample2.log | sort -u

echo
echo "END DEMO"

