from _pylibsufr import *
from sys import stdout

def test(
    input_file: str = "test.fa", 
    output_file: str = "_test.sufr", 
    queries: list[str] = ["TT", "GATT", "ATTA", "ACTG"], 
    list_output_file: str = "test.list",
    expected_count_data: list[int] = [3,2,3,2],
    expected_extract_data: list[list] = [
        [(27, 27, '4', 25, (2, 33), 0), (2, 28, '1', 0, (2, 8), 0), (11, 29, '2', 8, (3, 16), 0)],
        [(25, 20, '4', 25, (0, 33), 0), (0, 21, '1', 0, (0, 8), 0)],
        [(26, 9, '4', 25, (1, 33), 0), (1, 10, '1', 0, (1, 8), 0), (10, 11, '2', 8, (2, 16), 0)],
        [(20, 6, '3', 16, (4, 25), 0), (16, 7, '3', 16, (0, 25), 0)],
    ],
    expected_locate_data: list[list] = [
        [(27, 27, '4', 2), (2, 28, '1', 2), (11, 29, '2', 3)], 
        [(25, 20, '4', 0), (0, 21, '1', 0)], 
        [(26, 9, '4', 1), (1, 10, '1', 1), (10, 11, '2', 2)], 
        [(20, 6, '3', 4), (16, 7, '3', 0)]
    ],
):
    # creating the suffix array
    sequence_file_data = py_read_sequence_file(
        input_file, 
        ord('%'))
    sufr_builder_args = PySufrBuilderArgs(
        sequence_file_data.seq(), 
        output_file, 
        False, 
        None, 
        True, 
        False, 
        False, 
        sequence_file_data.start_positions(), 
        sequence_file_data.sequence_names(), 
        16, 
        None, 
        42,
    )
    suffix_array = PySuffixArray(sufr_builder_args)

    # listing; only works if it's the first operation done to the suffix array?
    list_options = PyListOptions(
        [],
        True,
        True,
        True,
        None,
        None,
        list_output_file,
    )
    suffix_array.list(list_options)
    with open(list_output_file, 'r') as f:
        print(f"list_results: {f.read()}")

    # counting
    count_options = PyCountOptions(
        queries,
        None,
        False,
    )
    count_results = suffix_array.count(count_options)
    count_data = [res.count for res in count_results]
    assert count_data == expected_count_data

    # extracting
    extract_options = PyExtractOptions(
        queries,
        None,
        False,
        None,
        None,
    )
    extract_results = suffix_array.extract(extract_options)
    extract_data = [[(seq.suffix, seq.rank, seq.sequence_name, seq.sequence_start, seq.sequence_range, seq.suffix_offset) for seq in res.sequences] for res in extract_results]
    assert extract_data == expected_extract_data

    # locating
    locate_options = PyLocateOptions(
        queries,
        None,
        False,
    )
    locate_results = suffix_array.locate(locate_options)
    locate_data = [[(pos.suffix, pos.rank, pos.sequence_name, pos.sequence_position) for pos in res.positions] for res in locate_results]
    assert locate_data == expected_locate_data

    # metadata
    metadata = suffix_array.metadata()
    metadata_attrs = dir(metadata)[-13:]
    for attr in metadata_attrs:
        val = getattr(metadata, attr)
        print(f"{attr}: {val}")

    # listing; only works if it's the first operation done to the suffix array?
    list_options = PyListOptions(
        [],
        True,
        True,
        True,
        None,
        None,
        list_output_file,
    )
    suffix_array.list(list_options)
    with open(list_output_file, 'r') as f:
        print(f"list_results: (second time) {f.read()}")