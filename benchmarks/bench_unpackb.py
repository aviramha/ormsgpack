import os.path
import pytest
import msgpack
import ormsgpack

DATASETS = ("canada", "citm_catalog", "github", "twitter")
DATASETS_DATA = {
    dataset: open(
        os.path.join(os.path.dirname(__file__), "samples", f"{dataset}.mpack"), "rb"
    ).read()
    for dataset in DATASETS
}


@pytest.mark.parametrize("dataset", DATASETS)
def test_msgpack_unpackb(benchmark, dataset):
    benchmark.group = dataset
    benchmark(msgpack.unpackb, DATASETS_DATA[dataset])


@pytest.mark.parametrize("dataset", DATASETS)
def test_ormsgpack_unpackb(benchmark, dataset):
    benchmark.group = dataset
    benchmark(ormsgpack.unpackb, DATASETS_DATA[dataset])
