import re
import sys
import csv
from pathlib import Path

def parse_time(time_str):
    """Converteer '1m-15s' of '15s' naar seconden"""
    match = re.match(r'(?:(\d+)m-)?(\d+)s', time_str)
    if match:
        minutes = int(match.group(1)) if match.group(1) else 0
        seconds = int(match.group(2))
        return minutes * 60 + seconds
    return 0

def parse_log(log_path, config_name, sample_name, run_number):
    log = open(log_path).read()
    rows = []
    last_time = 0
    seen_stages = set() 

    for line in log.split('\n'):
        # Update tijd
        time_match = re.search(r'run time: ([\dm\-s]+)', line)
        if time_match:
            last_time = parse_time(time_match.group(1))
            
        if '(CLIENT)' in line:
            map_match = re.search(r'map_feedback: (\d+)/(\d+)', line)
            if map_match:
                rows.append({'config': config_name, 'sample': sample_name,
                            'run': run_number, 'event': 'coverage',
                            'value': int(map_match.group(1)), 'time': last_time})

        if '(GLOBAL)' in line:
            corpus_match = re.search(r'corpus: (\d+)', line)
            exec_match = re.search(r'exec/sec: ([\d.]+)(k?)', line)

            if corpus_match:
                rows.append({'config': config_name, 'sample': sample_name,
                            'run': run_number, 'event': 'corpus',
                            'value': int(corpus_match.group(1)), 'time': last_time})

            if exec_match:
                speed = float(exec_match.group(1))
                if exec_match.group(2) == 'k':
                    speed *= 1000
                rows.append({'config': config_name, 'sample': sample_name,
                            'run': run_number, 'event': 'throughput',
                            'value': int(speed), 'time': last_time})

        # Nieuwe syscalls
        syscall_match = re.match(r'syscall_log,(\d+),(\d+)', line)
        if syscall_match:
            rows.append({'config': config_name, 'sample': sample_name,
                         'run': run_number, 'event': 'new_syscall',
                         'value': int(syscall_match.group(1)),
                         'time': int(syscall_match.group(2))})

        # Milestones
        milestone_match = re.match(r'milestone,(\w+),(\d+)', line)
        if milestone_match:
            rows.append({'config': config_name, 'sample': sample_name,
                         'run': run_number, 'event': f'milestone_{milestone_match.group(1)}',
                         'value': 1, 'time': int(milestone_match.group(2))})

        # Stages
        stage_match = re.search(r'-+ (STAGE \d+ \w+)', line)
        if stage_match:
            stage = stage_match.group(1).replace(' ', '_')
            if stage not in seen_stages:  # <-- alleen eerste keer
                seen_stages.add(stage)
                rows.append({'config': config_name, 'sample': sample_name,
                             'run': run_number, 'event': f'stage_{stage}',
                             'value': 1, 'time': last_time})

        # Target accepted
        if 'target environment accepted' in line:
            rows.append({'config': config_name, 'sample': sample_name,
                         'run': run_number, 'event': 'target_accepted',
                         'value': 1, 'time': last_time})

        # Payload reached
        if 'payload stage reached' in line:
            rows.append({'config': config_name, 'sample': sample_name,
                         'run': run_number, 'event': 'payload_reached',
                         'value': 1, 'time': last_time})

        # Final unique syscalls
        final_match = re.match(r'final,unique_syscalls,(\d+),(\d+)', line)
        if final_match:
            rows.append({'config': config_name, 'sample': sample_name,
                         'run': run_number, 'event': 'final_unique_syscalls',
                         'value': int(final_match.group(1)),
                         'time': int(final_match.group(2))})

    return rows

if __name__ == "__main__":
    log_path = sys.argv[1]
    config = sys.argv[2]
    sample = sys.argv[3]
    run = sys.argv[4]
    output = sys.argv[5]

    rows = parse_log(log_path, config, sample, run)

    output_path = Path(output)
    write_header = not output_path.exists()

    with open(output_path, 'a', newline='') as f:
        writer = csv.DictWriter(f,
            fieldnames=['config', 'sample', 'run', 'event', 'value', 'time'])
        if write_header:
            writer.writeheader()
        writer.writerows(rows)

    print(f"Parsed {len(rows)} rows → {output}")