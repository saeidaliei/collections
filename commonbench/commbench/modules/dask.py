import socket
from time import time
import logging
import numpy as np
from typing import List, Any, Dict
from dask.distributed import Client, wait
from dataclasses import dataclass, field

logger = logging.getLogger("dask.dask")


@dataclass
class BenchConfig:
    dask_scheduler_address: str | None = None
    data_sizes: List[int] = field(default_factory=lambda: [1024, 1024 ** 2, 1024 ** 3])
    num_iters: int = 10


def dask_benchmark(bench_config: BenchConfig) -> Dict[str, Any]:
    try:
        if bench_config.dask_scheduler_address:
            logger.debug(f"trying to connecto to cluster scheduler: {bench_config.dask_scheduler_address}")
            client = Client(bench_config.dask_scheduler_address)
            logger.debug(f"connected to dask scheduler: {bench_config.dask_scheduler_address}")
            logger.debug(f"dask cluster dashboard: {client.dashboard_link}")
        else:
            logger.debug("creating local dask cluster")
            client = Client()
            logger.debug(f"created local dask cluster: {client.dashboard_link}")
    except Exception as e:
        logger.exception(f"ecxception occurred while accessing dask cluster: {e}")
        return {"error": "dask cluster could not be accessed"}

    workers = list(client.scheduler_info()["workers"].keys())
    logger.debug(f"got dask workers: {workers}")
    if len(workers) < 2:
        logger.error(f"need at least 2 dask workers, got: {len(workers)}")
        return {"error": "few dask workers"}

    worker1 = workers[0]
    worker2 = workers[1]
    logger.debug(f"selected workers: {worker1}, {worker2}")

    host1 = client.submit(lambda: socket.gethostname(), workers=worker1).result()
    host2 = client.submit(lambda: socket.gethostname(), workers=worker2).result()
    logger.debug(f"testing communication between {host1}:{worker1} and {host2}:{worker2}")

    results = []
    for size in bench_config.data_sizes:  # type: ignore
        logger.debug(f"testing with data size: {size} Bytes")
        iter_results = []
        transfer_times = []
        transfer_rates = []

        for iter in range(bench_config.num_iters):  # type: ignore
            data = client.submit(lambda n: np.random.random(n // 8), size, workers=worker1)

            start_time = time()
            # force the transfer of data between workers
            result = client.submit(lambda x: x.shape, data, workers=worker2)
            wait(result)
            end_time = time()
            transfer_time = end_time - start_time
            transfer_rate = size / transfer_time / (1024 * 1024)  # MB/s
            transfer_times.append(transfer_time)
            transfer_rates.append(transfer_rate)

            logger.info(f"iteration: {iter+1}, transfer time: {transfer_time: .3f} s, rate: {transfer_rate:.3f} MB/s")
            iter_results.append({
                "iteration": iter+1,
                "transfer_time_secs": transfer_time,
                "transfer_rate_MBps": transfer_rate
            })

        avg_time = np.mean(transfer_times)
        std_time = np.std(transfer_times)
        logger.info(f"average transfer time: {avg_time:.3f} ± {std_time:.3f} s")

        avg_rate = np.mean(transfer_rates)
        std_rate = np.std(transfer_rates)
        logger.info(f"average transfer rate: {avg_rate:.3f} ± {std_rate:.3f} MB/s")

        results.append({
            "size": size,
            "source_host": host1,
            "target_host": host2,
            "source_worker": worker1,
            "target_worker": worker2,
            "iteration_results": iter_results,
            "avg_time_secs": float(avg_time),
            "std_time_secs": float(std_time),
            "avg_rate_MBps": float(avg_rate),
            "std_rate_MBps": float(std_rate)
        })

    scheduler_info = client.scheduler_info()
    workers_info = scheduler_info["workers"]
    cluster_info = {
        "dashboard_link": client.dashboard_link,
        "worker_count": len(workers_info),
        "workers": [
            {
                "address": address,
                "host": info.get("host", ''),
                "resources": info.get("resources", {}),
                "mem_limit": info.get("memory_limit", 0)
            } for address, info in workers_info.items()
        ]
    }
    client.close()

    return_dict = {
        "benchmarks": results,
        "cluter_info": cluster_info
    }
    return return_dict
