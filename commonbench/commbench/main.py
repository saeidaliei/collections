import logging
from pprint import pprint

from logger.logger import ColorFormatter
from modules.dask import BenchConfig, dask_benchmark


def main():
    log_fmt_str = "%(asctime)s%(name)s%(filename)s%(levelname)s%(message)s"
    logger = logging.getLogger()
    handler = logging.StreamHandler()
    handler.setFormatter(ColorFormatter(log_fmt_str))
    logger.addHandler(handler)
    logger.setLevel(logging.DEBUG)

    bench_config = BenchConfig(data_sizes=[1024, 1024 ** 2, 1024 ** 3])

    results = dask_benchmark(bench_config)
    pprint(results)


if __name__ == "__main__":
    main()
