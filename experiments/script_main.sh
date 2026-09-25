proxmox="XXX"
TOKEN="XXX"
DATA_VM="ubuntu-data-server"
RESULTS_DIR="~/results"

declare -A VM_IPS=(
    [vm1]="ubuntu-server-1"
    [vm2]="ubuntu-server-2"
    [vm3]="ubuntu-server-3"
    [vm4]="ubuntu-server-4"
)

declare -A VM_IDS=(
    [vm1]="XXX3"
    [vm2]="XXX4"
    [vm3]="XXX5"
    [vm4]="XXX6"
)

SNAPSHOT_NAME="cleanstate"

restore_snapshot() {
    local vm=$1
    local vm_id=${VM_IDS[$vm]}
    echo "Restoring snapshot for $vm (ID: $vm_id)..."
    curl -k -X POST \
        -H "Authorization: PVEAPIToken=$TOKEN" \
        "$proxmox/api2/json/nodes/<NODENAME>/qemu/$vm_id/snapshot/$SNAPSHOT_NAME/rollback"
    # sleep 10  # wacht tot VM helemaal opgestart is
    echo "Waiting for $vm to come back online..."
    until ssh -o ConnectTimeout=5 -o StrictHostKeyChecking=no $vm_ip "echo ready" 2>/dev/null; do
        sleep 3
    done
    # echo "$vm is back online!"
    echo "Snapshot restored for $vm"
}

run_sample() {
    local vm=$1
    local vm_ip=${VM_IPS[$vm]}
    local sample=$2
    local sample_name=$(basename $sample)

    ssh $DATA_VM "mkdir -p $RESULTS_DIR/${vm}/${sample_name}"
    ssh $DATA_VM "mkdir -p $RESULTS_DIR/${vm}/${sample_name}/logs_baseline"
    ssh $DATA_VM "mkdir -p $RESULTS_DIR/${vm}/${sample_name}/logs_cmplog"

    ssh $vm_ip "chmod +x ~/samples/*"
    ssh $vm_ip "mkdir -p ~/results_baseline/output"
    ssh $vm_ip "mkdir -p ~/syscall_results"

    # ssh $vm_ip "pkill -f qemu_launcher; sleep 2"
    ssh $vm_ip "pkill -9 -f qemu_launcher; while pgrep -f qemu_launcher > /dev/null; do sleep 1; done"
    sleep 2
    ssh $vm_ip "ipcs -m | grep ubuntu | awk '{print \$2}' | xargs -r ipcrm -m 2>/dev/null; true"
    ssh $vm_ip "rm -f /dev/shm/libafl_* 2>/dev/null; true"
    
    # Clean output directories
    ssh $vm_ip "rm -rf ~/results_baseline/output/*"

    echo "[$vm] Running baseline for $sample_name"

    # ssh $vm_ip "ulimit -n 65535"
    
    timeout 620s ssh -o ConnectTimeout=60 -o ServerAliveInterval=2 -o ServerAliveCountMax=30 $vm_ip "bash -c 'ulimit -n 1048576; RUST_BACKTRACE=full timeout -k 15s 600s ~/qemu_run2/qemu_launcher \
    --input ~/corpus \
    --output ~/results_baseline/output \
    --cores 0-1 \
    --snapshots \
    --verbose \
    -- ~/samples/$sample_name 2>&1 | \
     tr -d \"\000\" | \
    tee >(head -c 100M > ~/results_baseline/${sample_name}_full_baseline.log) | \
    grep -E \"UserStats|Heartbeat|Testcase|Objective|map_feedback|exec/sec|corpus:|finding main|Timeout|Fuzzer-respawner|Child exited|panicked|ERROR|imported|Forced\" \
    --line-buffered \
    > ~/results_baseline/${sample_name}_baseline.log'" &

    SSH_PID=$!

    broker_count=0
    while kill -0 $SSH_PID 2>/dev/null; do
        sleep 5
        latest=$(ssh $vm_ip "tail -4 ~/results_baseline/${sample_name}_baseline.log 2>/dev/null")
        if  echo "$latest" | grep -q "Broker Heartbeat" && ! echo "$latest" | grep -qE "Client Heartbeat|UserStats|Testcase"; then
            broker_count=$((broker_count + 1))
        else
            broker_count=0
        fi
        if [ "$broker_count" -ge 3 ]; then
            echo "[$vm] Dead fuzzer detected for $sample_name baseline, killing early"
            ssh $vm_ip "echo '[STOPPED!!!] Dead fuzzer detected, stopped early' >> ~/results_baseline/${sample_name}_baseline.log"
            kill $SSH_PID 2>/dev/null
            # ssh $vm_ip "pkill -9 -f qemu_launcher" 2>/dev/null
            break
        fi
    done
    wait $SSH_PID 2>/dev/null

    # ssh $vm_ip "pkill -9 -f qemu_launcher"
    ssh $vm_ip "pkill -9 -f qemu_launcher; sleep 2; \
    ipcs -m | grep ubuntu | awk '{print \$2}' | xargs -r ipcrm -m 2>/dev/null; \
    rm -f /dev/shm/libafl_* 2>/dev/null; true"

    echo "[$vm] Collecting baseline results"
    scp $vm_ip:~/results_baseline/${sample_name}_baseline.log \
        $DATA_VM:$RESULTS_DIR/${vm}/${sample_name}/logs_baseline/${sample_name}_baseline.log
    scp $vm_ip:~/results_baseline/${sample_name}_full_baseline.log \
        $DATA_VM:$RESULTS_DIR/${vm}/${sample_name}/logs_baseline/${sample_name}_full_baseline.log
    scp -r $vm_ip:~/results_baseline/output \
        $DATA_VM:$RESULTS_DIR/${vm}/${sample_name}/baseline_fuzz_output
    scp -r $vm_ip:~/syscall_results/ \
        $DATA_VM:$RESULTS_DIR/${vm}/${sample_name}/syscall_results_baseline/

    echo "[$vm] Restoring snapshot after baseline"
    restore_snapshot $vm

    #CMPLOG
    ssh $vm_ip "rm -rf ~/syscall_results/*"

    ssh $vm_ip "chmod +x ~/samples/*"

    ssh $vm_ip "mkdir -p ~/results_cmplog/output"
    ssh $vm_ip "mkdir -p ~/syscall_results"
    ssh $vm_ip "rm -rf  ~/results_cmplog/output/*"

    ssh $vm_ip "pkill -f qemu_launcher; sleep 2"
    ssh $vm_ip "pkill -9 -f qemu_launcher; while pgrep -f qemu_launcher > /dev/null; do sleep 1; done"

    #  ssh $vm_ip "ulimit -n 65535"
    echo "[$vm] Running CmpLog for $sample_name"

    timeout 620s ssh -o ConnectTimeout=60 -o ServerAliveInterval=2 -o ServerAliveCountMax=30 $vm_ip "bash -c 'ulimit -n 1048576; RUST_BACKTRACE=full timeout -k 15s 600s ~/qemu_run2/qemu_launcher \
    --input ~/corpus \
    --output ~/results_cmplog/output \
    --cores 0-1 \
    --snapshots \
    --verbose \
    --cmplog-cores 1 \
    -- ~/samples/$sample_name 2>&1 | \
     tr -d \"\000\" | \
    tee >(head -c 100M > ~/results_cmplog/${sample_name}_full_cmplog.log) | \
    grep -E \"UserStats|Heartbeat|Testcase|Objective|map_feedback|exec/sec|corpus:|finding main|Timeout|Fuzzer-respawner|Child exited|panicked|ERROR|imported|Forced\" \
    --line-buffered \
    > ~/results_cmplog/${sample_name}_cmplog.log'" &

    SSH_PID=$!

    broker_count=0
    while kill -0 $SSH_PID 2>/dev/null; do
        sleep 5
        latest=$(ssh $vm_ip "tail -4 ~/results_cmplog/${sample_name}_cmplog.log 2>/dev/null")
        if  echo "$latest" | grep -q "Broker Heartbeat" && ! echo "$latest" | grep -qE "Client Heartbeat|UserStats|Testcase"; then
            broker_count=$((broker_count + 1))
        else
            broker_count=0
        fi
        if [ "$broker_count" -ge 3 ]; then
            echo "[$vm] Dead fuzzer detected for $sample_name cmplog, killing early"
            ssh $vm_ip "echo '[STOPPED!!!] Dead fuzzer detected, stopped early' >> ~/results_cmplog/${sample_name}_cmplog.log"
            kill $SSH_PID 2>/dev/null
            # ssh $vm_ip "pkill -9 -f qemu_launcher" 2>/dev/null
            break
        fi
    done
    wait $SSH_PID 2>/dev/null

    echo "[$vm] Collecting CmpLog results"
    scp $vm_ip:~/results_cmplog/${sample_name}_cmplog.log \
        $DATA_VM:$RESULTS_DIR/${vm}/${sample_name}/logs_cmplog/${sample_name}_cmplog.log
    scp $vm_ip:~/results_cmplog/${sample_name}_full_cmplog.log \
        $DATA_VM:$RESULTS_DIR/${vm}/${sample_name}/logs_cmplog/${sample_name}_full_cmplog.log
    scp -r $vm_ip:~/results_cmplog/output \
        $DATA_VM:$RESULTS_DIR/${vm}/${sample_name}/cmplog_fuzz_output
    scp -r $vm_ip:~/syscall_results/ \
        $DATA_VM:$RESULTS_DIR/${vm}/${sample_name}/syscall_results_cmplog/

    echo "[$vm] Restoring snapshot after CmpLog"
    restore_snapshot $vm

    echo "[$vm] Done with $sample_name"
}

echo "Starting experiments!!"

(
    for sample in $(ssh ubuntu-server-1 "ls ~/samples/subset_included/"); do
        run_sample vm1 $sample
    done
) &

(
    for sample in $(ssh ubuntu-server-2 "ls ~/samples/subset_included/"); do
        run_sample vm2 $sample
    done
) &

(
    for sample in $(ssh ubuntu-server-3 "ls ~/samples/subset_included/"); do
        run_sample vm3 $sample
    done
) &

(
    for sample in $(ssh ubuntu-server-4 "ls ~/samples/subset_included/"); do
        run_sample vm4 $sample
    done
) &
wait

echo "All experiments done!"
echo "Collecting all results from data VM..."
rsync -av $DATA_VM:$RESULTS_DIR ~/results_10min_run/
echo "Done!"