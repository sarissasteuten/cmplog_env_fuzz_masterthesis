# Master Thesis 
Malware can use environment-dependent checks to determine whether it is being executed in an analysis environment. If this is the case, the malware performs evasive behavior instead of its intended behavior. Such evasive behavior can range from simply exiting to deleting the entire file system. Environment fuzzing is an approach in which environment artifacts, such as system call return values, are modeled as input. While this approach is usually used to investigate how the environment affects execution, it can also be adapted to systematically manipulate environment artifacts and reach the intended behavior of the malware. A key limitation in fuzzing is solving magic byte comparisons, in other words multi byte string comparisons or integer comparisons against a specific value. This paper investigates whether CmpLog, a fuzzing technique specifically geared to solving such comparisons, can improve the effectiveness of environment fuzzing for malware. The approach is implemented in the LibAFL framework using QEMU-based instrumentation and is evaluated against a baseline configuration without CmpLog. The effectiveness is measured in terms of code coverage, triggered system calls and performance. Additionally, we included code coverage of a no fuzzing execution to determine the impact of environment fuzzing itself. The approach targets Linux-based malware, as this is an underexplored area compared to Windows. The evaluation is done using a representative and diverse set of real Linux-based malware samples. 

The evaluation of our approach shows that CmpLog significantly improves code coverage in 49% of the samples compared to the baseline. It also discovers additional unique system calls in 53.7% of the samples over the baseline. This difference is statistically significant in 25.6% of all samples. We observed the largest improvements in the malware families Gafgyt and Mirai. Regarding the performance, CmpLog introduces a median overhead of 18% in comparison to the baseline. The results suggest that CmpLog has a positive effect on environment fuzzing and bypassing environment dependent checks. 

## Overview
<img width="915" height="678" alt="image" src="https://github.com/user-attachments/assets/436c4547-9cfe-40b3-bb31-cd4570accae3" />

## Hardware dependencies
The artifact runs on a standard x86\_64 system and needs at least 2 CPU cores, to mimic the experiments. However, the fuzzer can also be run with 1 core. 

## Software dependencies
For software we used Ubuntu 24.04, Rust 1.94.0 and QEMU usermode. 

## Setup
The fuzzer can be built with either: 
cargo build, for debugging mode or cargo build --release

The fuzzer can be run using the following command: 
./target/release/qemu\_launcher \
  --input <input\_dir> \
  --output <output\_dir> \
  --cores 0-1 \
  --snapshots \
  --cmplog-cores 1 \
  -- <path\_to\_malware\_sample> 
  
Running without --cmplog-cores 1 \ results in running the baseline fuzzer. 

## Hashes of all malware samples
The complete list of all malware sample hashes is available in the Github in the dataset folder.
