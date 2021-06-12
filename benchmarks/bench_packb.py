import os.path
import pytest
import msgpack
import ormsgpack

DATASETS = ("canada", "citm_catalog", "github", "twitter")
DATASETS_DATA = {
    dataset: msgpack.unpackb(
        open(
            os.path.join(os.path.dirname(__file__), "samples", f"{dataset}.mpack"), "rb"
        ).read()
    )
    for dataset in DATASETS
}


@pytest.mark.parametrize("dataset", DATASETS)
def test_msgpack_packb(benchmark, dataset):
    benchmark.group = f"{dataset} packb"
    benchmark(msgpack.packb, DATASETS_DATA[dataset])


@pytest.mark.parametrize("dataset", DATASETS)
def test_ormsgpack_packb(benchmark, dataset):
    benchmark.group = f"{dataset} packb"
    benchmark(ormsgpack.packb, DATASETS_DATA[dataset])
