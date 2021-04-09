#!/usr/bin/env python3
import os
import sys
from datetime import datetime


DIRECTORY = 'benches/profiles/'
DATETIME_FORMAT = "%Y%m%d-%H%M%S"

BENCHMARK_TEMPLATE = "cargo +nightly bench &> {output}"

PROFILE_TEMPLATE   = 'instruments -t "Time Profiler" -D {output} -l {time_limit} {target}'
PROFILE_OUTPUT     = DIRECTORY + 'profile'
PROFILE_TIME_LIMIT = 30000  # ms

SHOW_RESULTS = 'open -a Instruments {target}'


def find_benchmark_executable():
    search_directory = 'target/release/deps/'
    for file in os.listdir('target/release/deps/'):
        if file.startswith('bench-') and '.' not in file:
            return search_directory + file
    raise FileNotFoundError(f"Couldn't find benchmark in {search_directory}.")


def find_latest_before_benchmark():
    files = [file.split('_')[0] for file in os.listdir(DIRECTORY) if 'before' in file]
    return sorted(files, key=lambda date: datetime.strptime(date, DATETIME_FORMAT), reverse=True)[0]


def run(command):
    os.system(command)


def main():
    if 'before' in sys.argv:
        output = DIRECTORY + datetime.now().strftime(DATETIME_FORMAT) + '_before.txt'
    elif 'after' in sys.argv:
        output = DIRECTORY + find_latest_before_benchmark() + '_after.txt'
    else:
        raise SystemExit("Needs argument 'before' or 'after'")

    print('* Benchmarking: ', end='', flush=True)
    run(BENCHMARK_TEMPLATE.format(output=output))
    print(f'Done! Written to {output}.')

    print('* Profiling: ', end='', flush=True)
    run(PROFILE_TEMPLATE.format(output=PROFILE_OUTPUT, time_limit=PROFILE_TIME_LIMIT, target=find_benchmark_executable()))
    print(f'Done! Written to {PROFILE_OUTPUT}.trace.')

    print('* Showing results: ')
    run(SHOW_RESULTS.format(target=PROFILE_OUTPUT + '.trace'))


if __name__ == '__main__':
    main()